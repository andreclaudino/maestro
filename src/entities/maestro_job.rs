use k8s_openapi::api::core::v1::Pod;
use anyhow::Ok;
use futures::{pin_mut, AsyncBufReadExt, Stream, StreamExt, TryStreamExt};
use k8s_openapi::api::batch::v1::Job;
use kube::{api::{DeleteParams, ListParams, LogParams}, runtime::{watcher::{watcher, Config as WatcherConfig}, WatchStreamExt}, Api, Client};
use async_stream::try_stream;

use super::{maestro_job_status::MaestroJobStatus, maestro_log::MaestroLogLine};

const POD_PHASE_RUNNING: &str = "RUNNING";

pub struct MaestroJob {
    client: kube::Client,
    namespace: String,
    name: String,
}

impl MaestroJob {
    pub fn new(job: &Job, client: kube::Client) -> MaestroJob {
        let job_metadata = job.metadata.clone();
        let name = job_metadata.name.unwrap();
        let namespace = job_metadata.namespace.unwrap();

        MaestroJob {
            client: client.clone(),
            namespace,
            name,
        }
    }

    pub async fn wait(&self) -> anyhow::Result<MaestroJobStatus> {
        let api = Api::<Job>::namespaced(self.client.clone(), &self.namespace);
        
        let watcher_config = self.create_watch_name_selector();

        let stream = watcher(api, watcher_config).applied_objects();

        pin_mut!(stream);

        while let Some(job) = stream.try_next().await? {
            if let Some(status) = job.status {
                if let Some(conditions) = status.conditions {
                    if let Some(last_condition) = conditions.last() {
                        let message = last_condition.message.clone().unwrap_or_default();
                        let condition_type: MaestroJobStatus = last_condition.type_.clone().into();

                        match condition_type {
                            MaestroJobStatus::Complete => {
                                log::info!("Job {} completed", self.name);
                                return Ok(MaestroJobStatus::Complete);
                            },
                            MaestroJobStatus::Failed => {
                                log::warn!("Job {} failed with message `{}`", self.name, message);
                                return Ok(MaestroJobStatus::Failed);
                                
                            },
                            MaestroJobStatus::Suspended => {
                                log::info!("Job {} is suspended", self.name);
                                return Ok(MaestroJobStatus::Suspended);
                            },
                            MaestroJobStatus::Unknow(received_value) => {
                                log::info!("Job {} is with unknow status: `{}`", self.name, received_value);
                                return Ok(MaestroJobStatus::Unknow(received_value));
                            },
                        }
                    }
                }
            }
        }

        Ok(MaestroJobStatus::Failed)
    }

    fn create_watch_name_selector(&self) -> WatcherConfig {
        let name_filter = format!("metadata.name={}", self.name);
        let watcher_config = WatcherConfig{
            field_selector: Some(name_filter),
            ..WatcherConfig::default()
        };
        watcher_config
    }

    /// TODO Implement a way to be sure that delection is competed
    /// TODO the delete job function do not delete the pod created by the job, this should be implemented
    pub async fn delete_job(&self, dry_run: bool) -> anyhow::Result<()> {
        let jobs_api = Api::<Job>::namespaced(self.client.clone(), &self.namespace);
        let job_name = &self.name;
        let delete_params = DeleteParams{dry_run, ..DeleteParams::default()};
        
        jobs_api.delete(job_name, &delete_params).await?;
        
        Ok(())
    }

    /// TODO: Em lugar de apenas verificar a existência do pod a ser deletado, confirmar se a mensagem de erro realmente se refere
    /// a não existência do pod, pois pode ter havido qualquer outro erro de comunicação e pod permanecerá existindo no kuberentes.
    pub async fn delete_associated_pods(&self) -> Result<(), anyhow::Error> {
        let pods_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);
        let pod_delete_parameters = DeleteParams::default();
        
        let associated_pods = list_pods(self.client.clone(), self.namespace.clone(), self.name.clone(), None, None).await?;

        for pod in associated_pods {
            let pod_name = pod.metadata.name.clone().unwrap();
            pods_api.delete(&pod_name, &pod_delete_parameters).await?;
        }

        Ok(())
    }

    pub async fn stream_pods(&self, field_selector: Option<String>, timeout: Option<u32>) -> impl Stream<Item=anyhow::Result<Pod>> {
        let client = self.client.clone();
        let namespace = self.namespace.clone();
        let job_name = self.name.clone();

        let stream = stream_pods(client, namespace, job_name, field_selector, timeout).await;

        stream
    }

    /// TODO: Tratar casos de erro, quando o pod não está disponível por exemplo,
    /// pensar uma solução para o caso de multiplos pods ou verificar como aguardar
    /// até que o pod esteja em estado running
    pub async fn stream_logs(&self, timeout: Option<u32>) -> impl Stream<Item = anyhow::Result<MaestroLogLine>>{
        let client = self.client.clone();
        let namespace = self.namespace.clone();
        let job_name = self.name.clone();
        
        let output_stream = 
            try_stream! {
                let running_pod_stream = stream_pods(client.clone(), namespace.clone(), job_name.clone(), None, timeout).await;
                let log_stream_parameters = LogParams::default();

                pin_mut!(running_pod_stream);

                let pod_api = Api::<Pod>::namespaced(client, &namespace);

                while let Some(pod) = running_pod_stream.next().await.transpose()? {
                    if pod.status.unwrap().phase == Some(POD_PHASE_RUNNING.to_string()) {
                        let pod_name = pod.metadata.name.as_ref().unwrap();
                        let mut logs = pod_api.log_stream(pod_name, &log_stream_parameters).await?.lines();

                        while let Some(line) = logs.try_next().await? {
                            let maestro_log_line =  MaestroLogLine::new(&line, pod_name, &job_name);
                            log::trace!("Retrieved log line for job `{}` from pod `{}`: {}", job_name, pod_name, line);
                            yield maestro_log_line
                        }
                        log::debug!("Log stream finished for job `{}` from pod `{}`", job_name, pod_name);
                    }
                }
            };

        output_stream
    }

}


pub async fn stream_pods(client: Client, namespace: String, job_name: String, field_selector: Option<String>, timeout: Option<u32>) -> impl Stream<Item=anyhow::Result<Pod>> {
    let label_selector = Some(format!("job-name={}", job_name));

    let watcher_config = WatcherConfig{
        field_selector,
        label_selector,
        timeout,
        ..WatcherConfig::default()
    };

    let api = Api::<Pod>::namespaced(client, &namespace);

    try_stream! {
        let mut apply_stream = watcher(api, watcher_config).applied_objects().boxed();
        while let Some(pod) = apply_stream.try_next().await? {
            yield pod
        }
    }
}

pub async fn list_pods(client: Client, namespace: String, job_name: String, field_selector: Option<String>, timeout: Option<u32>) -> anyhow::Result<Vec<Pod>> {
    let label_selector = Some(format!("job-name={}", job_name));

    let list_parameters = ListParams{
        field_selector,
        label_selector,
        timeout,
        ..ListParams::default()
    };

    let pods_api = Api::<Pod>::namespaced(client, &namespace);
    let associated_pods =
        pods_api.list(&list_parameters)
            .await?
            .iter()
            .map(|pod| pod.to_owned())
            .collect();

    Ok(associated_pods)
}