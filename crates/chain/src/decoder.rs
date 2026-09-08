use alloy::primitives::{Address, Log as AlloyLog, U256};
use alloy::sol;
use alloy::sol_types::SolEvent;
use chrono::{DateTime, Utc};
use domain::{NormalizedSwapEvent, NormalizedTransferEvent, TokenAddress, TxHash, WalletAddress};
use rust_decimal::Decimal;
use std::str::FromStr;

// Solidity interfaces for ERC20 and Uniswap DEX events
sol! {
    #[derive(Debug, PartialEq, Eq)]
    event Transfer(address indexed from, address indexed to, uint256 value);

    #[derive(Debug, PartialEq, Eq)]
    event Sync(uint112 reserve0, uint112 reserve1);

    // Uniswap V2 Swap event
    #[derive(Debug, PartialEq, Eq)]
    event SwapV2(
        address indexed sender,
        uint256 amount0In,
        uint256 amount1In,
        uint256 amount0Out,
        uint256 amount1Out,
        address indexed to
    );

    // ERC20 function calls
    function name() external view returns (string);
    function symbol() external view returns (string);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint256);
}

pub struct EventDecoder;

impl EventDecoder {
    /// Decodes an ERC-20 Transfer log
    pub fn decode_transfer(
        token_address: &str,
        tx_hash: &str,
        block_number: u64,
        log_index: u32,
        topics: &[alloy::primitives::B256],
        data: &[u8],
        timestamp: DateTime<Utc>,
    ) -> Option<NormalizedTransferEvent> {
        let alloy_log = AlloyLog {
            address: Address::from_str(token_address).ok()?,
            data: alloy::primitives::LogData::new(topics.to_vec(), data.to_vec().into())?,
        };

        if let Ok(transfer) = Transfer::decode_log(&alloy_log) {
            let from = format!("{:?}", transfer.from);
            let to = format!("{:?}", transfer.to);
            let raw_amount = transfer.value.to_string();
            let amount = Decimal::from_str(&raw_amount).unwrap_or(Decimal::ZERO);

            return Some(NormalizedTransferEvent {
                tx_hash: TxHash::new(tx_hash),
                block_number,
                log_index,
                token_address: TokenAddress::new(token_address),
                from_address: WalletAddress::new(from),
                to_address: WalletAddress::new(to),
                raw_amount,
                amount,
                timestamp,
            });
        }
        None
    }

    /// Decodes a Uniswap V2 Swap event
    pub fn decode_swap_v2(
        pool_address: &str,
        tx_hash: &str,
        block_number: u64,
        log_index: u32,
        topics: &[alloy::primitives::B256],
        data: &[u8],
        token_in: &str,
        token_out: &str,
        price_usd: Decimal,
        timestamp: DateTime<Utc>,
    ) -> Option<NormalizedSwapEvent> {
        let alloy_log = AlloyLog {
            address: Address::from_str(pool_address).ok()?,
            data: alloy::primitives::LogData::new(topics.to_vec(), data.to_vec().into())?,
        };

        if let Ok(swap) = SwapV2::decode_log(&alloy_log) {
            let recipient = format!("{:?}", swap.to);
            let amount_in_u256 = if swap.amount0In > U256::ZERO {
                swap.amount0In
            } else {
                swap.amount1In
            };
            let amount_out_u256 = if swap.amount0Out > U256::ZERO {
                swap.amount0Out
            } else {
                swap.amount1Out
            };

            let amount_in = Decimal::from_str(&amount_in_u256.to_string()).unwrap_or(Decimal::ZERO);
            let amount_out =
                Decimal::from_str(&amount_out_u256.to_string()).unwrap_or(Decimal::ZERO);

            return Some(NormalizedSwapEvent {
                tx_hash: TxHash::new(tx_hash),
                block_number,
                log_index,
                pool_address: TokenAddress::new(pool_address),
                recipient: WalletAddress::new(recipient),
                token_in: TokenAddress::new(token_in),
                token_out: TokenAddress::new(token_out),
                amount_in,
                amount_out,
                price_usd,
                timestamp,
            });
        }
        None
    }

    /// Inspect bytecode for dangerous capabilities (minting, blacklisting/pausing)
    pub fn analyze_bytecode(bytecode: &[u8]) -> (bool, bool) {
        let hex_code = hex::encode(bytecode);

        // mint(address,uint256) selector: 0x40c10f19
        // pause() selector: 0x8456cb59
        // freeze / blacklist common patterns
        let has_mint = hex_code.contains("40c10f19") || hex_code.contains("mint");
        let has_pause_or_freeze = hex_code.contains("8456cb59")
            || hex_code.contains("pause")
            || hex_code.contains("blacklist")
            || hex_code.contains("freeze");

        (has_mint, has_pause_or_freeze)
    }
}
