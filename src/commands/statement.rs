use crate::commands::command::Command;

pub struct RedirectStatement {
    pub command: Command,
    pub redirect_std_out: Option<Redirect>,
    pub redirect_std_err: Option<Redirect>,
}

pub struct Redirect {
    pub redirect_mode: RedirectMode,
    pub file_location: String,
}

pub enum RedirectMode {
    Overwrite,
    Append,
}
