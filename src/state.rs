use cosmwasm_std::{Coin, HumanAddr, Storage};
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static NAMESPACE_ORDER_ASK: &[u8] = b"ask";
pub static NAMESPACE_ORDER_BID: &[u8] = b"bid";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct AskOrder {
    pub asset: Vec<Coin>,
    pub id: String,
    pub owner: HumanAddr,
    pub price: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct BidOrder {
    pub asset: Vec<Coin>,
    pub id: String,
    pub owner: HumanAddr,
    pub price: Vec<Coin>,
}

pub fn get_ask_storage(storage: &mut dyn Storage) -> Bucket<AskOrder> {
    bucket(storage, NAMESPACE_ORDER_ASK)
}

pub fn get_ask_storage_read(storage: &dyn Storage) -> ReadonlyBucket<AskOrder> {
    bucket_read(storage, NAMESPACE_ORDER_ASK)
}
pub fn get_bid_storage(storage: &mut dyn Storage) -> Bucket<BidOrder> {
    bucket(storage, NAMESPACE_ORDER_BID)
}

pub fn get_bid_storage_read(storage: &dyn Storage) -> ReadonlyBucket<BidOrder> {
    bucket_read(storage, NAMESPACE_ORDER_BID)
}