use crate::db::PgPool;
use anyhow::Result;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

pub async fn allocate_rewards(
    pool: &PgPool,
    proposer: &str,
    total_amount: f64,
) -> Result<()> {
    let client = pool.get().await?;
    let proposal_id = Uuid::new_v4();
    let asset = "SGC";
    let status = "PENDING";
    let stmt = client
        .prepare(
            "INSERT INTO dao_treasury (proposal_id, proposer, amount, asset, status)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .await?;
    let params: &[&(dyn ToSql + Sync)] = &[
        &proposal_id,
        &proposer,
        &total_amount,
        &asset,
        &status,
    ];
    client.execute(&stmt, params).await?;
    Ok(())
}
