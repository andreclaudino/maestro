use k8s_openapi::api::core::v1::{PersistentVolumeClaimVolumeSource, Volume};

use crate::entities::volumes::volume_like::VolumeLike;

use super::volume::MaestroPVCMountVolume;


impl VolumeLike for MaestroPVCMountVolume {
    fn into_volume(&self) -> anyhow::Result<Volume> {
        let persistent_volume_claim =
            PersistentVolumeClaimVolumeSource {
                claim_name: self.pvc_name.to_owned(),
                read_only: self.read_only,
            };

        let volume = 
            Volume {
                name: self.volume_name.to_owned(),
                persistent_volume_claim: Some(persistent_volume_claim),
                ..Volume::default()
            };
        
        Ok(volume)
    }
}