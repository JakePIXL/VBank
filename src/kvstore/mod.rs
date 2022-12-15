use rand::{thread_rng, Rng};
use rand_distr::Alphanumeric;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::{collections::BTreeMap, fs::File};
use tracing::{info, warn};

mod errors;
use errors::KVStoreError;

#[derive(Serialize, Deserialize, Debug)]
struct KV {
    key: String,
    data: Value,
}

pub struct KVStore {
    pub store: Arc<Mutex<BTreeMap<String, Value>>>,
}

impl KVStore {
    pub fn new() -> Self {
        let kvs = KVStore {
            store: Arc::new(Mutex::new(BTreeMap::new())),
        };
        {
            read_kvstore(&kvs.store).unwrap();
        }
        kvs
    }

    fn generate_random_string(key_length: usize) -> String {
        let rng = thread_rng();

        let chars: Vec<char> = rng
            .sample_iter(&Alphanumeric)
            .map(|x| x as char)
            .take(key_length)
            .collect();
        chars.into_iter().collect()
    }

    pub async fn create_key(&self, value: Value) -> Result<String, Box<dyn Error>> {
        let mut key = Self::generate_random_string(8);
        {
            let mut kvs = self.store.lock().unwrap();

            while kvs.contains_key(&key) {
                key = Self::generate_random_string(8);
            }

            kvs.insert(key.to_string(), value.clone());
        }

        write_kvstore(&self.store).expect("Error writing to disk");

        info!("Created key: {}", key);

        Ok(format!("Key created: {}", key))
    }

    pub async fn create_key_with_key(
        &self,
        key: String,
        value: Value,
    ) -> Result<String, Box<dyn Error>> {
        {
            let mut kvs = self.store.lock().unwrap();
            if kvs.contains_key(&key.to_string()) {
                return Err(Box::new(KVStoreError::new("Key already exists")));
            }

            kvs.insert(key.to_string(), value.clone());
        }

        write_kvstore(&self.store).expect("Error writing to disk");

        info!("Created key: {}", key);

        Ok(format!("Key created: {}", key))
    }

    pub async fn insert(&self, key: String, value: Value) -> Result<String, Box<dyn Error>> {
        let mut store = self.store.lock().unwrap();

        info!("Patched key: {}", key);

        store.insert(key.clone(), value.to_owned());

        Ok(format!("Key created: {}", key))
    }

    pub async fn get(&self, key: String) -> Result<Value, Box<dyn Error>> {
        let store = self.store.lock().unwrap();

        if !store.contains_key(&key.to_string()) {
            warn!("Key not found: {}", key);
            return Err(Box::new(KVStoreError::new(
                format!("Key not found: {}", key).as_str(),
            )));
        }

        info!("Grabbing key: {}", key);

        Ok(store.get(&key.to_string()).unwrap().to_owned())
    }

    pub async fn delete(&self, key: String) -> Result<String, Box<dyn Error>> {
        let mut store = self.store.lock().unwrap();

        if store.contains_key(&key.to_string()) {
            store.remove(&key.to_string());

            info!("Deleted key: {}", key);

            Ok(format!("Key deleted: {}", key))
        } else {
            warn!("Delete error - Key not found: {}", key);
            Err(Box::new(KVStoreError::new(&format!(
                "Key not found: {}",
                key
            ))))
        }
    }

    pub async fn list_keys(
        &self,
        skip: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Value, Box<dyn Error>> {
        let kvs = &self.store.lock().unwrap();
        let mut kv_list = Vec::new();

        let skip = skip.unwrap_or(0);
        let limit = limit.unwrap_or(1000);

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

        if count == 0 {
            info!("No documents found");
            return Err(Box::new(KVStoreError::new("No documents found")));
        }

        info!("Returning {} keys after skipping {}", count, skip);

        Ok(serde_json::json!(kv_list))
    }
}

impl Clone for KVStore {
    fn clone(&self) -> Self {
        KVStore {
            store: Arc::new(Mutex::new(self.store.lock().unwrap().clone())),
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

fn read_kvstore(kvstore: &Arc<Mutex<BTreeMap<String, Value>>>) -> Result<(), Box<dyn Error>> {
    let mut file = check_file_exists();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut kvstore_file = kvstore.lock().unwrap();
    for line in contents.lines() {
        let mut kv = line.split("|");
        let key = kv.next().unwrap();
        let value = kv.next().unwrap_or("");

        if key.is_empty() || value.is_empty() {
            continue;
        }

        let value = if value.starts_with('"') && value.ends_with('"') {
            &value[1..value.len() - 1]
        } else {
            value
        };
        let decoded_value = base64::decode(value)?;
        let json_value = serde_json::from_slice(&decoded_value)?;

        kvstore_file.insert(key.to_string(), json_value);
    }
    let count = kvstore_file.len();
    info!("Loaded {} keys from disk", count);
    Ok(())
}

pub fn write_kvstore(kvstore: &Arc<Mutex<BTreeMap<String, Value>>>) -> Result<(), Box<dyn Error>> {
    info!("Writing to data to disk");

    let mut file = File::create("./database.vbank")?;
    let kvstore_file = kvstore.lock().unwrap();
    for (key, value) in kvstore_file.iter() {
        let json_value = serde_json::to_string(value)?;
        let encoded_value = base64::encode(&json_value);

        let json_value = encoded_value.replace("|", "\\|");

        file.write_all(format!("{}|{}\n", key, json_value).as_bytes())?;
    }
    Ok(())
}