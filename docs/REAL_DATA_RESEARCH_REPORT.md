# Research Experiment Report: EXP-2026-355219

> [!IMPORTANT]
> **DATA SOURCE**: `REAL` | **VERDICT STATUS**: `EDGE_NOT_COPIABLE`
> **NOTICE**: Empirical on-chain event evaluation.

---

## 1. Experiment Metadata & Provenance

- **Experiment ID**: `EXP-2026-355219`
- **Git Commit**: `0.1.0`
- **Dataset SHA-256**: `913ea66396fc0cceb2266728a0cab0f8e674dfbfebfdbb36c2e27811e949f48c`
- **Evaluation Timestamp**: `2026-09-08 02:37:30.430114919 UTC`
- **Time Window**: `2026-08-07 23:55:23 UTC -> 2026-09-07 01:58:35 UTC`
- **Total Trades Evaluated**: `846`
- **Train-Selected Wallets Count**: `3`

## 2. Scientific Verdict & Executive Summary

> **Verdict Status**: `EDGE_NOT_COPIABLE`
> **Conclusion**: EDGE NOT COPIABLE: Alpha exists in theoretical zero-latency terms but is eliminated under realistic latency (break-even: 1s).

| Hypothesis / Condition | Result | Assessment |
|---|---|---|
| Statistical Significance ($p < 0.05$) | `true` | PASS |
| Latency Copiability ($d \ge 2\text{s}$) | `false` | DEGRADED / NEGATIVE |
| Max Scalable Capital | `$0` | Illiquid / Unscalable |
| Break-Even Latency | `1s` | Alpha drops to zero after 1s delay |

## 3. Wallet Behavioral Taxonomy & Persistence

