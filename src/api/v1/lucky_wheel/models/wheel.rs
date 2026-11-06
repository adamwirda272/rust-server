use crate::api::v1::lucky_wheel::dto::api_request::WheelItemUpdate;
use futures::TryStreamExt;
use mongodb::{
    Collection, Database,
    bson::doc,
    error::Result,
    options::{FindOneAndUpdateOptions, ReturnDocument},
    results::InsertOneResult,
};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelItem {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub weight: u16,
}

impl WheelItem {
    pub fn create(name: &str, weight: u16) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            weight,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinResult {
    pub winning_item: WheelItem,
    pub winning_index: usize,
}

#[derive(Debug, Clone)]
pub struct WheelItems {
    pub coll: Collection<WheelItem>,
}

impl WheelItems {
    pub fn load(db: &Database) -> Self {
        Self {
            coll: db.collection::<WheelItem>("wheel_items"),
        }
    }

    pub async fn add(&self, item: &WheelItem) -> Result<InsertOneResult> {
        self.coll.insert_one(item).await
    }

    pub async fn get_all(&self) -> Result<Vec<WheelItem>> {
        let cursor = self.coll.find(doc! {}).await?;
        let items: Vec<WheelItem> = cursor.try_collect().await?;
        Ok(items)
    }

    pub async fn update(&self, data: &WheelItemUpdate) -> Result<Option<WheelItem>> {
        let mut update_doc = doc! {};

        if let Some(ref name) = data.name {
            update_doc.insert("name", name);
        }

        if let Some(weight) = data.weight {
            update_doc.insert("weight", weight as i32);
        }

        let query_options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        self.coll
            .find_one_and_update(doc! { "_id": &data.id }, doc! { "$set": update_doc })
            .with_options(query_options)
            .await
    }

    pub async fn random(&self) -> Result<Option<SpinResult>> {
        let items = self.get_all().await?;
        if items.is_empty() || items.len() < 2 {
            return Ok(None);
        }

        let total_weight: u16 = items.iter().map(|i| i.weight).sum();
        if total_weight == 0 {
            return Ok(None);
        }

        let mut rng = rand::rng();
        let mut random_target: u16 = rng.random_range(1..=total_weight);

        for (index, item) in items.iter().enumerate() {
            if random_target <= item.weight {
                return Ok(Some(SpinResult {
                    winning_item: item.clone(),
                    winning_index: index,
                }));
            }
            random_target -= item.weight;
        }

        // Fallback safety (jika terjadi rounding float/edge case)
        let last_idx = items.len() - 1;
        Ok(Some(SpinResult {
            winning_item: items[last_idx].clone(),
            winning_index: last_idx,
        }))
    }
}
