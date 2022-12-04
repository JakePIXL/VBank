use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::{fs, mem};
use std::fs::File;
use std::sync::{Arc, Mutex, MutexGuard};
use tracing::info;
use tracing_subscriber;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct KV {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct KVList {
    kvs: Vec<KV>,
}

#[derive(Clone)]
struct KVStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}

fn check_file_exists() -> File {
    let path = "kvstore.txt";
    let file_exists = fs::metadata(path).is_ok();
    if file_exists {
        return File::open(path).unwrap();
    } else {
        let file = File::create(path).unwrap();
        return file;
    }
}

fn read_kvstore(kvstore: Arc<Mutex<HashMap<String, String>>>) -> Result<(), Box<dyn Error>> {
    let mut file = check_file_exists();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut kvstore_file = kvstore.lock().unwrap();
    for line in contents.lines() {
        let mut kv = line.split(":");
        let key = kv.next().unwrap();
        let value = kv.next().unwrap();
        kvstore_file.insert(key.to_string(), value.to_string());
    }
    Ok(())
}

fn write_kvstore(kvstore: &MutexGuard<HashMap<String, String>>) -> Result<(), Box<dyn Error>> {

    info!("Writing to data to disk");

    // Handle the `Result` returned by `File::open`.
    let mut file = File::create("./kvstore.txt")?;
    let kvstore_file = kvstore;
    for (key, value) in kvstore_file.iter() {
        file.write_all(format!("{}:{}\n", key, value).as_bytes())?;
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting in-memory key-value store");
    
    let kvs: KVStore = KVStore {
        store: Arc::new(Mutex::new(HashMap::new())),
    };
    
    read_kvstore(kvs.store.clone()).unwrap();

    info!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(kvs.store.clone()))
            .route("/", web::get().to(index))
            .route("/{key}", web::get().to(get_key))
            .route("/{key}", web::put().to(put_key))
            .route("/{key}", web::delete().to(delete_key))
            .route("/list/", web::get().to(list_keys))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn index() -> impl Responder {

    info!("index hit");

    "Hello world!"
}

async fn get_key(kvs: web::Data<Arc<Mutex<HashMap<String, String>>>>, key: web::Path<String>) -> impl Responder {
    
    info!("get_key hit");

    let kvs = kvs.lock().unwrap();
    match kvs.get(&key.clone()) {
        Some(value) => format!("{}", value),
        None => format!("Key not found"),
    }
}

async fn put_key(kvs: web::Data<Arc<Mutex<HashMap<String, String>>>>, key: web::Path<String>, value: web::Json<String>) -> impl Responder {
    
    info!("put_key hit");
    
    let mut kvs = kvs.lock().unwrap();
    kvs.insert(key.to_string(), value.to_string());

    // Save the data to disk by calling the `write_kvstore` function.
    write_kvstore(&kvs).expect("Error writing to disk");

    format!("Key inserted")
}

async fn delete_key(kvs: web::Data<Arc<Mutex<HashMap<String, String>>>>, key: web::Path<String>) -> impl Responder {
    let mut kvs = kvs.lock().unwrap();
    let response = match kvs.remove(&key.clone()) {
        Some(_) => format!("Key deleted"),
        None => format!("Key not found"),
    };


    write_kvstore(&kvs).expect("Error writing to disk");

    response
}

async fn list_keys(kvs: web::Data<Arc<Mutex<HashMap<String, String>>>>) -> impl Responder {
   
    info!("list_keys hit");

    let kvs = kvs.lock().unwrap();
    let mut kv_list = KVList { kvs: Vec::new() };
    for (key, value) in kvs.iter() {
        kv_list.kvs.push(KV {
            key: key.to_string(),
            value: value.to_string(),
        });
    }
    web::Json(kv_list)
}