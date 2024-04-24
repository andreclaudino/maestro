use super::{container_like::ContainerLike, maestro_container::MaestroContainer};

const MAESTRO_JOB_NAME: &str = "maestro-";

#[derive(Clone, Debug)]
pub enum JobNameType {
    DefinedName(String),
    GenerateName(String)
}

impl Default for JobNameType {
    fn default() -> Self {
        JobNameType::GenerateName(MAESTRO_JOB_NAME.to_owned())
    }
}

#[derive(Clone, Debug)]
pub enum RestartPolicy {
    OnFailure,
    Never
}


impl Default for RestartPolicy {
    fn default() -> Self {
        RestartPolicy::OnFailure
    }
}


pub struct JobBuilder<C=MaestroContainer> where C: ContainerLike {
    name: JobNameType,
    namespace: String,
    backoff_limit: usize,

    restart_policy: RestartPolicy,
    containers: Vec<C>,
}

impl<C> JobBuilder<C> where C: ContainerLike {
    pub fn new(name: &JobNameType, namespace: &str) -> JobBuilder<C> {
        JobBuilder {
            name: name.clone(),
            namespace: namespace.to_owned(),
            
            backoff_limit: 6,
            restart_policy: RestartPolicy::default(),
            containers: Vec::new(),
        }
    }

    pub fn set_defined_name(mut self, defined_name: &str) -> JobBuilder<C> {
        self.name = JobNameType::DefinedName(defined_name.to_owned());
        self
    }

    pub fn set_generate_name(mut self, defined_name: &str) -> JobBuilder<C> {
        self.name = JobNameType::GenerateName(defined_name.to_owned());
        self
    }

    pub fn set_backoff_limit(mut self, backoff_limit: usize) -> JobBuilder<C> {
        self.backoff_limit = backoff_limit;
        self
    }

    pub fn set_restart_policy(mut self, restart_policy: &RestartPolicy) -> JobBuilder<C> {
        self.restart_policy = restart_policy.to_owned();
        self
    }

    pub fn add_container(mut self, container_like: C) -> JobBuilder<C> {
        self.containers.push(container_like);
        self
    }
}