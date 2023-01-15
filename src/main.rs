use actix_web::{HttpServer,App,get,web,middleware::Logger, Responder, HttpResponse};
use mongodb::{Client, Database};
use repository::wallet_repo::WalletRepo;

use std::sync::Mutex;

mod routes;
mod models;
mod controllers;
mod repository;

use routes::wallet::{get_user_wallets,create_wallet, delete_wallet,update_wallet};

pub struct AppState {
    pub wallet_repo: WalletRepo,
    pub db: Database
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Ewallet")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // setup logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // setup and connect to mongodb
    let client_uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(&client_uri).await.unwrap();
    let db = client.database("ewallet");

    let wallet_repo = WalletRepo::init(&db).await;

    let app_state = web::Data::new(Mutex::new(AppState {
        db,
        wallet_repo
    }));

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).app_data(app_state.clone()).service(get_user_wallets).service(create_wallet).service(delete_wallet).service(update_wallet)
    }).bind(("127.0.0.1", 8080))?.run().await
}