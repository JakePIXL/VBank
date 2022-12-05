use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::{fs};
use std::fs::File;
// use std::sync::{Arc, Mutex};
use tracing::info;
use tracing_subscriber;
use std::io::{Read, Write};
use rand::{thread_rng, Rng};
use rand_distr::Alphanumeric;
use parking_lot::RwLock;

#[derive(Serialize, Deserialize, Debug)]
struct KV {
    key: String,
    data: Value,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
   skip: Option<u64>,
   limit: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct KVList {
    kvs: Vec<KV>,
}

struct KVStore {
    store: RwLock<HashMap<String, Value>>,
}

impl Clone for KVStore {
    fn clone(&self) -> Self {
        KVStore {
            store: RwLock::new(self.store.read().clone()),
        }
    }
}

fn check_file_exists() -> File {
    let path = "database.vbank";
    let file_exists = fs::metadata(path).is_ok();
    if file_exists {
        return File::open(path).unwrap();
    } else {
        let file = File::create(path).unwrap();
        return file;
    }
}

fn read_kvstore(kvstore: &RwLock<HashMap<String, Value>>) -> Result<(), Box<dyn Error>> {
    let mut file = check_file_exists();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut kvstore_file = kvstore.write();
    for line in contents.lines() {
        let mut kv = line.split("|");
        let key = kv.next().unwrap();
        let value = kv.next().unwrap();

        // Use the `serde_json` crate to deserialize the value from JSON.
        let json_value = serde_json::from_str(value)?;

        kvstore_file.insert(key.to_string(), json_value);
    }
    let count = kvstore_file.capacity();
    info!("Loaded {} keys from disk", count);
    Ok(())
}

fn write_kvstore(kvstore: &RwLock<HashMap<String, Value>>) -> Result<(), Box<dyn Error>> {
    info!("Writing to data to disk");

    // Handle the `Result` returned by `File::open`.
    let mut file = File::create("database.vbank")?;
    let kvstore_file = kvstore.read();
    for (key, value) in kvstore_file.iter() {
        // Use the `serde_json` crate to serialize the value to JSON.
        let json_value = serde_json::to_string(value)?;

        // Use a delimiter that cannot appear in the JSON string.
        file.write_all(format!("{}|{}\n", key, json_value).as_bytes())?;
    }
    Ok(())
}

fn print_ascii_art() {
    info!(
        r#"
        

    ▄█    █▄  ▀█████████▄     ▄████████ ███▄▄▄▄      ▄█   ▄█▄ 
    ███    ███   ███    ███   ███    ███ ███▀▀▀██▄   ███ ▄███▀ 
    ███    ███   ███    ███   ███    ███ ███   ███   ███▐██▀   
    ███    ███  ▄███▄▄▄██▀    ███    ███ ███   ███  ▄█████▀    
    ███    ███ ▀▀███▀▀▀██▄  ▀███████████ ███   ███ ▀▀█████▄    
    ███    ███   ███    ██▄   ███    ███ ███   ███   ███▐██▄   
    ███    ███   ███    ███   ███    ███ ███   ███   ███ ▀███▄  v0.1.0
     ▀██████▀  ▄█████████▀    ███    █▀   ▀█   █▀    ███   ▀█▀  by @JakePIXL
                                                     ▀                 
"#
    );
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    print_ascii_art();

    info!("Starting in-memory key-value store");
    
    let kvs: KVStore = KVStore {
        store: RwLock::new(HashMap::new()),
    };
    
    read_kvstore(&kvs.store).unwrap();

    info!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(kvs.clone().store))
            .route("/", web::get().to(index))
            .route("/", web::put().to(create_key))
            .route("/{key}", web::get().to(get_key))
            .route("/{key}", web::put().to(create_key_with_key))
            .route("/{key}", web::patch().to(update_key))
            .route("/{key}", web::delete().to(delete_key))
            .route("/list/", web::get().to(list_keys))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    // write_kvstore(&kvs.store).unwrap();

    Ok(())
}

async fn index() -> impl Responder {
    "VBank Key-Value Store Online"
}

