# PHASE 2 — SYSTEM AUDIT & METHODOLOGICAL ANALYSIS

**Date**: 2026-09-08  
**Repository**: `garder500/onchain-smart-trader`  
**Auditor**: Senior Distributed Systems, On-Chain Analytics & Trading Systems Engineer  
**Objective**: Comprehensive, unvarnished audit of Phase 1 MVP to prepare for Phase 2 Alpha Research & Copiability Engine.

---

## 1. DATA AUDIT

### Quelles données réelles sont actuellement disponibles ?
* **Actuellement en base de données** : Seulement les transactions, tokens et wallets injectés par les tests d'intégration et les commandes CLI de test (environ 45 trades, 13 tokens, 19 wallets).
* **Données réelles de mainnet** : Le client `chain::EvmClient` est configuré pour se connecter via RPC HTTP à Ethereum (`CHAIN_ID=1`, Alchemy ou LlamaRPC). Cependant, aucune session d'ingestion longue durée n'a encore alimenté la base de données PostgreSQL avec un historique étendu de blocs Ethereum réels.

### Quelle blockchain est effectivement supportée ?
* **EVM standard** (Ethereum Mainnet par défaut, configurable via `CHAIN_ID` et `RPC_HTTP_URL` pour Arbitrum, Base, BSC, Polygon). Les abstractions Alloy (`alloy 2.4`) supportent tout réseau compatible EVM.

### Quels événements sont réellement indexés ?
* **Dans le décodeur (`crates/chain/src/decoder.rs`)** : Les signatures Solidity d'événements ERC-20 `Transfer(address,address,uint256)` et Uniswap V2 `SwapV2` sont implémentées et testées unitairement via les macros Alloy `sol!`.
* **Dans l'indexer (`crates/indexer/src/processor.rs`)** : `process_block` itère sur les transactions du bloc (`block.transactions`), extrait les expéditeurs et destinataires, et détecte les créations de contrat (`to` est None). **Faiblesse identifiée** : `process_block` ne requiert pas encore les reçus de transaction (`eth_getTransactionReceipt`) ni les logs au niveau du bloc (`eth_getLogs`) dans sa boucle standard. Les trades sont actuellement injectés via `ingest_trade` (manuellement ou par replay).

### Les swaps sont-ils réellement détectés ?
* **Partiellement** : Le décodeur `EventDecoder::decode_swap_v2` sait décoder les logs Uniswap V2. Mais la capture automatique continue depuis un flux RPC direct nécessite d'écouter les logs de filtre `Swap` sur les routeurs/factories DEX ou d'ingérer les receipts de bloc.

### Quels DEX sont supportés ?
* **Uniswap V2** (et forks AMM identiques : Sushiswap, Pancakeswap V2).
* **Uniswap V3 / Algebra** : Signature présente dans le design mais non encore activée dans le processeur de streaming.
* **Aggregateurs (1inch, 0x, CoW Swap)** : Non implémentés.

### Les prix sont-ils réels ou reconstruits ?
* **Reconstruits / Déclaratifs** : Pour les swaps décodés, le ratio `amount_in / amount_out` combiné au prix de base de l'ETH ou de l'USDC permet d'estimer le prix. Dans les tests actuels et le replay, le prix USD est fourni directement dans le modèle de `Trade`.

### Les données de liquidité sont-elles réelles ?
* **Partiellement réelles** : Le décodeur possède l'événement `Sync(reserve0, reserve1)`. Dans l'indexer actuel, si aucune requête RPC n'est faite sur la réserve de la pool, la liquidité par défaut est initialisée avec un placeholder ou via le contexte du token injecté.

### Les holders sont-ils réellement disponibles ?
* **Non disponibles en direct via simple RPC** : Obtenir la distribution complète des holders requiert soit d'indexer l'intégralité des `Transfer` depuis le bloc de genèse du token, soit d'interroger une API d'indexation (ex: Etherscan, Covalent, Moralis). Actuellement, `top_holders` et `top_10_holder_concentration` sont des champs `Option<T>` souvent nuls ou mockés lors des tests.

### Les données historiques sont-elles disponibles ?
* **Seulement les données injectées localement** : Pas de dataset historique mainnet volumineux pré-embarqué. Une commande d'ingestion/importation de dataset historique réel est requise.

---

## 2. WALLET PROFILER AUDIT

