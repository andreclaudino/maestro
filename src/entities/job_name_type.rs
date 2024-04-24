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
