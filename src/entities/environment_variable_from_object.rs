use k8s_openapi::api::core::v1::{ConfigMapEnvSource, EnvFromSource, SecretEnvSource};

#[derive(Debug, Clone)]
pub enum EnvironmentVariableFromObject{
    Secret(String),
    ConfigMap(String),
}

impl EnvironmentVariableFromObject {
    pub fn into_env_source(&self) -> EnvFromSource {
        match self {
            Self::Secret(name) => {
                let secret_source = SecretEnvSource {
                    name: Some(name.into()),
                    ..SecretEnvSource::default()
                };

                EnvFromSource {
                    secret_ref: Some(secret_source),
                    ..EnvFromSource::default()
                }
            },
            Self::ConfigMap(name) => {
                let configmap_source = ConfigMapEnvSource {
                    name: Some(name.into()),
                    ..ConfigMapEnvSource::default()
                };
                
                EnvFromSource {
                    config_map_ref: Some(configmap_source),
                    ..EnvFromSource::default()
                }
            }
        }
    }
}