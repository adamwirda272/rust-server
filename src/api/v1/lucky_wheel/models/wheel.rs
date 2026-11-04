use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelItem {
    pub id: String,
    pub name: String,
    pub weight: u32,
    pub is_grand_prize: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinResult {
    pub winning_item: WheelItem,
    pub winning_index: usize,
}