| Wallet Address | Cluster | Persistence Score | Copiable? | Selected in Train? | Notes |
|---|---|---|---|---|---|
| `0x8567b56b8f4d8faf369990cb060c7a9d164d377d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4df189617c1d903069bcf0f6812b273132b12ac6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x107971281cacd18204689e84c028b0fde2e1fc41` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8816d3ca88bf7777931189377146bca2b0f78e2c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x790f117af8169487725a1c3e92828176042cfc88` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x09d1d767edf8fa23a64c51fa559e0688e526812f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x867bdc57d1b071fe5a9f670dd70b91e4269814d3` | `SWING_TRADER` | `0` | Yes | No | Swing trader: long holding time of 505.9 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x0ff9f59d6d72a4421c745aeb3d792c96b7a5a8cf` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbc7ce7b6b5437d7d715fbb1cc7b4ec12399c5516` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1316f07d825a66bcb9250637bdfc18b99b2bd322` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x604d23a2ebb002df6fdc77b82da716222ca43b8b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5a4f527ced9311264fa51819c4949518f771408f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6487986a78b126538746937fbd25516971129b3a` | `HIGH_RISK_DEGEN` | `0.3` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x06ce4330a9acbb5fcd527188ad9dbf96a1a90ea0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd6f4567bfa29bc70b26dccaeb07e5868905d667c` | `SWING_TRADER` | `0.3` | Yes | No | Swing trader: long holding time of 417.6 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0xa2b04f8133fc25887a436812eae384e32a8a84f2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x50d3f135681304feef9f80d1d03404e2a0707e82` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd2700e7bbe0a8af6d6c5b3f61d27855daa425a53` | `HIGH_RISK_DEGEN` | `0.05` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1f2f10d1c40777ae1da742455c65828ff36df387` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2e81ec0b8b4022fac83a21b2f2b4b8f5ed744d70` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1dc10dc4bade89ccacd97dff0805a8b6ac5a576f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6f858f383842c887a82d30b392cfc686938413ec` | `HIGH_RISK_DEGEN` | `0.3` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x892715b1450f90b35ab96a969b83af8608bd5163` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xde27419fe44e204dbedd27fde1cdeaf21198e157` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbf5224320f8ec13548f4af46d46a717a1340e1b2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfdaf1f1714810f8d88a57c9d551d442c68ace2bb` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x08ce084b6f1835f61fac196f7740c2c745f426d5` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xe38fd1f9fee6a6111513ea21fbf2e0f843f4ef32` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x606fead1d6dca36f749282c29be8524b61e7445d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4a07f501d5f8b6447ef7add558207f20c79eeb4f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf5c299316699131d29adcb7ef87af8e97bbc7ead` | `SWING_TRADER` | `0.3` | Yes | No | Swing trader: long holding time of 423.3 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x9a84a1852bc7fb608794960960adb04666a12b41` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfa8461e81b9c3f099d4deddbeeed6b119d4990e1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xebeb7f55d53375c484244cd3c0ce6f22b5b760f0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x7abfe03949efa657e717f16d7eddc44ee94d702e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x624ec1cb8dfcb0e8d2795ff801f2cda618402db0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc52bd1d364ff11c03592d1cb701d2fd7140a475e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3136aa2a7bf0958968f437b66e00cd26fdce57ac` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1379afdb21e304c0d3c810273a81f4fce420aa68` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xff5910ab899d4d8ca4ff3b99f93a62bddbeda77e` | `HIGH_RISK_DEGEN` | `0.20` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x49a782c2660e77dded753bb88ac57619298ad50e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x113c1c2a6317a4ac0a61cd3a289776e08f9cd145` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x73207b20ae14892f8567e54b66b32645ef5ebfcf` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x93b48a3ca3bb08a4ce05f04bcac4b9a39c44c2b8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc6b9c3fa037303d10d50d71ae3cedb0eea7a1c78` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbdcc23c7e20d9f6af9b119e8c13191f707128325` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8f10b468b06c6fd214b65f87778827f7d113f996` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x33af6966f9bfd9f25085a892d8893323b62efd44` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb5765d79babbd12d5826c3f0d90a6e90540845b5` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa7b5ce53d71d2215d679642307adfa3f39de5448` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x400fa4d0617d48c483f4bc845b1e017e9e3a4671` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x77f12af1fcd55847fa3f96739c10b797c8d63bb9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x7371ea066e8de34a0dc45135ae383364b7ae19b9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5bb8f1ce603577a4d17cc9d72f6a4c38f3b0b74c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x25051076324830cbace51adbf565b6236b8ea114` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x928d517e59592fb98e8a99b7fa779eb5025f2f50` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x12c157dba58285c422180cabd8f3b7783ec1a379` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb7e77fdbc1168fe6d7e5972ca141b1c15383595a` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0fc06977378fd94aa23af24ee75b993c265c140b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcaa3a16f8440f85303afaab1992f2b97d12469b1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9235910f410478b89f86e06be0e4535335af6dea` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x7c8d53f9830bf2ea1bbec53e6f588b8f14970014` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9649e64c2d551567c213f39d28f9c505980223b1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd8305fba85347a596f7c1e165e5e871ff9981e0e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9e5fd9cdf6bb538ecf31fdb8f3386a27c682ed25` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x54c1c66ce1ae0070a0e558f81dae5ea39bc02ca2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9fe04652277ef2de68ac4e635c13ea9fb418abae` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbc6dbe2c5f172bbb736aa58cfa9bfcb20c85ffe2` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1f9b4a5ddd586f8471cda36262c52e5872a7d257` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x53706e1a50e21d81f4e85f2aa69633ae4a869017` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9dfc840576e285ee2d08358098d26cb02c0782cc` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xebedc8e9ff409b23dd251f87ccbffa8075f87255` | `SWING_TRADER` | `0.20` | Yes | No | Swing trader: long holding time of 99.4 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0xe7f88f2981cb0e0635a045e04e528544c4f47ce3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd293d05e83035a757d510e2a5ebfe705c4f05b1f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf994b7a3fca237ef472ba83a33a40e6307613327` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8973be4402bf0a39448f419c2d64bd3591dd2299` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x50bf934781f63028c5c8ef49c4affe19a86d99b6` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 66.6% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb6fa76be6f9503299dded765b903bbff3f9164d0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3655d11a671d3a5797577037ad8524667dbd242f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbc54c6e222ae663d993dbd229a6c20a675ddb864` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcbf5959b2c660fbc602e8e14749b3a355380aef8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x47dd8f4625135da6de6ab250592e87c08d31c279` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x144a32ae85dc9366ecbff13b6ed103fafdafc12f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd1f5c4341099ed1a5697c749aadb07f53fd0feb3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xedba30dbfccca7ddf3e26d5645e095d5909d2b60` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0124d0fa0dfb1430dfcff16ec6a96945e7a0bc10` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 71.4% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x74acb8341ba2fe9b9533beaf8d537f4e08b5d4e8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x74de5d4fcbf63e00296fd95d33236b9794016631` | `HIGH_RISK_DEGEN` | `0.05` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8674c15a9dfa1c05f5719b8a938e8897de5a18f3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0b8d6ab7cc4d99e8586200a633391a973faf4967` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9434cb4b697112ee56e97a161eb1dbcbb9c29fed` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf9e86093a15f43f5f4ba9dfc2d7731d43451b920` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfc2b5d8f62b3bc58694f44df9c5c78bc559ced31` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x18e5d061ed447fc0d4aa0d73d3c27507658e8bd8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3498b139e18432186a9fc297c12c53218a6596b0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0e70816fe5fff71b578ec4ed6fe19e9f13c5d4f3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfdb71c7ae32d13c55c87f1b179121b33f0483f13` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6b5c131a3ecf4060fb6e31986ff853994a5d5b94` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb57f6709d807184d69a7d75e7fd94d5528d1e350` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x98c3d3183c4b8a650614ad179a1a98be0a8d6b8e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x000000000004444c5dc75cb358380d2e3de08a90` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x12dde15d3bc00d69fb6f94b5afe8cb71a364756c` | `HIGH_RISK_DEGEN` | `0.05` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4d4e14fbdf6bb02b6e036f86f120f00abe41c791` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xffa98a091331df4600f87c9164cd27e8a5cd2405` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5344986e555d5e13157c42dd71791132032c6aa2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbd66017f0367cd09fed8efcd414cae12208ee3f5` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x442b356321eb18aac800b054b78363422cc806fb` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x74d8b52f655a65926d98ff034339749e7576001d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbeccf19b026f12677e5ce1ce9e2d94dcc4772feb` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xe22479f5db404af16ea32d77d24a71c513e748d4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc142841714663ef1f0b747493294f668bb11c715` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xac733dcd7677160350a5e23c5950bbbeafa2dd8c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf346d00965776e504930675100c8e2871bd6530d` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3881f0735d9c8aaa779fce6f72938c52e424f852` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x93f6a87eb364bf59b95a1f523a2c23e33c0eebda` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbb0010ebbbe767d8e2d649e333084dedeaacda33` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x57c2b84da8c41af5426d20f5370a440693db1750` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfd703dfa9563ce205ce430115cec14a4ce744e51` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfe6798bd487c7cd87c4171e6633ca6d0cde2dc5b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0f351ffb3e01a6a062f38afd54b1e8f6f3d763cb` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xde73e17961f613c38d996af8a504e3f007ca0bee` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x28bf6006d87de7f44445905aa4f5cb8c0d8cba02` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xde485e0e64ba35e2532de076d02c75d33ee61eb5` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb10dc6c9916c2ab4f75249e8292da149989be90e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0afa3a877055f93c87381a7407db9c3d8c1bff47` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf389a602da05a70602302db80284658b78c4119c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbdb3ba9ffe392549e1f8658dd2630c141fdf47b6` | `SWING_TRADER` | `0.3` | Yes | **YES** | Swing trader: long holding time of 348.6 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0xb36ce3f521a1ff4b0cffb82ddd71f0a51fcf56ce` | `SWING_TRADER` | `0.40` | Yes | No | Swing trader: long holding time of 428.3 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x7a250d5630b4cf539739df2c5dacb4c659f2488d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0e0d26d1c8bdf043870385451577ae06fab1e279` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5505067968303b825e277de143c514f1c2806c63` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4337012eaf1f862b8dbdc6b62a01782ae01ef038` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x597676b79dd4ea0497a6692b84a4f58a6267d1db` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9359269cfbb80d154129dd9b0074cd7336b9a3a6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xffdc5c95e052c868cc10f6ef8119556ad22fdb53` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x140022b7000081000001094700609300d2187dab` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd6ff359696cf2dd0a51e1e9f00f9ad3177541070` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x94307b9bd6e6b293521234e4ffecaa46b3cb218e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1062726f45726917105eb4161fd86194f97ab188` | `HIGH_RISK_DEGEN` | `0.05` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfd40632702d98f9e8e56975c674f809a7efa6a29` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x81dc771e308c76c3e06f635abed7fb24830fb824` | `SWING_TRADER` | `0` | Yes | No | Swing trader: long holding time of 630.6 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0xdfc66bc9e0d36d23227f6f6330b3e13b4b915e17` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xea290ce3eae57bdb37e57872a5a14dc0d2f6e614` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6a8904836c21d0fd01d15e947b47c7589c794ce8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2a31c3116f35c6f83bc4f1cf54a25e40d9f3bfa8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x949dc8a5aa7e538a78b6eb1f6063b06140681bd8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2d9f65534eecba1f3401fe39925dacb08ccf8e89` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x85aa97537837514a9229fcaeb66eeef7242122bf` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb7a92dadd630cf01ef7e5736a0ba0d49b40ea979` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x111111125421ca6dc452d289314280a0f8842a65` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5f6e9310fb34089442d0f4f36b831ed19f1ec1d3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xce54f65abb8b61b83c14cbe97de97fce75bbf556` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xda9f2211bdd7cb647afe2d8734e98e86c5df4ce2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5c79485385f35ffdf7a394cb66ec1a031da1faf0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc03c6f5d6c5bf2959a4e74e10fd916b5b50bf102` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xe08d97e151473a848c3d9ca3f323cb720472d015` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x7cd2e1f1872852d24514736c4af65dd058bb182b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xaf95d3a7a25f831dc2b9a6704554435957b51ec2` | `SWING_TRADER` | `0.05` | Yes | No | Swing trader: long holding time of 319.3 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x9008d19f58aabd9ed0d60971565aa8510560ab41` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb5dcd76a21e2af116b72641354718fcef5cb62ef` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x678806d763d794b341054def718c08e6e6bb3adb` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5c8e47d95714c0dd48c7b0c91b0228ccd3dfbfbe` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb658b412691e9b5f6455f908882d9261987adf38` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc1d86cf6205d226bf5848f094309ecdbbc4569b4` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xae0cc0d6e76c2b13451ab90cd13a5fa71a5c59ba` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5b43453fce04b92e190f391a83136bfbecedefd1` | `SWING_TRADER` | `0.20` | Yes | No | Swing trader: long holding time of 477.1 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x52a6611fb62b1fd4d584f18bb44f64b862d8fd15` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x43de4318b6eb91a7cf37975dbb574396a7b5b5c6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc4670e0004c57266fd8f1afefbbd14e27ced42bf` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd6f3768e62ef92a9798e5a8cedd2b78907cecef9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x68baa34d092bc0df6295d376797b758b091524a9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3f5b41de43c1b5bed768b9998be84be6f0a9beb3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2cdbd1f60e042f926f017b5aac219d152743e948` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0f432a1cd858e4c3262aa6023c75c0284cff7d8d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6f0a91ef8adeb54db0e63be507747ab9a31d3926` | `SWING_TRADER` | `0.05` | Yes | No | Swing trader: long holding time of 498.1 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0xfcfe058efa0be5bccfdd45e2c079997935fe0a64` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x57d361cc74926896b08b3f0e58a921283b56f77f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd86120f81ac77040b1f032175bc6eb76bd255d53` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x58a497019644d8c383c8d3764c32ffd43506e836` | `SWING_TRADER` | `0.85` | Yes | **YES** | Swing trader: long holding time of 177.6 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x76a411f14a704099ba476ce8dffc288a53295218` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3a9b501c84405b47487455cde7e6904f4d2cced6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6a000f20005980200259b80c5102003040001068` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4313c378cc91ea583c91387b9216e2c03096b27f` | `SWING_TRADER` | `0.20` | Yes | No | Swing trader: long holding time of 422.4 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x05d1136eac4d902727f9b012c6d8d7e1ad1351c8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x00351365832397c461114387d8edc4c30238f68e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbb15b371c703428b1a77aac36eee5aab04e69dac` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x10e3410f84b7acc2bb736abd0b7a18c5cdbd8e63` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcbc1a806b8fdea611251ff207c134475a2a8aed7` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6e2c8549ecc8615f14351889066a4ef196a688cd` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbedfb54088461888a8b472448109c962b0521940` | `HIGH_RISK_DEGEN` | `0.40` | No | No | High risk degen: 0 rug exposures, 90.9% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1d45a4e326a29454036e7154c60c4a855a041a9f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5f4c00ac8d434ae70c7d675a85afeebd5483a361` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa83d3cdff0c079693109a9176ab7980938697135` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x03c558aee40a9fced1cabe9b465bae8096e05230` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x94389149638d84271662998055cbf3607d08ac07` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbe6bbb45c0be7cba3087d345fab9a71ac88af200` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2cabfd2e2644bcfe5ffd570dc6e9ba9174cb1423` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb37361ebebfe7e0f0d98300f0a8ae777daa1cc12` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x51301f41c8f83fbdffaeee11f846e9c114fb8046` | `SWING_TRADER` | `0.20` | Yes | No | Swing trader: long holding time of 273.6 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x1ffec7119e315b15852557f654ae0052f76e6ae1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x72ed24bdd9253595bdd01469a15a8852bee7e0da` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9b1ce810ac13ca02ccd4104434b6aa9ee33c271b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x03618327cb01c5476b70f4cbec3edf0cef47cbe2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcf4236db746dbc1855a4d095aaf58da9b030491e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa8ca67aca86c0e081bd17b039de3d8822f3d3414` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x335b0d8c07457ea1aff2dcf4cc5d4c347553c855` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x423d607bd4e213e9b64a54b324ab7f632feec647` | `HIGH_RISK_DEGEN` | `0.05` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x53be4d533999526841879288bc340127a9cc122f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc36ac214a6c250a3ee68f7493da56a778f161680` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2cebe55c5dcfa3adf2db0b1e3c4ee18056ff6e3e` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd533e09500fd227252297d480b5d27bf669058a3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x111116053f09d34a7eae8102887004445176ca11` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa6af316c9c7e092638ea1c891270c069c6a31100` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x09d1fe7fd83e612d87450459098d8341c3f8a36f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1644d2477f809cc2c71bccfd6dc9497e3f83210d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4dd7b9e7c1545c4a2b0d86d6102c37ea594e8b18` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf59fe48b4f0ee7c611303def5db9891a5947b0c6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3d9aae030b9661e3605b3acb5d0385ede221a0cc` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0d8a76b89edba941093f3ce833c6193bf2b903d8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x08b9efba614e16c5ecf6e34414f75efe5ad03432` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3acd76b306c83762ecaf0d2404be799a0560fc87` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x58d3382fc3fc09b08ee4560a41008856321e926d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4328942957fdaab891145eb14dcad3a0d9024439` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x79bf38b6364457a32be11763ca3c8c2a658ff829` | `SWING_TRADER` | `0` | Yes | No | Swing trader: long holding time of 621.9 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x64f7b44f8b77fbb6b9e60c30cbb753ab682626e3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcd96af51f488567c1c51cf8165412cd49428cc7e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd32541b58cf2ae8b26ec8252b9b426eaf5f1b416` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xdbe0f8831a3a0e85f4a17184aedf08512bcc837f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc06ecb8a03dbf6041c2353f1da611743ec4ff65e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3c68bc99e4bcff0362b7a44dba37777fcd8e7028` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa7c30eccbd279ac7e192c3555bbe46a4f292cbc2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xe09cf8f1cc9416869604b588a2c33fb62b90f67e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xebc4e40722820b504bcaede09e4c971ea8f574b7` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x56c262027e0de4aea31d2489529cb25d23e58a8b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x22ec88b9ff78c6f2458ab1a7aa8bb99d84bd4b86` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xece9d5e60ff8f715f3b5669496b5948f9374ac67` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x88cc0800464fcef3e643a369e8a0532990995eee` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x57e134f63ee831cb2758a311f099f4221472d630` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb154256024041e3011c8024874e698b69a2081b3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xdd9ad611a305f35c817bb918d157f1297a864b5e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x69c7bd26512f52bf6f76fab834140d13dda673ca` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 60.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9eb9e9b2ba5c2665a16beaa8e137410358b6beaf` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x11111605ef067242653c980b8f6f1ffe50305afe` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3328f7f4a1d1c57c35df56bbf0c9dcafca309c49` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb8888e29804bbf646023cf1d7692af976dc7d5fb` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x36331e299247e5d0d3261e1d9852f6e0cffee95c` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 91.6% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbdb3264daa759c52169093711963b7e90b2446da` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa23b6c608e9bbd7a90a91f1adf534b626e4a3b9b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8f61ee081111e6bce0301a4bd5567d02322c8f9f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfd5ccd9b1c034dfc4294fdeaf7c70016d0fa735a` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc10ee9031f2a0b84766a86b55a8d90f357910fb4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2a6c340bcbb0a79d3deecd3bc5cbc2605ea9259f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5a09a50666e49d001fe6cb2d3dd41d287eb85274` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6ada49aeccf6e556bb7a35ef0119cc8ca795294a` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x11111605b53ecef22726df86881e4d6d40b5ca11` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9df87be88267c9ffbac077d4214270b3667cf581` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2fa0e85a69d2fcc87d028bb5b0fcb25859b4f5be` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5fa60dd1d2809604496d3315ab6f878bd59f64d4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfec0cd186927ee80d0bd5f7837efbeec0f7f7c11` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xdf8adfe10d4a4d9f0fc4d3e377a6e8d5730eb40c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x352690e95d5e47d4ea1c5bc2070456745e4bbc59` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x755c1a8f71f4210cd7b60b9439451efcbeba33d1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xbe3c963bf81410b465043b89f7280ebd994dcbb2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3e53028cf69949f3b961ce786baf2d4d75166562` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb4897d49c5859b9bb5e3d6c4372bdd83d55c8d6c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3f47ece7d2fd7f470c4a22a15c495d689e36010d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x51f2d44c61ffed56715bd4fb8d76b8a3a15137a1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5c3baf394f2150989df6dbe6af3667322172c3e0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9942d605a940adac4bc7e0d8f2a879b1c3fa9e03` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x93793bd1f3e35a0efd098c30e486a860a0ef7551` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfd68a66d8e60a469645eb2c7598d42b9daac5890` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xad2eae16157002a97e11b4201d111e6cd4c977ca` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xab659dee3030602c1af8c29d146facd4aed6ec85` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xabba7bef6dffba87d66e6c3d2d612b1a57df2303` | `SWING_TRADER` | `0.85` | Yes | **YES** | Swing trader: long holding time of 311.3 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0xfcf083b04a25ea6403f2a38b8e275c57e48e2c06` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x77e58cbbfce525461fc368be797142f655775397` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xe93685f3bba03016f02bd1828badd6195988d950` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x74c53e4b16ed0490e2c347410efcdda251a86e28` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc0a6bb3d31bb63033176edba7c48542d6b4e406d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5268431292f7b423e622832d21644ff81d16502d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xedf1c284d314c07bf6e11e5e485a6aff6a351d7e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd763419782bd4eb71c2fe55e38735a7a4cee7aa6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb44446b0c8e56988c34f7ff73ae904982b5fdda5` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2325e3f261cadb1c30cebf66c9f95f6fb016c0d4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4ab274540881e2bdbb4b35cacd2348364ba409b4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xae6bbb0ce3329e7e50d028a4c14db645e666688e` | `SWING_TRADER` | `0.20` | Yes | No | Swing trader: long holding time of 431.1 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x2c937e3b0ea4198303d85ae11e4ac5fe3181c990` | `SWING_TRADER` | `0.20` | Yes | No | Swing trader: long holding time of 302.8 hours; POTENTIALLY COPIABLE: Positive expectancy and latency-resilient horizon |
| `0x1bfffb738d69167d5592160a47d5404a3cf5a846` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf23ba94814a929e15ea8478b19d82e447114a1f3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x98cd5cb9cf1c00916c9b34089315abe8dec1d881` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x87e2d4658f691b9cbad30453a5f00c89d33a9a6d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd5c5d4a48db56c35f09dbd89ab2fc3ef878b2cb1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x470dc172d6502ac930b59322ece5345dd456a03d` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 75.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x448166a91e7bc50d0ac720c2fbed29e0963f5af8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8a6df6c629cf940b0cc4a32101caa4a022e80234` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4ce3687fed17e19324f23e305593ab13bbd55c4d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x778dcaa27e9c669a71c462b724a4e97fc4f1bcc5` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc8df1a5953aa7f62a35ff1501e0c10eb3c33ebfe` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xada3344693f368cd0ebf510f26617ad4213bf5b3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x7620ea10ff7265629dc402b153d9b6cbe870e9ae` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x27c2a1733f14e1247c5feb1c37cd52ae7d0d2bf2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1563e9af51616e78830de3325da752de369c1714` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x429cbcfde3f838405cf1a409c02b9613c84dae27` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0280b5a5292f815f2e5392006c6cd0c0bd5edaa9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc5ad22e5ab910bfd0d11db8342dddf3e7e4ce0ad` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x026b4ec01a34ce0a7c17d5c6338a7691ee67ba5b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb4a7eae4b0da371320536b6554654d42f4f48f22` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc0f2a2c3a554154543044c598250fa97c397078e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x951133ea0f197f2807cbe8d8207662d65213ccf5` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8f611c2f97e80f5690ec24309e995490caf0b2a6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc0ffeebabe5d496b2dde509f9fa189c25cf29671` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 66.6% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4f9fc4e0b79c1cbf16e68863ad5e9de6a94a346c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd5b71d91191120b45f269dcb0dc3583fec16ef27` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x933a5b57ee1ea75cafa67433979e99d819f78fb0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x53db11fb8277f2beb7f0341add802db966b969a1` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x4c82d1fbfe28c977cbb58d8c7ff8fcf9f70a2cca` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xdef171fe48cf0115b1d80b88dc8eab59176fee57` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf9817b4542b2308cdc944a6b914a30019bdd88c4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb7f3914250942018a5a3deca3f807d80fe357889` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcab953070193d7dd0d41a437ba4a264c052614d0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf279dfcdd57e1571c95e2c5b7e2ee453cbcdf77f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd7ca08ec1aee9cce8a8eda9365343ef197674e1a` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6613a1d523c97ed71eca42a71b08d39d40a8e3f3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1097727b7215ab9caf68ae6775f53ec9f1e91493` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6f386255dc0beaaa83f0e8316791b9922da32013` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc2f62f209a1c3625c5befcdfb67f23f8e8bafc4b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xdfcf744c8ae896e8631ba9b9dc717546646f6708` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5f07ac4735921ec7d1c71109f24d68184d6d034d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xafd748c66b0cce2fb4f4bb33b2d804145e603634` | `HIGH_RISK_DEGEN` | `0.3` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcf227fd481bb9d3d1a3662dcdf3f4a90ab6bf67b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa5e79baee540f000ef6f23d067cd3ac22c7d9fe6` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2de28e7cd5deca7c8c54f0dfb027355d7a446461` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x65229d343dbb63fe9826fcf9586965b7a13cb5a0` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x29ba3d899e8a819cf920adaff53ef1cf31969e66` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x252f79485d880413572a1a2141fcf21750e1cbc4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x35c19fe97e55719d86bb5b2c838bdbf1fbf95d77` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x592d528dedfad8b4962828258f4f460350220cd2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x68fa181c720c07b7ff7412220e2431ce90a65a14` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xa7842153fde380a864726d0e91f14f6ffab7d46c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc46083442448bce64d74d41c04b17829e13c437f` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x83429dcf89e0bf5094fc93ec7199cb6fb096c3f4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1112956589a2bea1b038732db4ea6b0c416ef130` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5ced44f03ff443bbe14d8ea23bc24425fb89e3ed` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x51c72848c68a965f66fa7a88855f9f7784502a7f` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 61.2% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x8ec41ae49e15bb51bb84935bcbb1d9404d700421` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x71d34b7d64b3bf0d41a71b2be78f01bae0e0ca04` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x950fd558f47e234a2fde23b7d61f7ccdbcb4a86f` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x69460570c93f9de5e2edbc3052bf10125f0ca22d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf73644da46ab85da864fc4284322f489134ee82c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd90acddac86eeeaba70f35a218929c795621607e` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf6b4cbf5f6211645cb251ab5e111f0a8881259ec` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc74281560aa0828f91dd95c20172fa4f219bb819` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0898aebd0d5b3188fc184ec5d4e99b269b065905` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf6f757c34a8b699deeb2f54fb7545cb100db52dd` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb975a3c7737c4bfdcbfb2c0c42873be8a081300b` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6897f92d97311949c55309d553001745f71ddbde` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcf57bfbc6e4acda88147634148ab17cbbe875ee4` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2d6c94c97f7944b35372270339d9f944cd11b2d8` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf6dcdce0ac3001b2f67f750bc64ea5beb37b5824` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf4a1c3a0f386867938443436e07c60f8c35ceb71` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xcbc76807fdc08e28ad16ec905a10205f9092bbe9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x1befbba53c0e72254fc4a67b97045fb7b22c8a31` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf28b0b5db307d91074534d0ccac78a1349c8ea69` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xc810e4d33270a23f84c6484a0a518638758ff377` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xaed106dc09de4e64769136dd71d85fa4120fca8a` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2164d6b2ef215a4f46912bdd18f27a25041ebe29` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x426d6ccab8ae3379cc8bb0fdc5587ecf11689df2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9fe5f3eef4f8497b584602259d0588d07be86510` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x48245147fe9b805efd599521d66a36f30c187328` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfeefee6e90b7bc64f6adba1ddb6d5a13c2d169d3` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0889e9327b98d7d1be3c301a4585ff3330502c9a` | `HIGH_RISK_DEGEN` | `0` | No | No | High risk degen: 0 rug exposures, 66.6% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb00439e6f77af558c79005ee9d158df547490e88` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x9c8d7afdd90bbb07e54a60291c5af391d8ae6dfa` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xf76b2a36217dd793359cfcd405efc37d830f45cb` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x2c59900b9442b7a865f93219c04f553a0d7bd003` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3d90f66b534dd8482b181e24655a9e8265316be9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3b7a62c61e65501a9036176be0221896bfbc3739` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0037825fd75af7eeace28889665e3fac8fdb6300` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6f3e42720cddfa175de0739d79f84cafb05928e9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xd8899a51f402cfd84469eb5b33697b3b8836dbbf` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xba63a648330a369d28dbbcdc3e582be949afa18c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x6532fecd7f475119eb2548294a9371728fa447a9` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x3cec8205d840c47740ba1e010295fdd368299508` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x30701054871bc1a9ad74c04534cbade910388a22` | `HIGH_RISK_DEGEN` | `0.05` | No | No | High risk degen: 0 rug exposures, 100.0% loss rate; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xeaa9ebddd373c4bd8bb92dfcc9c7e7fcdb268e51` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x0d87256cf234c5541effd644c83db6a9dec0ea85` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xae97836e3dc86b7ad22882f0164c32a2f78b90cc` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xb4e2566ec0f8d783e380d5ec23f0bf67d64e145c` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xfba3180de0b37f36352e831cd697d5fb8167a4b2` | `HIGH_RISK_DEGEN` | `0.3` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xdc582af922b4721f87c3c8e82cdde5b45347d90d` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x5ed864e8f37dcb4f79c5172a815d1b3a9147eaab` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0xe2b4f23956a0d4553bde2d0dcd21c604beb7db64` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |
| `0x164dc5333938299b6db62f73b63f8270ff3892c2` | `HIGH_RISK_DEGEN` | `0` | No | No | Insufficient trades for deterministic clustering at evaluation timestamp; UNCOPIABLE: Negative expectancy and high rug vulnerability |

