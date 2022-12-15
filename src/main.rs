use std::error::Error;

use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use serde_json::Value;

mod kvstore;
use kvstore::KVStore;
use tracing::log::info;

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
    ███    ███   ███    ███   ███    ███ ███   ███   ███ ▀███▄  v0.4.0
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
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn index() -> impl Responder {
    info!("Index page requested");
    "VBank Key-Value Store v0.4.0 Online"
}

async fn get_key(kvs: web::Data<KVStore>, key: web::Path<String>) -> impl Responder {
    match kvs.get(key.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().json(response),
        Err(e) => actix_web::HttpResponse::NotFound().body(e.to_string()),
    }
}

async fn create_key(kvs: web::Data<KVStore>, value: web::Json<Value>) -> impl Responder {
    match kvs.create_key(value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Created().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn create_key_with_key(
    kvs: web::Data<KVStore>,
    key: web::Path<String>,
    value: web::Json<Value>,
) -> impl Responder {
    match kvs.create_key_with_key(key.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Created().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn update_key(
    kvs: web::Data<KVStore>,
    key: web::Path<String>,
    value: web::Json<Value>,
) -> impl Responder {
    match kvs.insert(key.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn delete_key(kvs: web::Data<KVStore>, key: web::Path<String>) -> impl Responder {
    match kvs.delete(key.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn list_keys(kvs: web::Data<KVStore>, query: web::Query<ListQuery>) -> impl Responder {
    match kvs.list_keys(query.skip, query.limit).await {
        Ok(response) => actix_web::HttpResponse::Ok().json(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}
