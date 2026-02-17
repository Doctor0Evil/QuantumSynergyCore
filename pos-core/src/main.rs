mod config;
mod db;
mod kafka;
mod redis_cache;
mod web3_client;
mod solana_client;
mod router;
mod metrics;
mod models;
mod treasury;
mod aln_parser;
mod errors;

use crate::config::AppConfig;
use crate::db::init_pg_pool;
use crate::kafka::KafkaProducer;
use crate::metrics::init_metrics;
use crate::redis_cache::init_redis_pool;
use crate::web3_client::Web3Clients;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::signal;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let cfg = AppConfig::from_env().expect("config load failed");
    let pg_pool = init_pg_pool(&cfg.pg_dsn).await.expect("pg init failed");
    let redis_pool = init_redis_pool(&cfg.redis_url).await.expect("redis init failed");
    let kafka_producer = KafkaProducer::new(&cfg.kafka_brokers, &cfg.kafka_topic)
        .expect("kafka init failed");
    let web3_clients = Web3Clients::new(&cfg).await.expect("web3 init failed");

    init_metrics();

    let app_state = router::AppState::new(
        cfg.clone(),
        pg_pool,
        redis_pool,
        kafka_producer,
        web3_clients,
    );

    let app = Router::new()
        .route("/health", get(router::health))
        .route("/metrics", get(metrics::metrics_handler))
        .route("/pos/txn", axum::routing::post(router::pos_txn))
        .route("/pos/init", axum::routing::post(router::pos_init))
        .route("/dao/allocate", axum::routing::post(router::dao_allocate))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.http_port));
    info!("ALN-POS listening on {}", addr);

    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    let graceful = server.with_graceful_shutdown(shutdown_signal());
    if let Err(e) = graceful.await {
        error!("server error: {:?}", e);
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("ctrl_c");
    };
    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("sigterm");
        sigterm.recv().await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
