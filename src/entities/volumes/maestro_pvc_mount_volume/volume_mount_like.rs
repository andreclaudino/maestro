use k8s_openapi::api::core::v1::VolumeMount;

use crate::entities::volumes::{volume_mount_like::VolumeMountLike, VolumeLike};

use super::volume::MaestroPVCMountVolume;

impl VolumeMountLike for MaestroPVCMountVolume {
    fn into_volume_mount(&self) -> anyhow::Result<VolumeMount> {
        let volume_mount = 
            VolumeMount {
                mount_path: self.mount_path.to_owned(),
                name: self.volume_name.to_owned(),
                read_only: self.read_only,
                sub_path_expr: self.sub_path_expression.to_owned(),
                ..VolumeMount::default()
            };
        
        Ok(volume_mount)
    }

    fn mount_path(&self) -> anyhow::Result<String> {
        Ok(self.mount_path.to_owned())
    }

    fn volume_name(&self) -> anyhow::Result<String> {
        Ok(self.volume_name.to_owned())
    }
    
    fn volume_like(&self) -> anyhow::Result<Box<dyn VolumeLike>> {
        Ok(Box::new(self.clone()))
    }
}