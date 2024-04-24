use anyhow::Ok;
use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::batch::v1::Job;
use kube::{runtime::{watcher, WatchStreamExt}, Api, api::DeleteParams};

use super::maestro_job_status::MaestroJobStatus;

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
    pub async fn delete_job(&self, dry_run: bool) -> anyhow::Result<()> {
        let api = Api::<Job>::namespaced(self.client.clone(), &self.namespace);
        let job_name = &self.name;
        let delete_params = DeleteParams{dry_run, ..DeleteParams::default()};
        
        api.delete(job_name, &delete_params).await?;
        
        Ok(())
    }
}

