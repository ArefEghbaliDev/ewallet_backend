use std::sync::Mutex;

use actix_web::{get,post,delete,put,Responder,HttpResponse, web::{Json, self}};
use mongodb::{bson::doc, Collection};
use futures::stream::TryStreamExt;
use crate::{models, AppState};

use models::wallet::Wallet;

#[get("/wallets/{user_id}")]
pub async fn get_user_wallets(data: web::Data<Mutex<AppState>>, user_id: web::Path<String>) -> impl Responder {
    let collection: Collection<Wallet> = data.lock().unwrap().db.collection("wallet");
    let userid = user_id.into_inner();

    let mut wallets: Vec<Wallet> = Vec::new();
    let mut cursors = collection.find(doc! {"user_id": userid},None).await.ok().expect("Could not fetch wallets");

    while let Some(wallet) = cursors.try_next().await.ok().expect("Error mapping cursor") {
        wallets.push(wallet);
    }

    HttpResponse::Ok().json(wallets)

}

#[get("/wallet/{user_id}/{wallet_id}")]
pub async fn get_single_wallet(data: web::Data<Mutex<AppState>>, path: web::Path<(String,String)>) -> impl Responder {
    let wallet_result = data.lock().unwrap().wallet_repo.get_wallet(path.into_inner()).await;

    match wallet_result {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[post("/wallet")]
pub async fn create_wallet(data: web::Data<Mutex<AppState>>,body: Json<Wallet>) -> impl Responder {
    let new_wallet = data.lock().unwrap().wallet_repo.create_wallet(body.into_inner()).await;

    match new_wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[delete("/wallet/{id}")]
pub async fn delete_wallet(data: web::Data<Mutex<AppState>>, path: web::Path<String>) -> impl Responder {
    let wallet_id = path.into_inner();

    if wallet_id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid Wallet Id");
    }

    let delete_result = data.lock().unwrap().wallet_repo.delete_wallet(wallet_id).await;

    match delete_result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Wallet deleted successfully");
            }
            else {
                println!("Could not find wallet");
                return HttpResponse::NotFound().json("Could not find wallet");
            }
        }
        Err(err) => {
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    }
}

#[put("/wallet/{id}")]
pub async fn update_wallet(data: web::Data<Mutex<AppState>>, body: Json<Wallet>) -> impl Responder {
    let update_result = data.lock().unwrap().wallet_repo.update_wallet(body.into_inner()).await;

    match update_result {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}
