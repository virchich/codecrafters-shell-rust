use std::env::split_paths;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn resolve_executable(command: &str, paths: &str) -> Option<String> {
    executable_commands(paths)
        .into_iter()
        .find(|path| path.ends_with(format!("/{}", command).as_str()))
}

pub fn executable_commands(paths: &str) -> Vec<String> {
    let mut executable_commands: Vec<String> = Vec::new();

    for path in split_paths(paths) {
        let dir = Path::new(path.as_path());

        if dir.exists() && dir.is_dir() {
            for dir_entry in std::fs::read_dir(dir).unwrap() {
                let entry = dir_entry.unwrap().path();

                if entry.is_file() {
                    let entry_permissions = entry.metadata().unwrap().permissions().mode();

                    if entry_permissions & 0o111 != 0 {
                        executable_commands.push(entry.display().to_string());
                    }
                }
            }
        }
    }

    executable_commands
}

#[cfg(test)]
mod tests {
    use super::{executable_commands, resolve_executable};
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rust-shell-{}-{}-{}",
            label,
            std::process::id(),
            unique
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn lists_only_executable_files() {
        let dir = temp_dir("resolver");
        let executable = dir.join("run-me");
        let regular = dir.join("read-me");

        fs::write(&executable, "echo hi").unwrap();
        fs::write(&regular, "plain").unwrap();

        fs::set_permissions(&executable, fs::Permissions::from_mode(0o755)).unwrap();
        fs::set_permissions(&regular, fs::Permissions::from_mode(0o644)).unwrap();

        let commands = executable_commands(dir.to_str().unwrap());

        assert_eq!(commands, vec![executable.display().to_string()]);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn resolves_command_from_path_list() {
        let dir_a = temp_dir("resolver-a");
        let dir_b = temp_dir("resolver-b");
        let target = dir_b.join("mycmd");

        fs::write(dir_a.join("other"), "echo nope").unwrap();
        fs::write(&target, "echo ok").unwrap();
        fs::set_permissions(&target, fs::Permissions::from_mode(0o755)).unwrap();

        let paths = std::env::join_paths([dir_a.as_path(), dir_b.as_path()])
            .unwrap()
            .into_string()
            .unwrap();

        assert_eq!(
            resolve_executable("mycmd", &paths),
            Some(target.display().to_string())
        );

        fs::remove_dir_all(dir_a).unwrap();
        fs::remove_dir_all(dir_b).unwrap();
    }
}