## 4. Latency Degradation Curve (Copiability Engine)

| Delay (s) | Mode | Realized Net PnL ($) | Win Rate | Copy Efficiency | Trades | Slippage USD |
|---|---|---|---|---|---|---|
| `0s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$0.00` |
| `1s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$44.59` |
| `2s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$60.81` |
| `5s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$92.69` |
| `10s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$128.12` |
| `15s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$154.96` |
| `30s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$214.47` |
| `60s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$296.18` |
| `120s` | `Empirical` | `$0.00` | `0.0%` | `0.00x` | `0` | `$407.09` |

## 5. Capital Scalability Curve (Liquidity Impact)

| Capital ($) | Net PnL ($) | Return (%) | Price Impact (bps) | Real Liquidity? | Capacity Status |
|---|---|---|---|---|---|
| `$100` | `$0.00` | `0.00%` | `0.0 bps` | `REAL_ONCHAIN` | EXHAUSTED |
| `$500` | `$0.00` | `0.00%` | `0.3 bps` | `REAL_ONCHAIN` | EXHAUSTED |
| `$1000` | `$0.00` | `0.00%` | `0.6 bps` | `REAL_ONCHAIN` | EXHAUSTED |
| `$5000` | `$0.00` | `0.00%` | `3.3 bps` | `REAL_ONCHAIN` | EXHAUSTED |
| `$10000` | `$0.00` | `0.00%` | `6.6 bps` | `REAL_ONCHAIN` | EXHAUSTED |
| `$50000` | `$0.00` | `0.00%` | `33.2 bps` | `REAL_ONCHAIN` | EXHAUSTED |
| `$100000` | `$0.00` | `0.00%` | `66.2 bps` | `REAL_ONCHAIN` | EXHAUSTED |

