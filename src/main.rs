use std::error::Error;

use actix_web::{
    web,
    App,
    HttpServer,
    Responder,
    get,
    put,
    patch,
    delete,
};
use serde::Deserialize;
use serde_json::Value;

mod kvstore;
use kvstore::KVStore;
use tracing::log::info;

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
    ███    ███   ███    ███   ███    ███ ███   ███   ███ ▀███▄  v0.6.1
     ▀██████▀  ▄█████████▀    ███    █▀   ▀█   █▀    ███   ▀█▀  by @JakePIXL
                                                     ▀                 
"#
    );
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    print_ascii_art();

    let kvs: KVStore = KVStore::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(kvs.clone()))
            .service(index)
            .service(get_key)
            .service(create_document)
            .service(create_document_with_key)
            .service(update_document)
            .service(delete_document)
            .service(list_documents)
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    info!("Index page requested");
    "VBank Key-Value Store v0.6.1 Online"
}

#[get("/{namespace}/{key}")]
async fn get_key(kvs: web::Data<KVStore>, path: web::Path<(String, String)>) -> impl Responder {

    let (namespace, key) = path.into_inner();

    match kvs.get(namespace.clone(), key.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().json(response),
        Err(e) => actix_web::HttpResponse::NotFound().body(e.to_string()),
    }
}

#[put("/{namespace}/")]
async fn create_document(kvs: web::Data<KVStore>, namespace: web::Path<String>, value: web::Json<Value>) -> impl Responder {
    match kvs.create_document(namespace.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Created().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/{namespace}/{key}")]
async fn create_document_with_key(
    kvs: web::Data<KVStore>,
    path: web::Path<(String, String)>,
    value: web::Json<Value>,
) -> impl Responder {

    let (namespace, key) = path.into_inner();

    match kvs.create_document_with_key(namespace.clone(), key.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Created().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[patch("/{namespace}/{key}")]
async fn update_document(
    kvs: web::Data<KVStore>,
    path: web::Path<(String, String)>,
    value: web::Json<Value>,
) -> impl Responder {

    let (namespace, key) = path.into_inner();

    match kvs.insert(namespace.clone(), key.clone(), value.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/{namespace}/{key}")]
async fn delete_document(kvs: web::Data<KVStore>, path: web::Path<(String, String)>) -> impl Responder {

    let (namespace, key) = path.into_inner();

    match kvs.delete(namespace.clone(), key.clone()).await {
        Ok(response) => actix_web::HttpResponse::Ok().body(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{namespace}/list/")]
async fn list_documents(kvs: web::Data<KVStore>, namespace: web::Path<String>, query: web::Query<ListQuery>) -> impl Responder {
    match kvs.list_documents(namespace.clone(), query.skip, query.limit).await {
        Ok(response) => actix_web::HttpResponse::Ok().json(response),
        Err(e) => actix_web::HttpResponse::InternalServerError().body(e.to_string()),
    }
}
