use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::api::resource::Quantity;

use crate::entities::container::EnvironmentVariableFromObject;

use super::super::{compute_resource::ComputeResource, environment_variable_source::EnvironmentVariableSource};


#[derive(Debug, Default)]
pub struct MaestroContainer {
    pub(super) name: String,
    pub(super) image: String,
    pub(super) arguments: Vec<String>,
    pub(super) resource_bounds: BTreeMap<ComputeResource, Quantity>,
    pub(super) environment_variables: BTreeMap<String, EnvironmentVariableSource>,
    pub(super) environment_variables_from_objects: Vec<EnvironmentVariableFromObject>,

}




