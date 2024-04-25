use k8s_openapi::api::{batch::v1::{Job, JobSpec}, core::v1::{Container, PodSpec, PodTemplateSpec}};
use kube::api::ObjectMeta;

use super::{job_name_type::JobNameType, restart_policy::RestartPolicy};


pub struct JobBuilder {
    pub name: JobNameType,
    pub namespace: String,
    pub backoff_limit: usize,

    pub restart_policy: RestartPolicy,
    pub containers: Vec<Container>,
}

impl JobBuilder{
    pub fn new(name: &JobNameType, namespace: &str) -> JobBuilder {
        JobBuilder {
            name: name.clone(),
            namespace: namespace.to_owned(),
            
            backoff_limit: 6,
            restart_policy: RestartPolicy::default(),
            containers: Vec::new(),
        }
    }

    pub fn set_defined_name(mut self, defined_name: &str) -> JobBuilder {
        self.name = JobNameType::DefinedName(defined_name.to_owned());
        self
    }

    pub fn set_generate_name(mut self, defined_name: &str) -> JobBuilder {
        self.name = JobNameType::GenerateName(defined_name.to_owned());
        self
    }

    pub fn set_backoff_limit(mut self, backoff_limit: usize) -> JobBuilder {
        self.backoff_limit = backoff_limit;
        self
    }

    pub fn set_restart_policy(mut self, restart_policy: &RestartPolicy) -> JobBuilder {
        self.restart_policy = restart_policy.to_owned();
        self
    }

    pub fn add_container(mut self, container_like: Container) -> JobBuilder {
        self.containers.push(container_like);
        self
    }

    pub fn build(self) -> anyhow::Result<Job> {
        
        let pod_spec = PodSpec {
            restart_policy: self.restart_policy.into(),
            containers: extract_container_list(&self.containers),
            ..PodSpec::default()
        };

        let pod_template_spec = PodTemplateSpec{
            spec: Some(pod_spec),

            ..PodTemplateSpec::default()
        };
                
        let job_spec = JobSpec{
            template: pod_template_spec,
            backoff_limit: Some(self.backoff_limit as i32),
            ..JobSpec::default()
        };
        
        let job_meta = match self.name {
            JobNameType::DefinedName(define_name) => ObjectMeta{
                name: Some(define_name.to_string()),
                namespace: Some(self.namespace.to_owned()),
                ..ObjectMeta::default()
            },
            JobNameType::GenerateName(generate_name) => ObjectMeta{
                generate_name: Some(generate_name.to_string()),
                namespace: Some(self.namespace.to_owned()),
                ..ObjectMeta::default()
            },
        };

        let job = Job{ 
            spec: Some(job_spec),
            metadata: job_meta,
            ..Job::default()
        };
        
        Ok(job)
    }

}

fn extract_container_list(containers: &Vec<Container>) -> Vec<Container>{
    containers.iter().map(|container|{
        anyhow::Ok(container.to_owned())
    }).filter_map(|container_result| {
        if let Ok(container) = container_result {
            Some(container)
        } else {
            None
        }
    })
    .collect()
}