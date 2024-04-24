use k8s_openapi::{api::batch::v1::Job, serde::de::DeserializeOwned};

pub trait JobLike: for<'a> DeserializeOwned {
    fn into_job(&self) -> anyhow::Result<Job>;
}