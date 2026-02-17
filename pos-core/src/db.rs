use anyhow::Result;
use deadpool-postgres::{Manager, Pool};
use tokio_postgres::NoTls;

pub type PgPool = Pool;

pub async fn init_pg_pool(dsn: &str) -> Result<PgPool> {
    let mgr = Manager::new(dsn.parse()?, NoTls);
    let pool = Pool::builder(mgr).max_size(16).build()?;
    {
        let client = pool.get().await?;
        client
            .batch_execute(
                r#"
                CREATE TABLE IF NOT EXISTS pos_transactions (
                  transaction_id UUID PRIMARY KEY,
                  user_id VARCHAR(50) NOT NULL,
                  amount FLOAT NOT NULL,
                  currency VARCHAR(10) NOT NULL,
                  blockchain VARCHAR(50) NOT NULL,
                  timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                  status VARCHAR(20) NOT NULL
                );
                CREATE TABLE IF NOT EXISTS dao_treasury (
                  proposal_id UUID PRIMARY KEY,
                  proposer VARCHAR(50) NOT NULL,
                  amount FLOAT NOT NULL,
                  asset VARCHAR(10) NOT NULL,
                  status VARCHAR(20) NOT NULL,
                  timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                );
                CREATE TABLE IF NOT EXISTS sgc_transactions (
                  transaction_id UUID PRIMARY KEY,
                  user_id VARCHAR(50) NOT NULL,
                  amount FLOAT NOT NULL,
                  type VARCHAR(20) NOT NULL,
                  blockchain VARCHAR(50) NOT NULL,
                  timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                );
                CREATE TABLE IF NOT EXISTS syntax_log (
                  id BIGSERIAL PRIMARY KEY,
                  raw_command TEXT NOT NULL,
                  matched_pattern TEXT NOT NULL,
                  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                );
                "#,
            )
            .await?;
    }
    Ok(pool)
}
