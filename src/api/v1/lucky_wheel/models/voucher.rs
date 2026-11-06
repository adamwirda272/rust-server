use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::doc,
    error::Result,
    results::{InsertOneResult, UpdateResult},
};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voucher {
    #[serde(rename = "_id")]
    pub id: String,
    pub user: String,
    pub code: String,
    pub is_used: bool,
    pub created_at: String,
    pub prize: Option<String>,
    pub used_at: Option<String>,
}

pub fn generate_voucher_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    let len: u8 = 6;
    (0..len)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

impl Voucher {
    pub fn new(user: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user: user.to_string(),
            code: generate_voucher_code(),
            is_used: false,
            created_at: Utc::now().to_rfc3339(),
            prize: None,
            used_at: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vouchers {
    pub coll: Collection<Voucher>,
}

impl Vouchers {
    pub fn load(db: &Database) -> Self {
        Self {
            coll: db.collection::<Voucher>("vouchers"),
        }
    }

    pub async fn add(&self, voucher: &Voucher) -> Result<InsertOneResult> {
        self.coll.insert_one(voucher).await
    }

    pub async fn get_all(&self) -> Result<Vec<Voucher>> {
        let cursor = self.coll.find(doc! {}).await?;
        let vouchers: Vec<Voucher> = cursor.try_collect().await?;
        Ok(vouchers)
    }

    pub async fn find_active(&self, code: &str) -> Result<Option<Voucher>> {
        self.coll
            .find_one(doc! { "code": code, "is_used": false })
            .await
    }

    pub async fn update(&self, voucher: &Voucher) -> Result<UpdateResult> {
        self.coll
            .update_one(
                doc! {"_id": &voucher.id},
                doc! {
                    "$set": {
                        "is_used": true,
                        "prize": Some(&voucher.prize),
                        "used_at": Some(Utc::now().to_rfc3339())
                    }
                },
            )
            .await
    }
}
