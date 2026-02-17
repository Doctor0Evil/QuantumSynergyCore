use crate::aln_serializer::{build_pos_init_command, build_pos_txn_command};
use crate::config::CliConfig;
use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::Serialize;

#[derive(Parser)]
#[command(name = "aln-pos", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    PosInit {
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        wallet_address: String,
    },
    PosTxn {
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        recipient: String,
        #[arg(long)]
        amount: f64,
        #[arg(long)]
        blockchain: String,
    },
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        let cfg = CliConfig::from_env()?;
        let client = Client::new();
        match self.cmd {
            Commands::PosInit {
                user_id,
                wallet_address,
            } => {
                let cmd = build_pos_init_command();
                let body = PosInitBody {
                    user_id,
                    wallet_address,
                };
                let url = format!("{}/pos/init", cfg.pos_endpoint);
                let resp = client.post(url).json(&body).send().await?;
                let txt = resp.text().await?;
                println!("CMD: {}", cmd);
                println!("RESP: {}", txt);
            }
            Commands::PosTxn {
                user_id,
                recipient,
                amount,
                blockchain,
            } => {
                let cmd = build_pos_txn_command(amount);
                let body = PosTxnBody {
                    user_id,
                    recipient,
                    amount,
                    currency: "SGC".into(),
                    blockchain,
                };
                let url = format!("{}/pos/txn", cfg.pos_endpoint);
                let resp = client.post(url).json(&body).send().await?;
                let txt = resp.text().await?;
                println!("CMD: {}", cmd);
                println!("RESP: {}", txt);
            }
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct PosInitBody {
    user_id: String,
    wallet_address: String,
}

#[derive(Serialize)]
struct PosTxnBody {
    user_id: String,
    recipient: String,
    amount: f64,
    currency: String,
    blockchain: String,
}
