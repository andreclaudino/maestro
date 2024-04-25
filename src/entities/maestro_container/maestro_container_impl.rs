use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

use crate::entities::{compute_resource::ComputeResource, container::EnvironmentVariableFromObject, environment_variable_source::EnvironmentVariableSource};

use super::MaestroContainer;

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

    pub fn set_resource_bounds(mut self, resource_bounds: BTreeMap<ComputeResource, Quantity>) -> MaestroContainer {
        self.resource_bounds = resource_bounds;
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

    pub fn set_environment_variables_from_objects(mut self, source_objects: &Vec<EnvironmentVariableFromObject>) -> MaestroContainer {
        self.environment_variables_from_objects = source_objects.to_owned();
        self
    }
}