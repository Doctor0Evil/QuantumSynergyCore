use anyhow::Result;
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use redis::AsyncCommands;

pub type RedisPool = Pool<RedisConnectionManager>;

pub async fn init_redis_pool(url: &str) -> Result<RedisPool> {
    let mgr = RedisConnectionManager::new(url)?;
    let pool = Pool::builder().max_size(16).build(mgr).await?;
    Ok(pool)
}

pub async fn cache_txn_status(
    pool: &RedisPool,
    txn_id: &str,
    status: &str,
    ttl_secs: usize,
) -> Result<()> {
    let mut conn = pool.get().await?;
    conn.set_ex(format!("txn:{txn_id}"), status, ttl_secs).await?;
    Ok(())
}