### Comment les trades sont reconstruits ?
* Dans `crates/wallet-profiler/src/metrics.rs`, les transactions `Trade` sont filtrées avec `timestamp <= eval_timestamp` et triées chronologiquement.

### Comment les positions sont reconstruites et comment le FIFO fonctionne ?
* Un dictionnaire `buy_queues: HashMap<TokenAddress, Vec<(buy_price, remaining_qty, buy_time, fee_per_unit)>>` accumule les achats.
* Lorsqu'un `TradeSide::Sell` survient, la quantité vendue est appariée avec les tranches d'achat les plus anciennes (FIFO pur).
* Si la vente épuise une tranche, elle est retirée (`queue.remove(0)`), sinon la quantité restante est décrémentée.

### Comment le PnL est calculé ?
* `gross_pnl = (sell_price - buy_price) * matched_qty`
* `net_pnl = gross_pnl - total_fee`
* Le PnL réalisé est la somme exacte des `net_pnl` de tous les round-trips terminés.

### Comment les ventes partielles sont gérées ?
* Correctement gérées par la découpe de `matched_qty = sell_qty_remaining.min(buy_qty)` et l'ajustement du reliquat dans la file FIFO.

### Comment les tokens sans prix exploitable sont traités ?
* Si un trade n'a pas de prix ou si `buy_price == 0`, le calcul de pourcentage de retour retourne `0` pour éviter une division par zéro.

### Comment les wallets multi-DEX sont traités ?
* Le profiler traite les tokens par leur adresse de contrat ERC-20, indépendamment du DEX sur lequel l'échange a eu lieu.

### Comment les pertes totales / rugs sont traitées ? (FAILLE CRITIQUE IDENTIFIÉE)
* **Biais de survie majeur dans le profiler actuel** : Si un wallet achète un token qui fait un rug pull (chute à 0) et qu'il **ne vend jamais**, aucun événement `TradeSide::Sell` n'est émis. Par conséquent, **aucun round-trip n'est créé**, et la perte totale n'est **jamais comptabilisée dans le realized PnL ou le win rate** !
* Un wallet pourrait avoir acheté 10 tokens qui ont rug et 1 token gagnant vendu à +50% : son win rate calculé serait de 100% avec le code actuel !
* **Correction impérative** : Il faut comptabiliser les positions ouvertes dépréciées / tokens abandonnés / rugs comme des pertes réalisées à 100% lors de l'évaluation du wallet à la date $T$.

---

## 3. BACKTESTING & SIMULATION AUDIT

### Look-ahead bias
* **Bien protégé au niveau de l'accès aux données** : Les requêtes SQL (`WHERE timestamp <= $2`) et le tri chronologique dans `ReplayEngine` appliquent strictement la règle $T_{\text{event}} \le T_{\text{eval}}$.
* Le test `test_anti_look_ahead_enforcement` vérifie que toute tentative de violation déclenche une erreur explicite `DomainError::AntiLookAheadViolation`.

### Survivorship bias (Biais de survie)
* **Partiellement présent** : Si le dataset ne contient que des tokens ayant survécu ou enregistrés manuellement, les tokens morts sont ignorés. Le schéma de base de données supporte les tokens abandonnés, mais il faut s'assurer que les jeux de données d'expérience intègrent explicitement les rugs et échecs.

### Selection bias (Biais de sélection)
* Actuellement, les benchmarks comparatifs dans `ab_test.rs` utilisaient la même logique de signal avec des seuils différents au lieu d'instancier des stratégies formellement distinctes (`NaiveCopyStrategy`, `RandomSelectionStrategy`, `BuyAndHoldStrategy`).

### Timestamp accuracy & Ordre des événements
* Précision à la seconde (`DateTime<Utc>`).
* L'ordre au sein d'un même bloc dépend de l'index de transaction / log_index.

### Prix d'entrée & Prix de sortie
* Actuellement dans `ReplayEngine`, un signal émis à $T$ est exécuté au prix du trade de l'événement déclencheur sans délai de latence réseau ou de minage. C'est irréaliste : dans la réalité, copier un wallet prend entre 1 et 15 secondes (ou plusieurs blocs).

### Slippage, Frais, Liquidité & Fills partiels
* `PaperExecutor` implémente un modèle propre combinant spread de base (ex: 50 bps) et impact dynamique AMM proportionnel à `order_size / (2 * pool_liquidity)`.
* Les fills partiels sont simulés à 80% si la taille dépasse 5% du pool.
* Les frais sont déduits en basis points (ex: 30 bps).

