use mongodb::{Collection,Database,results::{InsertOneResult, DeleteResult, UpdateResult},bson::{doc, oid::ObjectId}};
use crate::models::wallet::Wallet;
use futures::stream::TryStreamExt;

pub struct WalletRepo {
    collection: Collection<Wallet>
}

impl WalletRepo {
    pub async fn init(db: &Database) -> Self {
        let col: Collection<Wallet> = db.collection("wallet");

        WalletRepo {collection: col}
    }
    pub async fn create_wallet(&self, data: Wallet) -> Result<InsertOneResult,mongodb::error::Error> {
        let new_wallet = self.collection.insert_one(data, None).await.ok().expect("Could not add new wallet");

        Ok(new_wallet)
    }
    pub async fn get_wallet(&self, path: (String,String)) -> Result<Wallet,mongodb::error::Error> {
        let wallet = self.collection.find_one(doc! {"_id": path.1,"user_id": path.0},None).await.ok().expect("Could not get wallet");

        Ok(wallet.unwrap())
    }
    pub async fn get_user_wallets(&self, user_id: String) -> Result<Vec<Wallet>,mongodb::error::Error> {
        let mut wallets: Vec<Wallet> = Vec::new();
        let mut cursors = self.collection.find(doc! {"user_id": user_id},None).await.ok().expect("Could not fetch wallets");

        while let Some(wallet) = cursors.try_next().await.ok().expect("Error mapping cursor") {
            wallets.push(wallet);
        }

        Ok(wallets)
    }
    pub async fn update_wallet(&self, wallet: Wallet) -> Result<UpdateResult,mongodb::error::Error> {
        let update_result = self.collection.replace_one(doc! {"_id": wallet.id}, wallet, None).await.expect("Could not update wallet");

        Ok(update_result)
    }
    pub async fn delete_wallet(&self, wallet_id: String) -> Result<DeleteResult,mongodb::error::Error> {
        let object_wallet_id = ObjectId::parse_str(&wallet_id).unwrap();

        let delete_result = self.collection.delete_one(doc! {"_id": object_wallet_id}, None).await.expect("Could not delete wallet");

        Ok(delete_result)
    }
}
