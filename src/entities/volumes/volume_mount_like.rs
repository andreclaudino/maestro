use std::fmt::Debug;

use k8s_openapi::api::core::v1::VolumeMount;

use super::VolumeLike;

pub trait VolumeMountLike: Debug {
    fn into_volume_mount(&self) -> anyhow::Result<VolumeMount>;
    fn mount_path(&self) -> anyhow::Result<String>;
    fn volume_name(&self) -> anyhow::Result<String>;
    fn volume_like(&self) -> anyhow::Result<Box<dyn VolumeLike>>;
}