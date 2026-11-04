use crate::api::v1::lucky_wheel::config::Config;
use futures::TryStreamExt;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
    error::Result,
    results::{InsertOneResult, UpdateResult},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelItem {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub weight: u32,
}

impl WheelItem {
    pub fn create(name: &str, weight: u32) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            weight,
        }
    }

    pub fn insert_id(&mut self, id: ObjectId)  {
        self.id = Some(id);
    }
}

pub struct WheelItems {
    pub coll: Collection<WheelItem>,
}

impl WheelItems {
    pub fn load(cfg: &Config) -> Self {
        let coll = cfg.db.collection::<WheelItem>("wheel_items");
        Self { coll }
    }

    pub async fn add(&self, item: &WheelItem) -> Result<InsertOneResult> {
        self.coll.insert_one(item).await
    }

    pub async fn get_all(&self) -> Result<Vec<WheelItem>> {
        let cursor = self.coll.find(doc! {}).await?;
        let items: Vec<WheelItem> = cursor.try_collect().await?;
        Ok(items)
    }

    pub async fn update(&self, item: &WheelItem) -> Result<UpdateResult> {
        self.coll
            .update_one(
                doc! { "_id": item.id },
                doc! { "$set": { "name": &item.name, "weight": item.weight } },
            )
            .await
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SpinResult {
//     pub winning_item: WheelItem,
//     pub winning_index: usize,
// }
