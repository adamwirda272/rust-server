use rand::Rng;
use crate::api::v1::lucky_wheel::models::wheel::{SpinResult, WheelItem};


pub fn pick_winning_item(items: &[WheelItem]) -> Option<SpinResult> {
    if items.is_empty() {
        return None;
    }

    let total_weight: u32 = items.iter().map(|item| item.weight).sum();
    if total_weight == 0 {
        return None;
    }

    let mut rng = rand::thread_rng();
    let mut random_weight = rng.gen_range(0..total_weight);

    for (index, item) in items.iter().enumerate() {
        if random_weight < item.weight {
            return Some(SpinResult {
                winning_item: item.clone(),
                winning_index: index,
            });
        }
        random_weight -= item.weight;
    }

    None
}