---

## 4. STRATEGY AUDIT

* La stratégie `SmartWalletCopyStrategy` fonctionne logiquement : elle calcule le score du wallet, filtre par risque de token, taille la position selon le score/liquidité et applique les règles de sortie (TP/SL/Time/Copy-exit).
* **Limitation** : Elle dépend fortement de la qualité du score du wallet et des métriques du token contextuel.

---

## 5. CLASSIFICATION DES COMPOSANTS

| Composant | Statut | Commentaire |
| :--- | :---: | :--- |
| **Domain Models & Money Math** | `REAL` | Précision financière pure `Decimal`, pas de floats pour l'argent. |
| **PostgreSQL Schema & Migrations** | `REAL` | Schéma relationnel complet, contraintes, index, migrations actives. |
| **Alloy EVM Connection & RPC** | `REAL` | Connexion active aux nœuds Ethereum via Alloy 2.4. |
| **Bytecode Safety Inspection** | `REAL` | Analyse réelle du bytecode EVM pour sélecteurs `mint`, `pause`, `freeze`. |
| **ERC-20 / Uniswap Event Decoding** | `REAL` | Décodeurs basés sur `sol!` compilés et validés par tests unitaires. |
| **RPC Block Scanning (Live Indexer)** | `PARTIALLY REAL` | Scan les blocs et transactions, mais n'extrait pas encore les logs de receipts en flux continu. |
| **Wallet Profiler (FIFO & Metrics)** | `PARTIALLY REAL` | FIFO et PnL fonctionnels, mais `early_entry_ratio` et `rug_exposure` mockés/placeholder, et non-prise en compte des positions ouvertes non vendues (biais de survie). |
| **Token Risk Engine** | `REAL` | Formule multi-facteurs explicable avec pondérations configurables et motifs de rejet explicites. |
| **Paper Order Execution & AMM Slippage** | `PARTIALLY REAL` | Modèle mathématique d'impact AMM et frais réaliste, mais exécution instantanée sans modélisation du délai de copie (latence 0s forcée). |
| **Anti-Look-Ahead Guards** | `REAL` | Assertions de timestamp et requêtes chronologiques strictes. |
| **Replay Engine & Backtest** | `PARTIALLY REAL` | Chronologique et fonctionnel, mais manque de modélisation de latence de copie et de split Train/Validation/Test. |
| **A/B Testing Framework** | `PARTIALLY REAL` | Compare 4 approches mais n'a pas encore de randomisation formelle par graine (seed) ni de walk-forward. |
| **Live Mainnet Trade Dataset** | `MOCKED` / `SYNTHETIC` | La base actuelle ne contient que les données générées par les tests et fixtures. |

---

## 6. FAILLES MÉTHODOLOGIQUES CRITIQUES À CORRIGER AVANT LE RESEARCH ENGINE

1. **Biais de survie dans le Wallet Profiler** :
   * Actuellement, les tokens achetés mais jamais vendus (rug pulls, faillites, honeypots) ne génèrent aucun round-trip FIFO et sont ignorés du calcul du win rate et du PnL.
   * *Correction* : Les tokens encore en portefeuille au moment de l'évaluation $T$ doivent être évalués au prix de marché actuel à $T$ (ou 0 si liquidité détruite / rug), et inclus dans le PnL total et les statistiques de trade.
2. **Absence de délai de copie (Zero Latency Illusion)** :
   * L'illusion de pouvoir acheter au prix exact où le smart wallet a acheté dans le même bloc.
   * *Correction* : Implémenter le `CopiabilityEngine` paramétrable par délai ($0\text{s}, 1\text{s}, 2\text{s}, 5\text{s}, 10\text{s}, 15\text{s}, 30\text{s}, 60\text{s}, 120\text{s}$) avec impact de glissement de prix temporel.
3. **Statut des données non explicite dans les résultats** :
   * Tout rapport doit afficher obligatoirement `DATA_SOURCE = REAL | SYNTHETIC | MIXED` et bannir toute revendication d'alpha sur des données synthétiques.
4. **Calculs statistiques robustes manquants** :
   * Win rate et ROI bruts sont insuffisants. Il faut ajouter `expectancy = (win_rate * avg_win) - (loss_rate * avg_loss)`, l'analyse de distribution, le bootstrap pour intervalles de confiance à 95%, et des fenêtres glissantes walk-forward.
