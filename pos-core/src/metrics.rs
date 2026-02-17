use axum::{response::IntoResponse, Json};
use lazy_static::lazy_static;
use prometheus::{
    Encoder, IntCounter, IntGauge, Opts, Registry, TextEncoder,
};
use serde::Serialize;
use std::sync::Arc;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    pub static ref TXN_THROUGHPUT: IntCounter = IntCounter::new(
        "qsc_pos_txn_throughput",
        "Total number of processed POS transactions"
    )
    .unwrap();
    pub static ref GAS_USAGE: IntGauge =
        IntGauge::with_opts(Opts::new("qsc_pos_gas_usage", "Last gas usage value")).unwrap();
    pub static ref SUCCESS_RATE: IntGauge = IntGauge::with_opts(Opts::new(
        "qsc_pos_success_rate",
        "Success rate percentage of POS transactions"
    ))
    .unwrap();
}

pub fn init_metrics() {
    REGISTRY
        .register(Box::new(TXN_THROUGHPUT.clone()))
        .ok();
    REGISTRY.register(Box::new(GAS_USAGE.clone())).ok();
    REGISTRY
        .register(Box::new(SUCCESS_RATE.clone()))
        .ok();
}

#[derive(Serialize)]
struct MetricsInfo {
    status: String,
}

pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    if encoder.encode(&metric_families, &mut buffer).is_ok() {
        let body = String::from_utf8(buffer).unwrap_or_default();
        return axum::response::Response::builder()
            .header("Content-Type", encoder.format_type())
            .body(body)
            .unwrap();
    }
    let info = MetricsInfo {
        status: "metrics encode error".into(),
    };
    Json(info).into_response()
}
