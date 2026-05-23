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
