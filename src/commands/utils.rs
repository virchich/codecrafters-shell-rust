use crate::syntax::statement::{Redirect, RedirectMode};
use std::fs::File;

pub fn open_redirect(redirect: &Redirect) -> File {
    match redirect.redirect_mode {
        RedirectMode::Append => File::options()
            .create(true)
            .append(true)
            .open(&redirect.file_location)
            .unwrap(),
        RedirectMode::Overwrite => File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&redirect.file_location)
            .unwrap(),
    }
}
