use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::database;

#[allow(dead_code)]
const DEBUG: bool = true;

lazy_static! {
    static ref DEBUG_LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref DATABASE: Arc<Mutex<database::Database>> = Arc::new(Mutex::new(database::Database::new("database.json".to_string())));
}

pub fn add_to_debug_log(value: &str) {
    let val: String = value.to_string();
    let mut vec = DEBUG_LOG.lock().unwrap();
    vec.push(val);
}

pub fn get_debug_log() -> Vec<String> {
    DEBUG_LOG.lock().unwrap().clone()
}

pub fn get_database() -> Arc<Mutex<database::Database>> {
    Arc::clone(&DATABASE)
}