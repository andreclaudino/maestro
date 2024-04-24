use k8s_openapi::api::batch::v1::Job;
use kube::{api::PostParams, Api};

use crate::entities::configuration::JobLike;


pub struct K8sClient {
    client: kube::Client
}

impl K8sClient {
    pub async fn new() -> anyhow::Result<K8sClient> {
        let client = kube::Client::try_default().await?;
        let k8s_client = K8sClient { client };

        Ok(k8s_client)
    }

    pub async fn create_job<J>(&self, job_like: &J, namespace: &str) -> anyhow::Result<Job> where J: JobLike {
        let api = Api::<Job>::namespaced(self.client.clone(), namespace);
        let job = job_like.into_job()?;
        let post_parameters = PostParams::default();
        
        let created_object = api.create(&post_parameters, &job).await?;

        Ok(created_object)
    }

    pub fn wait_job() -> anyhow::Result<> {
        
    }
}