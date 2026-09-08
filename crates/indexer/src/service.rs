use crate::db::Database;
use crate::processor::BlockProcessor;
use anyhow::Result;
use chain::EvmClient;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub struct IndexerService {
    client: Arc<EvmClient>,
    processor: Arc<BlockProcessor>,
    db: Database,
    checkpoint_id: String,
    poll_interval: Duration,
}

impl IndexerService {
    pub fn new(client: Arc<EvmClient>, db: Database) -> Self {
        let processor = Arc::new(BlockProcessor::new(db.clone(), Some(client.clone())));
        Self {
            client,
            processor,
            db,
            checkpoint_id: "indexer_evm".to_string(),
            poll_interval: Duration::from_secs(4),
        }
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting EVM Indexer Service loop");

        let mut current_block = match self.db.get_checkpoint(&self.checkpoint_id).await? {
            Some(cp) => cp + 1,
            None => {
                let latest = self.client.get_latest_block_number().await.unwrap_or(0);
                info!(
                    latest_block = latest,
                    "No checkpoint found, starting at latest block"
                );
                latest
            }
        };

        loop {
            match self.client.get_latest_block_number().await {
                Ok(latest_on_chain) => {
                    if current_block <= latest_on_chain {
                        info!(
                            current_block = current_block,
                            latest_on_chain = latest_on_chain,
                            "Fetching and processing block"
                        );

                        match self.client.get_block_by_number(current_block).await {
                            Ok(Some(block)) => {
                                if let Err(e) = self.processor.process_block(&block).await {
                                    error!(block = current_block, error = %e, "Failed to process block");
                                    sleep(self.poll_interval).await;
                                    continue;
                                }
                                current_block += 1;
                            }
                            Ok(None) => {
                                warn!(block = current_block, "Block not yet available from RPC");
                                sleep(self.poll_interval).await;
                            }
                            Err(e) => {
                                error!(block = current_block, error = %e, "Error fetching block from RPC");
                                sleep(self.poll_interval).await;
                            }
                        }
                    } else {
                        // Caught up to latest chain head
                        sleep(self.poll_interval).await;
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to get latest block number from RPC");
                    sleep(self.poll_interval).await;
                }
            }
        }
    }
}
