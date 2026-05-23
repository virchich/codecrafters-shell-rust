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

    guard.push(CompletionRecord { path, command });
}

pub fn remove(command: String) {
    let mut guard = get_store().lock().unwrap();

    guard.retain(|record| record.command != command);
}

pub fn get_all() -> Vec<CompletionRecord> {
    let guard = get_store().lock().unwrap();

    guard.iter().cloned().collect()
}

#[cfg(test)]
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[cfg(test)]
pub(crate) fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[cfg(test)]
pub(crate) fn clear() {
    get_store().lock().unwrap().clear();
}

#[cfg(test)]
mod tests {
    use super::{clear, get_all, push, remove, test_lock};

    #[test]
    fn adds_and_removes_completion_records() {
        let _guard = test_lock();
        clear();

        push("/tmp/one".to_string(), "git".to_string());
        push("/tmp/two".to_string(), "cargo".to_string());
        remove("git".to_string());

        let records = get_all();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].command, "cargo");
        assert_eq!(records[0].path, "/tmp/two");
    }
}
