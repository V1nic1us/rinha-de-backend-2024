mod handlers;
mod models;

use std::env;
use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;
use crate::handlers::{ create_transaction, get_all};


struct ConnectionManager(String);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=secret dbname=test", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    HttpServer::new(|| {
        App::new()
            .app_data(client.clone())
            .service(create_transaction)
            .service(get_all)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}