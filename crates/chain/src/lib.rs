pub mod client;
pub mod decoder;

pub use client::{Erc20TokenMetadata, EvmClient};
pub use decoder::EventDecoder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytecode_analyzer() {
        // Mock bytecode with 0x40c10f19 (mint) and 0x8456cb59 (pause)
        let malicious_code =
            hex::decode("608060405234801561001057600080fd5b5040c10f198456cb59").unwrap();
        let (has_mint, has_pause) = EventDecoder::analyze_bytecode(&malicious_code);
        assert!(has_mint);
        assert!(has_pause);

        let safe_code = hex::decode("608060405234801561001057600080fd5b5000").unwrap();
        let (has_mint_safe, has_pause_safe) = EventDecoder::analyze_bytecode(&safe_code);
        assert!(!has_mint_safe);
        assert!(!has_pause_safe);
    }
}
