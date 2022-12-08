use std::error::Error;

use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize};
use serde_json::Value;
use tracing::info;
use tracing_subscriber;

mod kvstore;
use kvstore::KVStore;

// use crate::kvstore::read_kvstore;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
   skip: Option<u64>,
   limit: Option<u64>,
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
    ███    ███   ███    ███   ███    ███ ███   ███   ███ ▀███▄  v0.2.1
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
    
    let kvs: KVStore = KVStore::new();

    info!("Starting server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(kvs.clone()))
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

    Ok(())
}

async fn index() -> impl Responder {
    "VBank Key-Value Store Online"
}

async fn get_key(
    kvs: web::Data<KVStore>,
    key: web::Path<String>,
) -> impl Responder {
    kvs.get(key).await
}

async fn create_key(kvs: web::Data<KVStore>, value: web::Json<Value>) -> impl Responder {
    kvs.create_key(value).await
}

async fn create_key_with_key(kvs: web::Data<KVStore>, key: web::Path<String>, value: web::Json<Value>) -> impl Responder {
    kvs.create_key_with_key(key, value).await
}
    
async fn update_key(kvs: web::Data<KVStore>, key: web::Path<String>, value: web::Json<Value>) -> impl Responder {
    kvs.insert(key, value).await
}

async fn delete_key(kvs: web::Data<KVStore>, key: web::Path<String>) -> impl Responder {
    kvs.delete(key).await
}

async fn list_keys(
    kvs: web::Data<KVStore>,
    query: web::Query<ListQuery>
) -> impl Responder {
    kvs.list_keys(query.skip, query.limit).await
}
