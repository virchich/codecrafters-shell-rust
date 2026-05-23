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

#[cfg(test)]
mod tests {
    use super::open_redirection;
    use crate::syntax::redirection::{Redirection, RedirectionMode};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_file(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rust-shell-{}-{}-{}",
            label,
            std::process::id(),
            unique
        ))
    }

    #[test]
    fn overwrites_existing_file_contents() {
        let path = temp_file("overwrite");
        fs::write(&path, "old").unwrap();

        let redirection = Redirection {
            mode: RedirectionMode::Overwrite,
            file_path: path.display().to_string(),
        };

        let mut file = open_redirection(&redirection);
        write!(file, "new").unwrap();
        drop(file);

        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn appends_to_existing_file_contents() {
        let path = temp_file("append");
        fs::write(&path, "old").unwrap();

        let redirection = Redirection {
            mode: RedirectionMode::Append,
            file_path: path.display().to_string(),
        };

        let mut file = open_redirection(&redirection);
        write!(file, "new").unwrap();
        drop(file);

        assert_eq!(fs::read_to_string(&path).unwrap(), "oldnew");
        fs::remove_file(path).unwrap();
    }
}
