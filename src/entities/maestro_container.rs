use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::{Container, ResourceRequirements}, apimachinery::pkg::api::resource::Quantity};

use super::{compute_resource::ComputeResource, container_like::ContainerLike, environment_variable_source::EnvironmentVariableSource};


#[derive(Debug, Default)]
pub struct MaestroContainer {
    name: String,
    image: String,
    arguments: Vec<String>,
    resources: BTreeMap<ComputeResource, Quantity>,
    environment_variables: BTreeMap<String, EnvironmentVariableSource>

}

impl MaestroContainer {
    pub fn new(image: &str, name: &str) -> MaestroContainer {
        MaestroContainer {
            name: name.to_owned(),
            image: image.to_owned(),
            ..MaestroContainer::default()
        }
    }

    pub fn set_image(mut self, image: &str) -> MaestroContainer {
        self.image = image.to_owned();
        self
    }

    pub fn set_resources(mut self, resources: BTreeMap<ComputeResource, Quantity>) -> MaestroContainer {
        self.resources = resources;
        self
    }

    pub fn set_arguments(mut self, arguments: &Vec<String>) -> MaestroContainer {
        self.arguments = arguments.to_owned();
        self
    }
    
    pub fn set_environment_variables(mut self, environment_variables: BTreeMap<String, EnvironmentVariableSource>) -> MaestroContainer {
        self.environment_variables = environment_variables;
        self
    }
}


impl ContainerLike for MaestroContainer {
    fn into_container(&self) -> anyhow::Result<Container> {
        let resource_map: BTreeMap<_, _> = self.resources.clone().into_iter().map(|(resource_key, resource_value)| {
            (resource_key.to_string(), resource_value)
        })
        .collect();

        let resources = ResourceRequirements {
            limits: Some(resource_map.clone()),
            requests: Some(resource_map.clone()),

            ..ResourceRequirements::default()
        };

        let container = Container {
            name: self.name.clone(),
            image: Some(self.image.clone()),
            args: Some(self.arguments.clone()),
            resources: Some(resources),

            ..Container::default()
        };

        Ok(container)
    }
}