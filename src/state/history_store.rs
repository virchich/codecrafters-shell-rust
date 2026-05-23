use std::sync::{Mutex, OnceLock};

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
