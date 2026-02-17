use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PosTxnRequest {
    pub user_id: String,
    pub amount: f64,
    pub currency: String,
    pub blockchain: String,
    pub recipient: String,
}

#[derive(Serialize, Deserialize)]
pub struct PosInitRequest {
    pub user_id: String,
    pub wallet_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct DaoAllocateRequest {
    pub proposer: String,
    pub amount: f64,
    pub asset: String,
}

#[derive(Serialize, Deserialize)]
pub struct PosTxnRecord {
    pub transaction_id: Uuid,
    pub user_id: String,
    pub amount: f64,
    pub currency: String,
    pub blockchain: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
}
