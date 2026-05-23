use std::sync::{Mutex, MutexGuard, OnceLock};

static HISTORY: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn get_store() -> &'static Mutex<Vec<String>> {
    HISTORY.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn push(entry: String) {
    get_store().lock().unwrap().push(entry);
}

pub fn get_all() -> Vec<String> {
    get_store().lock().unwrap().clone()
}

#[cfg(test)]
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[cfg(test)]
pub(crate) fn test_lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[cfg(test)]
pub(crate) fn clear() {
    get_store().lock().unwrap().clear();
}

#[cfg(test)]
mod tests {
    use super::{clear, get_all, push, test_lock};

    #[test]
    fn stores_entries_in_order() {
        let _guard = test_lock();
        clear();

        push("echo one".to_string());
        push("echo two".to_string());

        assert_eq!(get_all(), vec!["echo one", "echo two"]);
    }
}
