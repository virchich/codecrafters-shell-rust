use std::ffi::OsStr;

pub enum SupportedEnv {
    PATH,
    HISTFILE,
}

impl AsRef<OsStr> for SupportedEnv {
    fn as_ref(&self) -> &OsStr {
        match self {
            SupportedEnv::PATH => OsStr::new("PATH"),
            SupportedEnv::HISTFILE => OsStr::new("HISTFILE"),
        }
    }
}
