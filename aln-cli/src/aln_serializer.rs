use rand::{distributions::Alphanumeric, Rng};

pub fn build_pos_init_command() -> String {
    let session: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    format!("POS_INIT_{}", session)
}

pub fn build_pos_txn_command(amount: f64) -> String {
    let session: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    format!("POS_TXN_{}_{}_SGC", session, amount)
}
