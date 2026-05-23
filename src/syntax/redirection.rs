pub struct Redirection {
    pub mode: RedirectionMode,
    pub file_path: String,
}

pub enum RedirectionMode {
    Overwrite,
    Append,
}
