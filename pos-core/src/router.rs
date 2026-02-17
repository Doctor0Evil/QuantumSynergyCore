use crate::aln_parser::{parse_command, AlnCommand};
use crate::db::PgPool;
use crate::errors::AppError;
use crate::kafka::KafkaProducer;
use crate::metrics::{SUCCESS_RATE, TXN_THROUGHPUT};
use crate::models::{DaoAllocateRequest, PosInitRequest, PosTxnRecord, PosTxnRequest};
use crate::redis_cache::{cache_txn_status, RedisPool};
use crate::treasury::allocate_rewards;
use crate::web3_client::Web3Clients;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub cfg: crate::config::AppConfig,
    pub pg_pool: PgPool,
    pub redis_pool: RedisPool,
    pub kafka: KafkaProducer,
    pub web3: Web3Clients,
}

impl AppState {
    pub fn new(
        cfg: crate::config::AppConfig,
        pg_pool: PgPool,
        redis_pool: RedisPool,
        kafka: KafkaProducer,
        web3: Web3Clients,
    ) -> Self {
        Self {
            cfg,
            pg_pool,
            redis_pool,
            kafka,
            web3,
        }
    }
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn pos_init(
    State(state): State<AppState>,
    Json(req): Json<PosInitRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = format!("POS_INIT_{}", generate_session_id());
    match parse_command(&command) {
        AlnCommand::PosInit { .. } => Ok((
            StatusCode::OK,
            Json(json!({ "session": command, "wallet": req.wallet_address })),
        )),
        _ => Err(AppError::Validation("invalid POS_INIT".into())),
    }
}

pub async fn pos_txn(
    State(state): State<AppState>,
    Json(req): Json<PosTxnRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.currency != "SGC" {
        return Err(AppError::Validation("unsupported currency".into()));
    }

    let txn_id = Uuid::new_v4();
    let now = Utc::now();

    {
        let client = state.pg_pool.get().await.map_err(|e| AppError::Db(e.to_string()))?;
        let stmt = client
            .prepare(
                "INSERT INTO pos_transactions (transaction_id, user_id, amount, currency, blockchain, status)
                 VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .await
            .map_err(|e| AppError::Db(e.to_string()))?;
        client
            .execute(
                &stmt,
                &[
                    &txn_id,
                    &req.user_id,
                    &req.amount,
                    &req.currency,
                    &req.blockchain,
                    &"PENDING",
                ],
            )
            .await
            .map_err(|e| AppError::Db(e.to_string()))?;
    }

    TXN_THROUGHPUT.inc();

    state
        .kafka
        .send(
            &txn_id.to_string(),
            &json!({
                "transaction_id": txn_id,
                "user_id": req.user_id,
                "amount": req.amount,
                "currency": req.currency,
                "blockchain": req.blockchain
            }),
        )
        .await
        .map_err(|e| AppError::Internal)?;

    cache_txn_status(&state.redis_pool, &txn_id.to_string(), "PENDING", 24 * 3600)
        .await
        .map_err(|e| AppError::Internal)?;

    SUCCESS_RATE.set(100);

    Ok((
        StatusCode::ACCEPTED,
        Json(PosTxnRecord {
            transaction_id: txn_id,
            user_id: req.user_id,
            amount: req.amount,
            currency: req.currency,
            blockchain: req.blockchain,
            timestamp: now,
            status: "PENDING".into(),
        }),
    ))
}

pub async fn dao_allocate(
    State(state): State<AppState>,
    Json(req): Json<DaoAllocateRequest>,
) -> Result<impl IntoResponse, AppError> {
    allocate_rewards(&state.pg_pool, &req.proposer, req.amount)
        .await
        .map_err(|e| AppError::Db(e.to_string()))?;
    Ok((StatusCode::OK, Json(json!({ "status": "QUEUED" }))))
}

fn generate_session_id() -> String {
    let id = uuid::Uuid::new_v4()
        .to_simple()
        .to_string()
        .to_uppercase();
    id.chars().take(8).collect()
}
