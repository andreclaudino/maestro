use k8s_openapi::api::batch::v1::Job;

pub trait JobLike {
    fn into_job(&self) -> anyhow::Result<Job>;
}

