use std::process::Child;
use std::sync::{Mutex, OnceLock};

pub struct Job {
    pub id: usize,
    pub pid: u32,
    pub command: String,
    pub child: Child,
}

static JOBS: OnceLock<Mutex<Vec<Job>>> = OnceLock::new();

fn get_store() -> &'static Mutex<Vec<Job>> {
    JOBS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn push(child: Child, command: String) -> (usize, u32) {
    let pid = child.id();
    let mut guard = get_store().lock().unwrap();
    let id = guard.len() + 1;
    guard.push(Job {
        id,
        pid,
        command,
        child,
    });
    (id, pid)
}

pub fn snapshot() -> Vec<(usize, u32, String)> {
    get_store()
        .lock()
        .unwrap()
        .iter()
        .map(|j| (j.id, j.pid, j.command.clone()))
        .collect()
}