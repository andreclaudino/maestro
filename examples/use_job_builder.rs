use k8s_openapi::api::{batch::v1::{Job, JobSpec}, core::v1::{Container, PodSpec, PodTemplateSpec}};
use maestro::{clients::MaestroK8sClient, entities::{container::MaestroContainer, job::{JobBuilder, JobNameType, RestartPolicy}}};


#[tokio::main(flavor="current_thread")]
pub async fn main() -> anyhow::Result<()>{
    log::set_max_level(log::LevelFilter::Error);
    
    let job_name = "maestro";
    let namespace = "staging";
    let image = "docker.io/bash:5.2";
    let dry_run = false;
    
    let maestro_client = MaestroK8sClient::new().await?;
    
    let test_job_input = build_job(&image, &job_name, &namespace)?;
    let suceed_job = maestro_client.create_job(&test_job_input, namespace, dry_run).await?;
    suceed_job.delete_job(dry_run).await?;
    
    Ok(())
}

fn build_job(image: &str, name: &str, namespace: &str) -> anyhow::Result<Job> {
    let job_name = JobNameType::DefinedName(name.to_owned());
    let container_name = "main";
    
    let container =
        MaestroContainer::new(image, container_name)
            .set_arguments(&vec![
                "bash".to_owned(),
                "-c".to_owned(),
                "echo 'Testing pod'; sleep 3; echo 'Finalizado'".to_owned()
            ]);

    let job = 
        JobBuilder::new(&job_name, namespace)
            .set_backoff_limit(4)
            .set_restart_policy(&RestartPolicy::OnFailure)
            .add_container(container)
            .build()?;

    Ok(job)
}