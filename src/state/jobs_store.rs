use std::process::Child;
use std::sync::{Mutex, OnceLock};

pub struct Job {
    pub id: usize,
    pub _pid: u32,
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
    let id = next_id(&guard);

    guard.push(Job {
        id,
        _pid: pid,
        command,
        child,
    });

    (id, pid)
}

pub fn push_pipeline(children: Vec<Child>, command: String) -> (usize, Vec<u32>) {
    let mut guard = get_store().lock().unwrap();

    let id = next_id(&guard);
    let mut pids = Vec::new();

    for child in children {
        pids.push(child.id());

        guard.push(Job {
            id,
            _pid: child.id(),
            command: command.clone(),
            child,
        });
    }

    (id, pids)
}

fn next_id(jobs: &[Job]) -> usize {
    jobs.iter().map(|j| j.id).max().unwrap_or(0) + 1
}

pub fn list_and_reap() -> Vec<JobStatus> {
    let mut guard = get_store().lock().unwrap();
    let mut result = Vec::new();

    guard.retain_mut(|j| {
        let (status, keep) = match j.child.try_wait() {
            Ok(Some(_)) => ("Done", false),
            Ok(None) => ("Running", true),
            Err(_) => ("Error", false),
        };
        result.push(JobStatus {
            id: j.id,
            command: j.command.clone(),
            status: status.to_string(),
        });
        keep
    });

    result
}
