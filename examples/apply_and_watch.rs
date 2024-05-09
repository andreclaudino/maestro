use k8s_openapi::api::{batch::v1::{Job, JobSpec}, core::v1::{Container, PodSpec, PodTemplateSpec}};
use k8s_maestro::clients::MaestroK8sClient;


#[tokio::main(flavor="current_thread")]
pub async fn main() -> anyhow::Result<()>{
    log::set_max_level(log::LevelFilter::Error);

    let suceed_name = "suceed-job";
    let failing_name = "failing-job";
    let namespace = "staging";
    let dry_run = false;
    
    let maestro_client = MaestroK8sClient::new().await?;
    
    let test_job_input = make_sleep_job(suceed_name, &namespace);
    let failed_job_input = make_failing_job(failing_name, &namespace);

    let suceed_job = maestro_client.create_job(&test_job_input, namespace, dry_run).await?;
    let failed_job = maestro_client.create_job(&failed_job_input, namespace, dry_run).await?;
    
    let _ = futures::join!(failed_job.wait(), suceed_job.wait());
    
    
    Ok(())
}

fn make_sleep_job(name: &str, namespace: &str) -> Job {
    let mut container = Container::default();
    container.name = "main".to_owned();
    container.image = Some("docker.io/bash:5.2".to_owned());
    container.args = Some(vec![
        "bash".to_owned(),
        "-c".to_owned(),
        "echo 'Testing pod'; sleep 3; echo 'Finalizado'".to_owned()
    ]);

    let mut pod_spec = PodSpec::default();
    pod_spec.containers.push(container);
    pod_spec.restart_policy = Some("OnFailure".to_string());

    let mut pod_template_spec = PodTemplateSpec::default();
    pod_template_spec.spec = Some(pod_spec);
    
    let mut job_spec = JobSpec::default();
    job_spec.template = pod_template_spec;
    job_spec.backoff_limit = Some(2);

    let mut job = Job::default();
    job.metadata.name = Some(name.to_owned());
    job.metadata.namespace = Some(namespace.to_owned());
    job.spec = Some(job_spec);

    job
}


fn make_failing_job(name: &str, namespace: &str) -> Job {
    let mut container = Container::default();
    container.name = "main".to_owned();
    container.image = Some("docker.io/bash:5.2".to_owned());
    container.args = Some(vec![
        "bash".to_owned(),
        "-c".to_owned(),
        "echo 'Testing pod'; sleep 3; exit 137".to_owned()
    ]);

    let mut pod_spec = PodSpec::default();
    pod_spec.containers.push(container);
    pod_spec.restart_policy = Some("OnFailure".to_string());

    let mut pod_template_spec = PodTemplateSpec::default();
    pod_template_spec.spec = Some(pod_spec);
    
    let mut job_spec = JobSpec::default();
    job_spec.template = pod_template_spec;
    job_spec.backoff_limit = Some(2);

    let mut job = Job::default();
    job.metadata.name = Some(name.to_owned());
    job.metadata.namespace = Some(namespace.to_owned());
    job.spec = Some(job_spec);

    job
}