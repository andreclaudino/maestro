
pub enum MaestroJobStatus {
    Complete,
    Failed,
    Suspended,
    Unknow(String),
}

impl From<String> for MaestroJobStatus {
    fn from(value: String) -> Self {
        if value.to_lowercase().eq("complete") {
            MaestroJobStatus::Complete
        } else if value.to_lowercase().eq("failed") {
            MaestroJobStatus::Failed
        } else if value.to_lowercase().eq("suspended") {
            MaestroJobStatus::Suspended
        } else {
            MaestroJobStatus::Unknow(value)
        }
    }
}