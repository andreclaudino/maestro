#[derive(Clone, Debug)]
pub enum RestartPolicy {
    OnFailure,
    Never
}

impl Into<Option<String>> for RestartPolicy {
    fn into(self) -> Option<String> {
        match self {
            RestartPolicy::Never => Some("Never".to_string()),
            RestartPolicy::OnFailure => Some("OnFailure".to_string())
        }
    }
}


impl Default for RestartPolicy {
    fn default() -> Self {
        RestartPolicy::OnFailure
    }
}