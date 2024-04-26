use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::{Container, EnvFromSource, EnvVar, EnvVarSource, ResourceRequirements, Volume, VolumeMount}, apimachinery::pkg::api::resource::Quantity};

use crate::entities::{compute_resource::ComputeResource, container::EnvironmentVariableFromObject, container_like::ContainerLike, volumes::VolumeMountLike};

use super::MaestroContainer;

impl ContainerLike for MaestroContainer {
    fn into_container(&self) -> anyhow::Result<Container> {
        let resource_bounds = extract_resource_bounds(&self.resource_bounds);
        let environment_variables = extract_environment_variables(&self.environment_variables);
        let environments_from_objects = extract_environment_variables_from_objects(&self.environment_variables_from_objects);
        let volume_mounts = extract_volume_mounts(&self.volume_mounts)?;
        

        let container = Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            args: Some(self.arguments.clone()),
            resources: Some(resource_bounds),

            env: Some(environment_variables),
            env_from: Some(environments_from_objects),
            volume_mounts: Some(volume_mounts),

            ..Container::default()
        };

        Ok(container)
    }

    fn get_volumes(&self) -> anyhow::Result<Vec<Volume>> {
        let mut volumes = Vec::new();
        for volume_mount_like in self.volume_mounts.iter() {
            let volume = volume_mount_like.volume_like()?.into_volume()?;
            volumes.push(volume);
        };

        Ok(volumes)
    }

    fn add_volume_mount_like(mut self, volume_mount_like: Box<dyn VolumeMountLike>) -> anyhow::Result<Self> {
        if self.volume_mounts.len() == 0 {
            self.volume_mounts = vec![];
        }

        self.volume_mounts.push(volume_mount_like);

        Ok(self)
    }
}


fn extract_volume_mounts(volume_mount_likes: &Vec<Box<dyn VolumeMountLike>>) -> anyhow::Result<Vec<VolumeMount>> {
    let mut volume_mounts = Vec::new();
    for volume_mount_like in volume_mount_likes.iter() {
        let volume_mount = volume_mount_like.into_volume_mount()?;
        volume_mounts.push(volume_mount);
    }
    Ok(volume_mounts)
}

fn extract_environment_variables_from_objects(environment_variables_from_objects: &Vec<EnvironmentVariableFromObject>) -> Vec<EnvFromSource> {
    environment_variables_from_objects.into_iter().map(|environment_object| environment_object.into_env_source()).collect()
}

fn extract_environment_variables(environment_variables: &BTreeMap<String, crate::entities::environment_variable_source::EnvironmentVariableSource>) -> Vec<EnvVar> {
    environment_variables.iter().map(|(name, source)|{
        match source {
            crate::entities::environment_variable_source::EnvironmentVariableSource::Value(raw_value) => {
                EnvVar {
                    name: name.to_owned(),
                    value: Some(raw_value.to_owned()),
                    ..EnvVar::default()
                }
            },
            crate::entities::environment_variable_source::EnvironmentVariableSource::ConfigMap(source) => {
                let value_source = EnvVarSource {
                    config_map_key_ref: Some(source.to_owned()),
                    ..EnvVarSource::default()
                };
                EnvVar {
                    name: name.to_owned(),
                    value_from: Some(value_source.to_owned()),
                    ..EnvVar::default()
                }
            },
            crate::entities::environment_variable_source::EnvironmentVariableSource::PodFieldReference(source) => {
                let value_source = EnvVarSource {
                    field_ref: Some(source.to_owned()),
                    ..EnvVarSource::default()
                };
                EnvVar {
                    name: name.to_owned(),
                    value_from: Some(value_source.to_owned()),
                    ..EnvVar::default()
                }
            },
            crate::entities::environment_variable_source::EnvironmentVariableSource::ResourceFieldSelector(source) => {
                let value_source = EnvVarSource {
                    resource_field_ref: Some(source.to_owned()),
                    ..EnvVarSource::default()
                };
                EnvVar {
                    name: name.to_owned(),
                    value_from: Some(value_source.to_owned()),
                    ..EnvVar::default()
                }
            },
            crate::entities::environment_variable_source::EnvironmentVariableSource::SecretKeyReference(source) => {
                let value_source = EnvVarSource {
                    secret_key_ref: Some(source.to_owned()),
                    ..EnvVarSource::default()
                };
                EnvVar {
                    name: name.to_owned(),
                    value_from: Some(value_source.to_owned()),
                    ..EnvVar::default()
                }
            },
        }
    })
    .collect()
}


fn extract_resource_bounds(resources: &BTreeMap<ComputeResource, Quantity>) -> ResourceRequirements {
    let resource_map: BTreeMap<_, _> = resources.clone().into_iter().map(|(resource_key, resource_value)| {
        (resource_key.to_string(), resource_value)
    })
    .collect();

    let resource_bounds = ResourceRequirements {
        limits: Some(resource_map.clone()),
        requests: Some(resource_map.clone()),

        ..ResourceRequirements::default()
    };

    resource_bounds
}

