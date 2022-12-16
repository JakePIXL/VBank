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
    ███    ███   ███    ███   ███    ███ ███   ███   ███ ▀███▄  v0.4.1
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
            .route("/{namespace}/", web::get().to(index))
            .route("/{namespace}/", web::put().to(create_key))
            .route("/{namespace}/{key}", web::get().to(get_key))
            .route("/{namespace}/{key}", web::put().to(create_key_with_key))
            .route("/{namespace}/{key}", web::patch().to(update_key))
            .route("/{namespace}/{key}", web::delete().to(delete_key))
            .route("/{namespace}/list/", web::get().to(list_keys))
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn index() -> impl Responder {
    info!("Index page requested");
    "VBank Key-Value Store v0.4.1 Online"
}

async fn get_key(kvs: web::Data<KVStore>, namespace: web::Path<String>, key: web::Path<String>) -> impl Responder {
    match kvs.get(namespace.clone(), key.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().json(response),
        Err(e) => actix_web::HttpResponse::NotFound().body(e.to_string()),
    }
}

async fn create_key(kvs: web::Data<KVStore>, namespace: web::Path<String>, value: web::Json<Value>) -> impl Responder {
    match kvs.create_key(namespace.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Created().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn create_key_with_key(
    kvs: web::Data<KVStore>,
    namespace: web::Path<String>,
    key: web::Path<String>,
    value: web::Json<Value>,
) -> impl Responder {
    match kvs.create_key_with_key(namespace.clone(), key.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Created().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn update_key(
    kvs: web::Data<KVStore>,
    namespace: web::Path<String>,
    key: web::Path<String>,
    value: web::Json<Value>,
) -> impl Responder {
    match kvs.insert(namespace.clone(), key.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn delete_key(kvs: web::Data<KVStore>, namespace: web::Path<String>, key: web::Path<String>) -> impl Responder {
    match kvs.delete(namespace.clone(), key.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn list_keys(kvs: web::Data<KVStore>, namespace: web::Path<String>, query: web::Query<ListQuery>) -> impl Responder {
    match kvs.list_keys(namespace.clone(), query.skip, query.limit).await {
        Ok(response) => actix_web::HttpResponse::Ok().json(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}
