use crate::config::AppConfig;
use anyhow::Result;
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionRequest, U256};
use std::sync::Arc;

pub struct Web3Clients {
    pub optimistic: OptimisticClient,
}

pub struct OptimisticClient {
    pub provider: Arc<Provider<Http>>,
    pub sgc_contract: Contract<Provider<Http>>,
}

impl Web3Clients {
    pub async fn new(cfg: &AppConfig) -> Result<Self> {
        let provider = Provider::<Http>::try_from(cfg.optimistic_rpc.as_str())?;
        let provider = Arc::new(provider);

        let abi: Abi = serde_json::from_str(
            r#"
            [
              {
                "name": "transfer",
                "type": "function",
                "stateMutability": "nonpayable",
                "inputs": [
                  { "name": "to", "type": "address" },
                  { "name": "amount", "type": "uint256" }
                ],
                "outputs": [
                  { "name": "ok", "type": "bool" }
                ]
              }
            ]
            "#,
        )?;

        let addr: Address = cfg.optimistic_sgc_address.parse()?;
        let sgc_contract = Contract::new(addr, abi, provider.clone());

        Ok(Self {
            optimistic: OptimisticClient {
                provider,
                sgc_contract,
            },
        })
    }
}

impl OptimisticClient {
    pub async fn transfer_sgc(
        &self,
        from_wallet: &LocalWallet,
        to: Address,
        amount: U256,
    ) -> Result<()> {
        let client = self
            .provider
            .clone()
            .with_signer(from_wallet.clone());
        let contract = Contract::new(self.sgc_contract.address(), self.sgc_contract.abi().clone(), client);
        let call = contract.method::<(Address, U256), bool>("transfer", (to, amount))?;
        let _receipt = call.send().await?;
        Ok(())
    }
}
