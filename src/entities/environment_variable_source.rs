use k8s_openapi::api::core::v1::{ConfigMapKeySelector, EnvVar, EnvVarSource, ObjectFieldSelector, ResourceFieldSelector, SecretKeySelector};


#[derive(Debug)]
pub enum EnvironmentVariableSource {
    Value(String),
    ConfigMap(ConfigMapKeySelector),
    PodFieldReference(ObjectFieldSelector),
    ResourceFieldSelector(ResourceFieldSelector),
    SecretKeyReference(SecretKeySelector)
}

impl EnvironmentVariableSource {
    pub fn into_k8s_env_var(&self, name: &str) -> EnvVar {
        match &self {
            EnvironmentVariableSource::Value(value) => EnvVar{
                name: name.to_owned(),
                value: Some(value.to_owned()),
                ..EnvVar::default()
            },
            EnvironmentVariableSource::ConfigMap(configmap) => {
                let source = EnvVarSource {
                    config_map_key_ref: Some(configmap.to_owned()),
                    ..EnvVarSource::default()
                };

                EnvVar{
                    name: name.to_owned(),
                    value_from: Some(source),
                    ..EnvVar::default()
                }  
            },
            EnvironmentVariableSource::PodFieldReference(field_reference) => {
                let source = EnvVarSource {
                    field_ref: Some(field_reference.to_owned()),
                    ..EnvVarSource::default()
                };
                EnvVar{
                    name: name.to_owned(),
                    value_from: Some(source),
                    ..EnvVar::default()
                }  
            },
            EnvironmentVariableSource::ResourceFieldSelector(resource_field_reference) => {
                let source = EnvVarSource {
                    resource_field_ref: Some(resource_field_reference.to_owned()),
                    ..EnvVarSource::default()
                };
                
                EnvVar{
                    name: name.to_owned(),
                    value_from: Some(source),
                    ..EnvVar::default()
                }  
            },
            EnvironmentVariableSource::SecretKeyReference(secret_key_reference) => {
                let source = EnvVarSource {
                    secret_key_ref: Some(secret_key_reference.to_owned()),
                    ..EnvVarSource::default()
                };
                
                EnvVar{
                    name: name.to_owned(),
                    value_from: Some(source),
                    ..EnvVar::default()
                }
            }
        }
    }
}

impl Default for EnvironmentVariableSource {
    fn default() -> Self {
        EnvironmentVariableSource::Value("".to_owned())
    }
}