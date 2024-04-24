use k8s_openapi::api::batch::v1::Job;
use kube::{api::PostParams, Api};

use crate::entities::maestro_job::MaestroJob;

pub struct MaestroK8sClient {
    client: kube::Client
}

impl MaestroK8sClient {
    pub async fn new() -> anyhow::Result<MaestroK8sClient> {
        let client = kube::Client::try_default().await?;
        let k8s_client = MaestroK8sClient{ client };

        Ok(k8s_client)
    }

    pub async fn create_job(&self, job: &Job, namespace: &str, dry_run: bool) -> anyhow::Result<MaestroJob> {
        let api = Api::<Job>::namespaced(self.client.clone(), namespace);
        let post_parameters = PostParams{ dry_run, ..PostParams::default()};
        
        let created_job = api.create(&post_parameters, &job).await?;
        let maestro_job = MaestroJob::new(&created_job, self.client.clone());

        Ok(maestro_job)
    }
}