use serde::Serialize;


#[derive(Serialize)]
pub struct MaestroLogLine {
    line: String,
    pod_name: String,
    job_name: String
}


impl MaestroLogLine{
    pub fn new(line: &str, pod_name: &str, job_name: &str) -> MaestroLogLine {
        MaestroLogLine{
            line: line.to_owned(),
            pod_name: pod_name.to_owned(),
            job_name: job_name.to_owned()
        }
    }

    pub fn message(&self) -> String {
        self.line.clone()
    }

    pub fn source(&self) -> String {
        format!("{}/{}", &self.job_name, &self.pod_name)
    }

    pub fn rich_message(&self) -> String {
        let source = self.source();
        let message = self.message();

        format!("[{}] {}", &source, &message)
    }
}