async fn get_key(
    kvs: web::Data<RwLock<HashMap<String, Value>>>,
    key: web::Path<String>,
) -> impl Responder {
    let kvs = kvs.read();
    match kvs.get(&key.clone()) {
        Some(value) => {
            let data: Value = value.clone();
            info!("grabbed key: {}", key);
            web::Json(KV { key: key.clone(), data })
        }
        None => {
            info!("key not found: {}", key);
            web::Json(KV {
                key: key.clone(),
                data: Value::Null,
            })
        }
    }
}

// async fn put_key(kvs: web::Data<Arc<Mutex<HashMap<String, Value>>>>, key: web::Path<String>, value: web::Json<Value>) -> impl Responder {
    
//     let mut kvs = kvs.lock().unwrap();
//     kvs.insert(key.to_string(), value.clone());

//     // Save the data to disk by calling the `write_kvstore` function.
//     write_kvstore(&kvs).expect("Error writing to disk");

//     info!("put key: {}", key);

//     format!("Key inserted")
// }

async fn create_key(kvs: web::Data<RwLock<HashMap<String, Value>>>, value: web::Json<Value>) -> impl Responder {
    // Generate a 8 character string for the key
    let mut key = generate_random_string(8);
    {
        // Check if the key already exists in the database
        let mut kvs = kvs.write();
        while kvs.contains_key(&key) {
            // Generate a new key if the key already exists
            key = generate_random_string(8);
        }
        
        // Insert the key-value pair into the database
        kvs.insert(key.to_string(), value.clone());
    }
    
    // Save the data to disk by calling the `write_kvstore` function.
    write_kvstore(&kvs).expect("Error writing to disk");
    
    info!("created key: {}", key);
    
    format!("Key created: {}", key)
}

async fn create_key_with_key(kvs: web::Data<RwLock<HashMap<String, Value>>>, key: web::Path<String>, value: web::Json<Value>) -> impl Responder {

    // Check if the key already exists in the database


    {
        let mut kvs = kvs.write();
        let mut key = key.to_string();

        while kvs.contains_key(&key.clone()) {
            // Generate a new key if the key already exists
            key = generate_random_string(8);
        }
        
        // Insert the key-value pair into the database
        kvs.insert(key.to_string(), value.clone());
    }

    // Save the data to disk by calling the `write_kvstore` function.
    write_kvstore(&kvs).expect("Error writing to disk");
    
    info!("created key: {}", key);
    
    format!("Key created: {}", key)
}
    
async fn update_key(kvs: web::Data<RwLock<HashMap<String, Value>>>, key: web::Path<String>, value: web::Json<Value>) -> impl Responder {
    {
        let mut kvs = kvs.write();

        // Check if the key exists in the database
        if !kvs.contains_key(&key.clone()) {
            return format!("Key does not exist: {}", key);
        }
        
        // Update the key-value pair in the database
        kvs.insert(key.to_string(), value.clone());
    }
    // Save the data to disk by calling the `write_kvstore` function.
    write_kvstore(&kvs).expect("Error writing to disk");
    
    info!("updated key: {}", key);
    
    format!("Key updated: {}", key)
}
    
fn generate_random_string(length: usize) -> String {
    // Get a reference to the default thread-local random number generator
    let rng = thread_rng();

    // Generate a random string of the given length
    let chars: Vec<char> = rng.sample_iter(&Alphanumeric)
        .map(|x| x as char)
        .take(length)
        .collect();
    chars.into_iter().collect()
}

async fn delete_key(kvs: web::Data<RwLock<HashMap<String, Value>>>, key: web::Path<String>) -> impl Responder {
    let response = {
        let mut kvs = kvs.write();
        let response = match kvs.remove(&key.clone()) {
            Some(_) => {
                info!("deleted key: {}", key);
                format!("Key deleted")
            },
            None => {
                info!("key not found: {}", key);
                format!("Key not found")
            },
        };
        response
    };

    write_kvstore(&kvs).expect("Error writing to disk");

    response
}

async fn list_keys(
    kvs: web::Data<RwLock<HashMap<String, Value>>>,
    query: web::Query<ListQuery>
) -> impl Responder {
    info!("listing keys");
    let kvs = kvs.read();
    let mut kv_list = Vec::new();

    // Determine the skip and limit values. If they are not specified in the
    // query parameters, the default values of 0 will be used.
    let skip = query.skip.unwrap_or(0);
    let limit = query.limit.unwrap_or(1000);

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
