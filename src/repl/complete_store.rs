use std::sync::{Mutex, OnceLock};

#[derive(Clone)]
pub struct CompletionRecord {
    pub path: String,
    pub command: String,
}

static COMPLETION_STORE: OnceLock<Mutex<Vec<CompletionRecord>>> = OnceLock::new();

fn get_store() -> &'static Mutex<Vec<CompletionRecord>> {
    COMPLETION_STORE.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn push(path: String, command: String) {
    let mut guard = get_store().lock().unwrap();

    guard.push(CompletionRecord {
        path,
        command,
    });
}

pub fn get_all() -> Vec<CompletionRecord> {
    let guard = get_store().lock().unwrap();

    guard.iter().cloned().collect()
}
