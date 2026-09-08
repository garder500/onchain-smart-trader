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

    #[test]
    fn test_decode_sync() {
        let pool = "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc";
        let sync_topic: alloy::primitives::B256 =
            "0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1"
                .parse()
                .unwrap();
        // 8808186452633 (0x80252194699) and 4667525694721402386667 (0x2547b7fe9579ff10eb)
        let r0_bytes =
            hex::decode("0000000000000000000000000000000000000000000000000000080252194699")
                .unwrap();
        let r1_bytes =
            hex::decode("00000000000000000000000000000000000000000000002547b7fe9579ff10eb")
                .unwrap();
        let mut data = Vec::new();
        data.extend_from_slice(&r0_bytes);
        data.extend_from_slice(&r1_bytes);

        let res = EventDecoder::decode_sync(pool, &[sync_topic], &data);
        assert!(res.is_some());
        let (r0, r1) = res.unwrap();
        assert_eq!(r0, 8806060344985);
        assert_eq!(r1, 687697409742634684651);
    }

    #[test]
    fn test_decode_swap_v2_real_log() {
        let pool = "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc";
        let topics: Vec<alloy::primitives::B256> = vec![
            "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822"
                .parse()
                .unwrap(),
            "0x0000000000000000000000001111111254eeb25477b68fb85ed929f73a960582"
                .parse()
                .unwrap(),
            "0x000000000000000000000000663dc15d3c1ac63ff12e45ab68fea3f0a883c251"
                .parse()
                .unwrap(),
        ];
        let data = hex::decode(
            "0000000000000000000000000000000000000000000000000000000000000000\
             0000000000000000000000000000000000000000000000000002053b45529d20\
             00000000000000000000000000000000000000000000000000000000001053a6\
             0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();

        let res = EventDecoder::decode_swap_v2(
            pool,
            "0x1af6f88b3625721fccb5cbeb7efc0d4d819260c0a936bb170566cd968f0948bd",
            25750014,
            1208,
            &topics,
            &data,
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            rust_decimal::Decimal::from(1881),
            chrono::Utc::now(),
        );

        assert!(res.is_some());
        let swap = res.unwrap();
        assert_eq!(swap.block_number, 25750014);
        assert_eq!(swap.log_index, 1208);
        assert_eq!(swap.amount_in.to_string(), "568702077672736");
        assert_eq!(swap.amount_out.to_string(), "1069990");
    }
}
