use k8s_openapi::{api::{batch::v1::Job, core::v1::Volume}, serde::de::DeserializeOwned};


pub trait VolumeLike: DeserializeOwned {
    fn into_volume(&self) -> anyhow::Result<Volume>;

    fn attach_to(&self, job: &mut Job) -> anyhow::Result<Job> {
        todo!()
    }
}
