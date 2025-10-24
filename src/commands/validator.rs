use std::env::split_paths;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn is_command_allowed(command: &String) -> bool {
    let allowed_commands = ["exit".to_string(), "echo".to_string(), "type".to_string()];

    allowed_commands.contains(command)
}

pub fn is_command_executable(command: &String, paths: String) -> (bool, String) {
    for path in split_paths(&paths) {
        let dir = Path::new(path.as_path());

        if dir.exists() && dir.is_dir() {
            for dir_entry in std::fs::read_dir(dir).unwrap() {
                let entry = dir_entry.unwrap().path();

                if entry.is_file() {
                    let entry_permissions = entry.metadata().unwrap().permissions().mode();

                    if entry.ends_with(command) && (entry_permissions & 0o111) != 0 {
                        return (true, entry.display().to_string());
                    }
                }
            }
        }
    }

    (false, String::from(""))
}
