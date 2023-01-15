use mongodb::{Collection, results::{InsertOneResult, UpdateResult},bson::{doc,oid::ObjectId}};
use actix_web::{web::{Json}};
use crate::models::wallet::Wallet;

pub async fn create_wallet(collection: &Collection<Wallet>, data: Wallet) -> Result<InsertOneResult,mongodb::error::Error> {
    let result = collection.insert_one(data,None).await.ok().expect("Could not insert wallet");

    Ok(result)
}

pub async fn get_wallet(collection: &Collection<Wallet>, wallet_id: &str) -> Result<Wallet,mongodb::error::Error> {
    let result = collection.find_one(doc! {"_id": ObjectId::parse_str(wallet_id).unwrap()}, None).await.ok().expect("Could not get wallet");

    Ok(result.unwrap())
}

pub async fn update_wallet(collection: &Collection<Wallet>, data: Json<Wallet>) -> Result<UpdateResult,mongodb::error::Error> {
    let wallet = data.into_inner();
    let result = collection.replace_one(doc! {"_id": wallet.id}, wallet, None).await;

    Ok(result.unwrap())
}