use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

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

pub fn remove(variable: String) {
    let mut guard = get_store().lock().unwrap();

    guard.remove(&variable);
}
