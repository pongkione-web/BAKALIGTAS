# BakaLigtas Crop Guard

An auditable, automated micro-subsidy distribution system built on Soroban to assist flood-impacted rice farmers in Central Luzon.

### Problem & Solution
Agricultural recovery initiatives suffer from weeks of bureaucratic processing time before physical checks are cleared. BakaLigtas clears these blockages by moving reserve allocations on-chain, triggering near-instant, auditable digital token disbursements straight to pre-verified rural agricultural workers when an emergency occurs.

### Timeline
* **Day 1:** Architecture mapping and smart contract structural configuration.
* **Day 2:** Mocking asset rules and compiling the code with strict test coverage.
* **Day 3:** Developing the front-end dashboard using the Stellar JavaScript SDK.
* **Day 4:** Final deployment on Testnet, validation tracking, and pitching adjustments.

### Stellar Features Used
* **Soroban Smart Contracts:** Secure and declarative logic controls ensuring public fund safety.
* **USDC Asset Transfers:** Immediate liquidity settlement bypassing localized bank clearings.

### Prerequisites
* Rust v1.70+
* Soroban CLI installed (`cargo install --locked soroban-cli`)

### How to Build
```bash
soroban contract build
## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CDEDFGYXJVQPTR57DK3UFFU32JCRAA3COBUISBFHHPWJLAQYXYMNMZHN` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CDEDFGYXJVQPTR57DK3UFFU32JCRAA3COBUISBFHHPWJLAQYXYMNMZHN) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/ce7fa0d07bdb277893b0b50e74dcbbeec1b85c79a5b96630a8a04b7c3c8e5710) |
| Deployed | 2026-06-26 06:40:26 UTC |
| Wallet | freighter (`GBN7…GEXB`) |
