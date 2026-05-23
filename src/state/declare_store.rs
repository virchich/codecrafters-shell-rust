use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

static DECLARE_STORE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn get_store() -> &'static Mutex<HashMap<String, String>> {
    DECLARE_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get(variable: &str) -> Option<String> {
    let guard = get_store().lock().unwrap();
    guard.get(variable).cloned()
}

pub fn add(variable: String, value: String) {
    let mut guard = get_store().lock().unwrap();
    guard.insert(variable, value);
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
    use super::{add, clear, get, test_lock};

    #[test]
    fn overwrites_existing_variable_values() {
        let _guard = test_lock();
        clear();

        add("NAME".to_string(), "first".to_string());
        add("NAME".to_string(), "second".to_string());

        assert_eq!(get("NAME"), Some("second".to_string()));
    }
}
