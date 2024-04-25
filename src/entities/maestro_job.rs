use k8s_openapi::api::core::v1::Pod;
use anyhow::Ok;
use futures::{pin_mut, AsyncBufReadExt, Stream, TryStreamExt};
use k8s_openapi::api::batch::v1::Job;
use kube::{api::{DeleteParams, ListParams, LogParams}, runtime::{watcher, WatchStreamExt}, Api};
use async_stream::try_stream;

use super::{maestro_job_status::MaestroJobStatus, maestro_log::MaestroLogLine};

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

    fn create_watch_name_selector(&self) -> watcher::Config {
        let name_filter = format!("metadata.name={}", self.name);
        let watcher_config = watcher::Config{
            field_selector: Some(name_filter),
            ..watcher::Config::default()
        };
        watcher_config
    }

    /// TODO Implement a way to be sure that delection is competed
    /// TODO the delete job function do not delete the pod created by the job, this should be implemented
    pub async fn delete_job(&self, dry_run: bool) -> anyhow::Result<()> {
        let api = Api::<Job>::namespaced(self.client.clone(), &self.namespace);
        let job_name = &self.name;
        let delete_params = DeleteParams{dry_run, ..DeleteParams::default()};
        
        api.delete(job_name, &delete_params).await?;
        
        Ok(())
    }

    pub async fn get_log(&self) -> impl Stream<Item = anyhow::Result<MaestroLogLine>>{
        let client = self.client.clone();
        let namespace = self.namespace.clone();
        let job_name = self.name.clone();
        try_stream! {
            let api = Api::<Pod>::namespaced(client, &namespace);
            let pod_label = format!("job-name={}", job_name);
            let field_selector = Some("status.phase=Running".to_owned());
            let list_paramms = ListParams{field_selector, ..ListParams::default()}
            .labels(&pod_label);
            
            let pods_list = api.list(&list_paramms).await?;
            if let Some(selected_pod) = pods_list.items.first() {
                let pod_name = selected_pod.metadata.name.as_ref().unwrap();
                let pod_params = LogParams::default();
                let mut logs = api.log_stream(pod_name, &pod_params).await?.lines();
                while let Some(line) = logs.try_next().await? {
                  let maestro_log_line =  MaestroLogLine::new(&line, pod_name, &job_name);
                    yield maestro_log_line
                }
            }
        }
    }
}

