use std::collections::BTreeMap;

use k8s_openapi::{api::batch::v1::Job, apimachinery::pkg::api::resource::Quantity};
use maestro::{clients::MaestroK8sClient, entities::{container::{ComputeResource, ContainerLike, EnvironmentVariableFromObject, EnvironmentVariableSource, MaestroContainer}, job::{JobBuilder, JobNameType, RestartPolicy}}};


#[tokio::main(flavor="current_thread")]
pub async fn main() -> anyhow::Result<()>{
    log::set_max_level(log::LevelFilter::Error);
    
    let job_name = "maestro";
    let namespace = "staging";
    let image = "docker.io/bash:5.2";
    let dry_run = false;
    
    let maestro_client = MaestroK8sClient::new().await?;
    
    let test_job_input = build_job(&image, &job_name, &namespace)?;
    println!("{}", serde_yml::to_string(&test_job_input)?);
    let suceed_job = maestro_client.create_job(&test_job_input, namespace, dry_run).await?;
    suceed_job.wait().await?;
    suceed_job.delete_job(dry_run).await?;
    
    Ok(())
}

fn build_job(image: &str, name: &str, namespace: &str) -> anyhow::Result<Job> {
    let job_name = JobNameType::DefinedName(name.to_owned());
    let container_name = "main";

    let environment_from_object = vec![EnvironmentVariableFromObject::Secret("s3-storage".into())];
    let resource_bounds: BTreeMap<ComputeResource, Quantity> = vec![
        (ComputeResource::Cpu, Quantity("100m".to_owned())),
        (ComputeResource::Memory, Quantity("50M".to_owned()))
    ].into_iter()
    .collect();

    let environment_variables = vec![
        ("MAESTRO_TEST".to_owned(), EnvironmentVariableSource::Value("MAESTRO_TEST_VARIABLE".to_owned()))
    ].into_iter()
    .collect();
    
    let container =
        MaestroContainer::new(image, container_name)
            .set_arguments(&vec![
                "bash".to_owned(),
                "-c".to_owned(),
                "echo 'Testing pod'; sleep 3; echo 'Finalizado'".to_owned()
            ])
            .set_environment_variables_from_objects(&environment_from_object)
            .set_environment_variables(environment_variables)
            .set_resource_bounds(resource_bounds)
            .into_container()?;

    let job = 
        JobBuilder::new(&job_name, namespace)
            .set_backoff_limit(4)
            .set_restart_policy(&RestartPolicy::OnFailure)
            .add_container(container)
            .build()?;

    Ok(job)
}