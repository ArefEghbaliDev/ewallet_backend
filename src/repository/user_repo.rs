use mongodb::{Database, Collection,error::Error, results::{InsertOneResult, DeleteResult}, bson::{doc,oid::ObjectId}};

use crate::models::user::{User, LoginData};

pub struct UserRepo {
    collection: Collection<User>
}

impl UserRepo {
    pub async fn init(db: &Database) -> Self {
        let col: Collection<User> = db.collection("wallet");

        UserRepo {collection: col}
    }
    pub async fn create_user(&self, user: User) -> Result<InsertOneResult,Error> {
        let new_user = self.collection.insert_one(user, None).await.expect("Could not create User");

        Ok(new_user)
    }
    pub async fn get_user(&self, data: LoginData) -> Result<User,Error> {
        let user = self.collection.find_one(doc! {"email": data.email, "password": data.password}, None).await.expect("Could not get user");

        Ok(user.unwrap())
    }
    pub async fn delete_user(&self, user_id: String) -> Result<DeleteResult,Error> {
        let user_objectid = ObjectId::parse_str(user_id).unwrap();
        let delete_result = self.collection.delete_one(doc! {"_id": user_objectid}, None).await.expect("Could not delete user");

        Ok(delete_result)
    }
}
