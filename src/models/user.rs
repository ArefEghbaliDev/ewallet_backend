use serde::{Serialize,Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize,Deserialize)]
pub struct User {
    #[serde(rename = "_id",skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub joined_date: String,
}

#[derive(Serialize,Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String
}
