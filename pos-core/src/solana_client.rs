use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, system_instruction, transaction::Transaction};

pub struct SolanaClient {
    pub rpc: RpcClient,
}

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc: RpcClient::new(rpc_url.to_string()),
        }
    }

    pub async fn transfer(
        &self,
        from: &Keypair,
        to: &Pubkey,
        lamports: u64,
    ) -> Result<()> {
        let recent_blockhash = self.rpc.get_latest_blockhash().await?;
        let ix = system_instruction::transfer(&from.pubkey(), to, lamports);
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&from.pubkey()),
            &[from],
            recent_blockhash,
        );
        self.rpc.send_and_confirm_transaction(&tx).await?;
        Ok(())
    }
}