## 6. Out-Of-Sample Validation (Anti-Look-Ahead Split)

| Data Partition | Trades | Gross PnL ($) | Fees ($) | Net PnL ($) | Win Rate | Expectancy ($) | Trade Sharpe |
|---|---|---|---|---|---|---|---|
| Train (60%) | `37` | `$854.18` | `$10.30` | `$843.87` | `91.8%` | `$22.80` | `0.67` |
| Validation (20%) | `6` | `$3.71` | `$1.12` | `$2.59` | `33.3%` | `$0.43` | `-0.11` |
| Test / Blind OOS (20%) | `21` | `$27.06` | `$0.46` | `$26.60` | `76.1%` | `$1.26` | `1.35` |

## 7. Statistical Rigor: Permutation Testing & Bootstrap CIs

- **Unit of Randomization**: `WALLET`
- **Observed Trade Sharpe**: `1.35`
- **Null Mean Sharpe ($H_0$)**: `0.21`
- **Null Median Sharpe**: `0.21`
- **Empirical $p$-value**: `0.0060` (Statistically Significant at alpha=0.05)

### Bootstrap Confidence Intervals (Resampled by Wallet)

| Metric | Unit | Mean | Median | 95% CI Lower | 95% CI Upper | 99% CI Lower | 99% CI Upper |
|---|---|---|---|---|---|---|---|
| Win Rate | % | `0.57` | `0.75` | `0.00` | `0.76` | `0.00` | `0.76` |
| Expectancy ($) | USD | `0.70` | `0.36` | `-0.01` | `1.26` | `-0.01` | `1.26` |
| Trade-Level Sharpe Ratio | ratio | `1.35` | `1.35` | `1.34` | `1.35` | `1.34` | `1.35` |

## 8. Benchmark Comparisons

| Strategy | Total Return (%) | Trade Sharpe | Max Drawdown (%) | Win Rate | Monte Carlo Detail |
|---|---|---|---|---|---|
| SmartWalletCopy (Filtered OOS) | `0.26%` | `1.35` | `0.0%` | `76.1%` | - |
| NaiveCopy (Unfiltered All Wallets) | `0.11%` | `0.37` | `3.3%` | `38.0%` | - |
| RandomSelection (Monte Carlo N=100) | `0.15%` | `0.28` | `1.4%` | `34.4%` | Mean: 0.15%, StdDev: 0.34%, 95% CI: [-0.35%, 1.01%] |
| Buy & Hold (Empirical Market Basket) | `3.73%` | `N/A` | `0.0%` | `100.0%` | - |

