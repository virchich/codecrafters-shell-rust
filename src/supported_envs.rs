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

#[cfg(test)]
mod tests {
    use super::SupportedEnv;
    use std::ffi::OsStr;

    #[test]
    fn maps_variants_to_expected_environment_keys() {
        assert_eq!(SupportedEnv::PATH.as_ref(), OsStr::new("PATH"));
        assert_eq!(SupportedEnv::HISTFILE.as_ref(), OsStr::new("HISTFILE"));
    }
}
