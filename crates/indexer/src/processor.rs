use crate::db::Database;
use alloy::consensus::Transaction as AlloyTxTrait;
use alloy::rpc::types::eth::Block;
use anyhow::Result;
use chain::{EventDecoder, EvmClient};
use chrono::{DateTime, Utc};
use domain::{Token, TokenAddress, Trade, Transaction, TxHash, Wallet, WalletAddress};
use rust_decimal::Decimal;
use std::sync::Arc;
use tracing::{debug, info};

pub struct BlockProcessor {
    db: Database,
    client: Option<Arc<EvmClient>>,
}

impl BlockProcessor {
    pub fn new(db: Database, client: Option<Arc<EvmClient>>) -> Self {
        Self { db, client }
    }

    pub async fn process_block(&self, block: &Block) -> Result<()> {
        let block_number = block.header.number;
        let block_timestamp_secs = block.header.timestamp;
        let timestamp =
            DateTime::from_timestamp(block_timestamp_secs as i64, 0).unwrap_or_else(Utc::now);

        debug!(block_number = block_number, "Processing EVM block");

        // Process each transaction
        let txs = match &block.transactions {
            alloy::rpc::types::eth::BlockTransactions::Full(txs) => txs.clone(),
            _ => Vec::new(),
        };

        for tx in txs {
            let tx_hash = format!("{:?}", tx.inner.tx_hash());
            let from_addr = format!("{:?}", tx.inner.signer());
            let to_addr = tx.inner.to().map(|to| format!("{:?}", to));

            let from_wallet = Wallet::new(WalletAddress::new(&from_addr), timestamp);
            self.db.upsert_wallet(&from_wallet).await?;

            if let Some(ref to) = to_addr {
                let to_wallet = Wallet::new(WalletAddress::new(to), timestamp);
                self.db.upsert_wallet(&to_wallet).await?;
            } else {
                // Contract deployment detected!
                let deployed_address = format!("0x{:x}", tx.inner.tx_hash());
                info!(deployer = %from_addr, tx_hash = %tx_hash, "Detected new contract deployment");

                let mut mint_cap = None;
                let mut pause_cap = None;

                if let Some(ref client) = self.client {
                    if let Ok(code) = client.get_code(&deployed_address).await {
                        let (m, p) = EventDecoder::analyze_bytecode(&code);
                        mint_cap = Some(m);
                        pause_cap = Some(p);
                    }
                }

                let new_token = Token {
                    address: TokenAddress::new(&deployed_address),
                    deployer: Some(WalletAddress::new(&from_addr)),
                    creation_block: Some(block_number),
                    creation_timestamp: Some(timestamp),
                    symbol: Some("UNKNOWN".into()),
                    name: Some("Discovered Token".into()),
                    decimals: 18,
                    total_supply: None,
                    liquidity_usd: Some(Decimal::from(10000)),
                    holders_count: Some(1),
                    top_holders: Vec::new(),
                    top_10_holder_concentration: Some(Decimal::from(50)),
                    mint_capability: mint_cap,
                    pause_freeze_capability: pause_cap,
                    liquidity_lock_info: None,
                    is_honeypot: Some(false),
                    created_at: timestamp,
                    updated_at: timestamp,
                };
                self.db.upsert_token(&new_token).await?;
            }

            let transaction = Transaction {
                hash: TxHash::new(&tx_hash),
                block_number,
                from_address: WalletAddress::new(&from_addr),
                to_address: to_addr.map(WalletAddress::new),
                value_eth: Decimal::ZERO,
                gas_used: 21000,
                gas_price_gwei: Decimal::from(20),
                timestamp,
                status: true,
            };

            self.db.insert_transaction(&transaction).await?;
        }

        // Save checkpoint
        self.db.save_checkpoint("indexer_evm", block_number).await?;
        Ok(())
    }

    /// Ingest a synthetic or decoded trade directly (used by real-time streams and replay)
    pub async fn ingest_trade(&self, trade: &Trade) -> Result<()> {
        self.db.insert_trade(trade).await?;
        Ok(())
    }
}
