use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tracing::{info, warn};
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::{collections::BTreeMap, fs::File};
use std::sync::RwLock;
use rand::{thread_rng, Rng};
use rand_distr::Alphanumeric;

#[derive(Serialize, Deserialize, Debug)]
struct KV {
    key: String,
    data: Value,
}

pub struct KVStore {
    pub store: RwLock<BTreeMap<String, Value>>,
}

impl KVStore {

    pub fn new() -> Self {
        KVStore {
            store: RwLock::new(BTreeMap::new()),
        }
    }

    // Generate a 8 character string for the key
    fn generate_random_string(key_length: usize) -> String {
        let rng = thread_rng();

        // Generate a random string of the given length
        let chars: Vec<char> = rng.sample_iter(&Alphanumeric)
            .map(|x| x as char)
            .take(key_length)
            .collect();
        chars.into_iter().collect()
    }

    pub async fn create_key(&self, value: web::Json<Value>) -> impl Responder {
        let mut key = Self::generate_random_string(8);
        {
            // Check if the key already exists in the database
            let mut kvs = self.store.write().unwrap();

            while kvs.contains_key(&key) {
                // Generate a new key if the key already exists
                key = Self::generate_random_string(8);
            }
            
            // Insert the key-value pair into the database
            kvs.insert(key.to_string(), value.clone());
        }
        
        // Save the data to disk by calling the `write_kvstore` function.
        write_kvstore(&self.store).expect("Error writing to disk");
        
        info!("created key: {}", key);
        
        format!("Key created: {}", key)
    }

    pub async fn create_key_with_key(&self, key: web::Path<String>, value: web::Json<Value>) -> impl Responder {
        {
            // Check if the key already exists in the database
            let mut kvs = self.store.write().unwrap();
            if kvs.contains_key(&key.to_string()) {
                return actix_web::HttpResponse::Conflict().body("Key already exists");
            }
            
            // Insert the key-value pair into the database
            kvs.insert(key.to_string(), value.clone());
        }
        
        // Save the data to disk by calling the `write_kvstore` function.
        write_kvstore(&self.store).expect("Error writing to disk");
        
        info!("created key: {}", key);
        
        actix_web::HttpResponse::Ok().body(format!("Key created: {}", key))
    }

    pub async fn insert(&self, key: web::Path<String>, value: web::Json<Value>) -> impl Responder {
        let mut store = self.store.write().unwrap();

        store.insert(key.clone(), value.to_owned());
        
        actix_web::HttpResponse::Ok().body(format!("Key created: {}", key))
    }

    pub async fn get(&self, key: web::Path<String>) -> impl Responder {

        let store = self.store.read().unwrap();

        if !store.contains_key(&key.to_string()) {
            warn!("Key not found: {}", key);
            return actix_web::HttpResponse::NotFound().body("Key not found");
        }

        actix_web::HttpResponse::Ok().body(store.get(&key.to_string()).unwrap().to_string())
    }

    pub async fn delete(&self, key: web::Path<String>) -> impl Responder {
        let mut store = self.store.write().unwrap();
        
        if store.contains_key(&key.to_string()) {
            store.remove(&key.to_string());
            actix_web::HttpResponse::Ok().body(format!("Key deleted: {}", key))
        } else {
            actix_web::HttpResponse::NotFound().body("Key not found")
        }
    }

    pub async fn list_keys(&self, skip: Option<u64>, limit: Option<u64>) -> impl Responder {
        let kvs = &self.store.read().unwrap();
        let mut kv_list = Vec::new();

        // Determine the skip and limit values. If they are not specified in the
        // query parameters, the default values of 0 will be used.
        let skip = skip.unwrap_or(0);
        let limit = limit.unwrap_or(1000);

        // Iterate over the keys and values in the `kvs` hash map, starting at
        // the index specified by `skip`.
        let mut count = 0;
        for (key, value) in kvs.iter().skip(skip.clone() as usize) {
            if count >= limit {
                break;
            }
            kv_list.push(KV {
                key: key.to_string(),
                data: value.clone(),
            });
            count += 1;
        }

        web::Json(kv_list)
    }
}

impl Clone for KVStore {
    fn clone(&self) -> Self {
        KVStore {
            store: RwLock::new(self.store.read().unwrap().clone()),
        }
    }
}

fn check_file_exists() -> File {
    let path = "database.vbank";
    let file_exists = fs::metadata(path).is_ok();
    if file_exists {
        match File::open(path) {
            Ok(file) => return file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    } else {
        File::create(path).unwrap();

        match File::open(path) {
            Ok(file) => return file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        }
    }
}

pub fn read_kvstore(kvstore: &RwLock<BTreeMap<String, Value>>) -> Result<(), Box<dyn Error>> {
    let mut file = check_file_exists();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut kvstore_file = kvstore.write().unwrap();
    for line in contents.lines() {
        let mut kv = line.split("|");
        let key = kv.next().unwrap();
        let value = kv.next().unwrap_or("");

        if key.is_empty() || value.is_empty() {
            continue;
        }

        // Use the `serde_json` crate to deserialize the value from JSON.
        // Check if the value string starts and ends with double quotes, and remove them if it does.
        let value = if value.starts_with('"') && value.ends_with('"') {
            &value[1..value.len() - 1]
        } else {
            value
        };
        let json_value = serde_json::from_str(value)?;

        kvstore_file.insert(key.to_string(), json_value);
    }
    let count = kvstore_file.len();
    info!("Loaded {} keys from disk", count);
    Ok(())
}

fn write_kvstore(kvstore: &RwLock<BTreeMap<String, Value>>) -> Result<(), Box<dyn Error>> {
    info!("Writing to data to disk");

    // Handle the `Result` returned by `File::open`.
    let mut file = File::create("./database.vbank")?;
    let kvstore_file = kvstore.read().unwrap();
    for (key, value) in kvstore_file.iter() {
        // Use the `serde_json` crate to serialize the value to JSON.
        let json_value = serde_json::to_string(value)?;

        // Check if the JSON string contains a pipe character, and escape it if it does.
        let json_value = json_value.replace("|", "\\|");

        // Use a delimiter that cannot appear in the JSON string.
        file.write_all(format!("{}|{}\n", key, json_value).as_bytes())?;
    }
    Ok(())
}