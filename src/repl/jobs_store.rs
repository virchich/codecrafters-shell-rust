use std::process::Child;
use std::sync::{Mutex, OnceLock};

pub struct Job {
    pub id: usize,
    pub pid: u32,
    pub command: String,
    pub child: Child,
}

pub struct JobStatus {
    pub id: usize,
    pub command: String,
    pub status: String,
}

static JOBS: OnceLock<Mutex<Vec<Job>>> = OnceLock::new();

fn get_store() -> &'static Mutex<Vec<Job>> {
    JOBS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn push(child: Child, command: String) -> (usize, u32) {
    let mut guard = get_store().lock().unwrap();

    let pid = child.id();
    let id = guard.len() + 1;

    guard.push(Job {
        id,
        pid,
        command,
        child,
    });

    (id, pid)
}

pub fn push_pipeline(children: Vec<Child>, command: String) -> (usize, Vec<u32>) {
    let mut guard = get_store().lock().unwrap();

    let id = guard.len() + 1;
    let mut pids = Vec::new();

    for child in children {
        pids.push(child.id());

        guard.push(Job {
            id,
            pid: child.id(),
            command: command.clone(),
            child,
        });
    }

    (id, pids)
}

pub fn snapshot() -> Vec<JobStatus> {
    get_store()
        .lock()
        .unwrap()
        .iter_mut()
        .map(|j| JobStatus {
            id: j.id,
            command: j.command.clone(),
            status: match j.child.try_wait() {
                Ok(Some(_)) => "Done".to_string(),
                Ok(None) => "Running".to_string(),
                Err(_) => "Error".to_string(),
            },
        })
        .collect()
}
