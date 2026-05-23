use crate::syntax::redirection::{Redirection, RedirectionMode};
use std::fs::File;

pub fn open_redirection(redirection: &Redirection) -> File {
    match redirection.mode {
        RedirectionMode::Append => File::options()
            .create(true)
            .append(true)
            .open(&redirection.file_path)
            .unwrap(),
        RedirectionMode::Overwrite => File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&redirection.file_path)
            .unwrap(),
    }
}
