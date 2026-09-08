use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::eth::{Block, BlockId, BlockNumberOrTag};
use anyhow::{Context, Result};
use std::str::FromStr;
use tracing::info;

pub struct Erc20TokenMetadata {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub total_supply: rust_decimal::Decimal,
}

#[derive(Clone)]
pub struct EvmClient {
    rpc_url: String,
}

impl EvmClient {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let _url: alloy::transports::http::reqwest::Url =
            rpc_url.parse().context("Invalid RPC HTTP URL")?;
        info!(rpc_url = %rpc_url, "Initialized EVM RPC client");
        Ok(Self {
            rpc_url: rpc_url.to_string(),
        })
    }

    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    fn provider(&self) -> Result<impl Provider> {
        let url = self.rpc_url.parse().context("Invalid RPC HTTP URL")?;
        Ok(ProviderBuilder::new().connect_http(url))
    }

    pub async fn get_latest_block_number(&self) -> Result<u64> {
        let provider = self.provider()?;
        let block_num = provider.get_block_number().await?;
        Ok(block_num)
    }

    pub async fn get_block_by_number(&self, block_number: u64) -> Result<Option<Block>> {
        let provider = self.provider()?;
        let block = provider
            .get_block(BlockId::Number(BlockNumberOrTag::Number(block_number)))
            .full()
            .await?;
        Ok(block)
    }

    pub async fn get_code(&self, address: &str) -> Result<Vec<u8>> {
        let provider = self.provider()?;
        let addr = Address::from_str(address).context("Invalid address for get_code")?;
        let bytes = provider.get_code_at(addr).await?;
        Ok(bytes.to_vec())
    }

    pub async fn is_contract(&self, address: &str) -> Result<bool> {
        let code = self.get_code(address).await?;
        Ok(!code.is_empty())
    }
}
