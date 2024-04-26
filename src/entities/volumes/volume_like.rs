use k8s_openapi::api::core::v1::Volume;

pub trait VolumeLike {
    fn into_volume(&self) -> anyhow::Result<Volume>;
}