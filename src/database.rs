use crate::util;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use chrono::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::globals;

/// Logs messages to the application's debug log.
fn log(message: &str) {
    globals::add_to_debug_log(message);
}

/// Generates a unique session ID using the current timestamp and a random value.
fn generate_session_id() -> i64 {
    let now = Utc::now();
    let timestamp = now.timestamp(); // timestamp is i64 by default

    let mut rng = rand::thread_rng();
    let random_id: i64 = rng.gen(); // Use i64 to prevent overflow

    timestamp + random_id // Now this is safe since we're working with i64
}

/// Represents the main database structure.
pub struct Database {
    path: String,
    id: i64,
    session_storage: HashMap<String, String>,
    session_storage_nested: HashMap<String, HashMap<String, String>>,
}

/// Checks if the database file exists at the given path.
/// Logs an appropriate message depending on the outcome.
pub fn check_for_database(path: &str) -> bool {
    let exists = Path::new(path).exists();

    if exists {
        globals::add_to_debug_log("'./database.rs': Database found.");
    } else {
        globals::add_to_debug_log("ERROR in './database.rs': Database was not found. A new one will be created.");
    }

    exists
}

/// Represents an itinerary entry in the database.
#[derive(Serialize, Deserialize, Debug)]
struct Itinerary {
    session_id: i64,
}

impl Database {
    /// Creates a new database instance. If no database exists, it initializes a new one.
    pub fn new(path: String) -> Database {
        if !check_for_database(&path) {
            match Database::initiate_db(&path) {
                Ok(_) => globals::add_to_debug_log(&format!("Database created successfully at {}", path)),
                Err(e) => {
                    globals::add_to_debug_log(&format!("Failed to create database at {}", path));
                    globals::add_to_debug_log(&e.to_string());
                }
            }
        }

        Database {
            path,
            id: generate_session_id(),
            session_storage: HashMap::new(),
            session_storage_nested: HashMap::new(),
        }
    }

    /// Retrieves the current session storage as a cloned HashMap.
    pub fn get_session_storage(&self) -> HashMap<String, String> {
        self.session_storage.clone()
    }

	pub fn list_all_itineraries(&self) -> Vec<String> {
	    let db_data = self.read_db().unwrap_or_else(|_| "{}".to_string()); 
	    let json_data: Value = serde_json::from_str(&db_data).unwrap();

	    if let Some(obj) = json_data.as_object() {
	        obj.keys().cloned().collect()
	    } else {
	        Vec::new()
	    }
	}

    /// Updates or inserts a key-value pair in the session storage.
    pub fn set_kv_session_storage(&mut self, key: &str, value: &str) {
        let key = key.to_string();
        let value = value.to_string();

        match self.session_storage.entry(key) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.insert(value);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
    }

    pub fn add_current_destination_to_session_storage(&mut self) {
        let destination_title = self.session_storage.get("current_country_title");
        let destination_start_date = self.session_storage.get("current_country_start_date");
        let destination_end_date = self.session_storage.get("current_country_end_date");

        if destination_title.is_some() && destination_start_date.is_some() && destination_end_date.is_some() {
            let title = destination_title.expect("destination_title should be present").to_string();

            let map = self.session_storage_nested.entry(title.clone()).or_insert_with(HashMap::new);
            map.insert("start_date".to_string(), destination_start_date.expect("start_date should be present").to_string());
            map.insert("end_date".to_string(), destination_end_date.expect("end_date should be present").to_string());
        	
        	self.session_storage.remove("current_country_title");
        	self.session_storage.remove("current_country_start_date");
        	self.session_storage.remove("current_country_end_date");
        }

        for (k, v) in &self.session_storage_nested {
        	let formatted_map = util::format_hashmap(&v);
        	log(&format!("{}: [{}]", k, &formatted_map));
        }
    }
 
	pub fn store_session_storage_in_database(&mut self) -> Result<String, io::Error> {
	    let session_storage = self.get_session_storage();

	    if let Some(title) = session_storage.get("title") {
	        let db_data = self.read_db()?;
	        let mut json_data: Value = serde_json::from_str(&db_data).unwrap_or_else(|_| json!({}));

	        // Ensure itinerary exists in json_data before updating
	        if !json_data.get(title).is_some() {
	            self.add_itinerary(title.to_string())?;
	            json_data[title] = json!(Itinerary { session_id: self.id });
	        }

	        // Now that the itinerary exists, update it
	        if let Some(itinerary) = json_data.get_mut(title) {
	            if let Value::Object(ref mut map) = itinerary {
	                for (key, value) in &session_storage {
	                	if key.clone() != "title" {
							map.insert(key.clone(), Value::String(value.clone()));
	                	}
	                }
	            }
	        }

	        // Write the updated JSON back to the file
	        let updated_json = serde_json::to_string_pretty(&json_data)?;
	        let mut file = File::create(&self.path)?;
	        file.write_all(updated_json.as_bytes())?;
	        file.flush()?; // Ensure the changes are written to disk

	    } else {
	        return Ok("ERROR_NO_TITLE".to_string());
	    }

	    self.session_storage = HashMap::new();
	    Ok("SUCCESS".to_string())
	}

    /// Creates a new empty database file at the specified path.
    fn initiate_db(path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(b"{\n}")?;
        Ok(())
    }

    /// Reads the database file and returns its content as a String.
    fn read_db(&self) -> io::Result<String> {
        let mut file = File::open(&self.path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Ok(data)
    }

    /// Returns the session ID of the database instance.
    pub fn session_id(&self) -> i64 {
        self.id
    }

    /// Retrieves the database file path.
    #[allow(dead_code)]
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// Adds a new itinerary to the database if it does not already exist.
    pub fn add_itinerary(&self, title: String) -> Result<String, io::Error> {
        let db_data = self.read_db()?;
        let mut itineraries: Value = match serde_json::from_str(&db_data) {
            Ok(itineraries) => itineraries,
            Err(_) => {
                globals::add_to_debug_log(&format!(
                    "ERROR In 'database.rs': Could not parse database JSON file at path {}",
                    self.path
                ));
                return Ok("ERROR_ALREADY_EXISTS".to_string());
            }
        };

        if let Some(map) = itineraries.as_object_mut() {
            if map.contains_key(&title) {
                globals::add_to_debug_log("ERROR In 'database.rs': itinerary already exists");
                return Ok("ERROR_ALREADY_EXISTS".to_string());
            }
            map.insert(title, json!(Itinerary { session_id: self.id }));
        }

        let json_data = serde_json::to_string_pretty(&itineraries)?;

        // Open the file with write permissions and truncate it to overwrite existing content
        let mut file = File::create(&self.path)?;
        file.write_all(json_data.as_bytes())?;
        file.flush()?;

        Ok("SUCCESS".to_string())
    }
}
