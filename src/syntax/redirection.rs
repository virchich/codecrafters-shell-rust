#[derive(Debug)]
pub struct Redirection {
    pub mode: RedirectionMode,
    pub file_path: String,
}

#[derive(Debug)]
pub enum RedirectionMode {
    Overwrite,
    Append,
}
