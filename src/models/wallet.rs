use serde::{Serialize,Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize,Deserialize)]
pub struct Wallet {
    #[serde(rename = "_id",skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub user_id: String,
    pub currency: String,
    pub created_date: String,
    pub transactions: Vec<Transaction>
}

#[derive(Serialize,Deserialize)]
pub struct Transaction {
    pub id: String,
    pub title: String,
    pub description: String,
    pub amount: f64,
    pub transaction_type: String,
    pub created_date: String
}
