# 🛡️ CasperGuard — AI-Powered RWA Compliance Oracle Agent

> **Casper Agentic Buildathon 2026** · Qualification Round Submission

CasperGuard is an autonomous AI agent that monitors Real-World Asset (RWA) compliance on the Casper Network. It fetches off-chain data via the **x402 pay-per-request protocol**, runs an AI-powered risk assessment, and publishes verifiable **compliance attestations on-chain** to Casper Testnet — creating a trust-minimized oracle for DeFi/RWA protocols.

---

## ✨ Key Features

| Feature | Description |
|---|---|
| 🤖 **Agentic AI** | Autonomous agent that monitors, assesses, and acts without human intervention |
| 💳 **x402 Micropayments** | Pays per API request using Casper's HTTP-native x402 protocol |
| ⛓️ **On-Chain Attestations** | Compliance results recorded on Casper Testnet via `ComplianceOracle` smart contract |
| 🏆 **Oracle Reputation** | On-chain accuracy score builds trust over time |
| 📊 **Live Dashboard** | Real-time web UI showing agent activity and risk reports |
| 🦀 **Odra Smart Contract** | Upgradable Rust contract deployed on Casper Testnet |

---

## ✅ Verified On-Chain (Testnet)

CasperGuard is not just deployed — it has produced a real, verifiable transaction on Casper Testnet.

| | |
|---|---|
| **Contract Hash** | `d37b0b7f234789379432d30de67bd54587f84e42969e87f5fa44e49e395a0987` |
| **Contract Package** | `464cbf123ad8a503b2c6e96dcdad8a9a421dcb953ed08514ec77322e49fc390f` |
| **Live Attestation Tx** | [`7f317f2d62ff3551f2751800816fcb20837befb50332b90fabb7dacfb9d1831a`](https://testnet.cspr.live/deploy/7f317f2d62ff3551f2751800816fcb20837befb50332b90fabb7dacfb9d1831a) |
| **Action** | `record_attestation` — Status: ✅ Success |
| **Live Trigger Tool** | [dashboard-trigger.html](https://3078024889.github.io/casperguard/dashboard-trigger.html) — connect your own Casper Wallet and submit a live attestation |

This wasn't just deployed and left idle — it has been called and produced a confirmed on-chain transaction, verifiable by anyone on testnet.cspr.live.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        CasperGuard Agent                        │
│                                                                 │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────────┐  │
│  │  x402 Data   │    │  AI Compliance│    │  Casper On-Chain │  │
│  │  Fetcher     │───▶│  Engine      │───▶│  Publisher       │  │
│  │              │    │              │    │                  │  │
│  │ Pay-per-req  │    │ Risk scoring │    │ Deploy to        │  │
│  │ data feeds   │    │ Flag analysis│    │ Testnet via RPC  │  │
│  └──────────────┘    └──────────────┘    └──────────────────┘  │
│          │                                        │             │
│          ▼                                        ▼             │
│   x402 Facilitator                    ComplianceOracle.wasm     │
│   cspr.cloud                          (Odra Framework)          │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
                    ┌───────────────────────┐
                    │   Web Dashboard       │
                    │   frontend/index.html │
                    │   Live attestations   │
                    └───────────────────────┘
```

---

## 📁 Project Structure

```
casperguard/
├── agent/
│   └── casperguard_agent.py       # Core AI agent (Python)
├── contracts/
│   └── compliance_oracle.rs       # Odra smart contract (Rust)
├── frontend/
│   └── index.html                 # Live dashboard (vanilla JS)
├── docs/
│   └── architecture.md            # Detailed architecture notes
├── compliance_reports.json        # Sample output (auto-generated)
├── requirements.txt
└── README.md
```

---

## 🚀 Quick Start

### Prerequisites
- Python 3.11+
- Rust + `cargo` (for contract compilation)
- A Casper Testnet account with test CSPR

### 1. Clone the repo
```bash
git clone https://github.com/YOUR_USERNAME/casperguard
cd casperguard
```

### 2. Install Python dependencies
```bash
pip install -r requirements.txt
```

### 3. Set environment variables
```bash
export CASPER_TESTNET_RPC="https://rpc.testnet.cspr.cloud"
export CSPR_CLOUD_API_KEY="your-cspr-cloud-api-key"
export AGENT_PRIVATE_KEY="your-ed25519-private-key-hex"
export COMPLIANCE_CONTRACT="your-deployed-contract-hash"
```

> 💡 Get a free CSPR.cloud API key at [cspr.cloud](https://cspr.cloud)  
> 💡 Get testnet CSPR from the [Casper Testnet Faucet](https://testnet.cspr.live/tools/faucet)

### 4. Run the AI agent
```bash
python agent/casperguard_agent.py
```

### 5. View the dashboard
Open `frontend/index.html` in your browser — no server needed.

---

## 🦀 Smart Contract Deployment (Odra)

```bash
# Install Odra CLI
cargo install odra-cli

# Build the contract
cd contracts
odra build

# Deploy to Casper Testnet
odra deploy --network testnet --contract ComplianceOracle

# Note the contract hash and set it in your .env
```

---

## 🔄 How It Works

### Step 1 — x402 Data Fetch
The agent requests RWA asset data from a protected endpoint. The server responds with `402 Payment Required` + payment details. The agent signs a micropayment in CSPR and retries — receiving the data atomically.

```
Agent → GET /api/asset/RWA-001
Server ← 402 Payment Required { amount: 0.001 CSPR, address: 01a3b5… }
Agent → GET /api/asset/RWA-001 + X-Payment: casper:01a3b5…:sig_ed25519…
Server ← 200 OK { asset data }
```

### Step 2 — AI Compliance Assessment
The AI engine applies multi-factor risk scoring:
- **Jurisdiction risk** (FATF blacklist / greylist matching)
- **Issuer identity verification** 
- **AML threshold checks** (>$10M triggers enhanced review)
- **Data freshness** (stale data increases uncertainty)

### Step 3 — On-Chain Attestation
Results are published to the `ComplianceOracle` contract on Casper Testnet:
```
record_attestation(
  asset_id: "RWA-001",
  risk_level: "LOW",
  attestation_hash: "a3f9b2c1…",  // SHA-256 of full report
  timestamp: 1749000000
)
```

### Step 4 — Oracle Reputation
The contract tracks each oracle's accuracy rate on-chain, creating a verifiable trust score that DeFi protocols can query before accepting attestations.

---

## 🌐 Live Demo

- **Dashboard:** [casperguard-dashboard.html](https://3078024889.github.io/casperguard/casperguard-dashboard.html) — hosted live, no install needed
- **Live Attestation Trigger:** [dashboard-trigger.html](https://3078024889.github.io/casperguard/dashboard-trigger.html) — connect Casper Wallet and submit a real on-chain attestation yourself
- **Testnet Explorer:** [testnet.cspr.live](https://testnet.cspr.live)
- **Verified Transaction:** [7f317f2d...1831a](https://testnet.cspr.live/deploy/7f317f2d62ff3551f2751800816fcb20837befb50332b90fabb7dacfb9d1831a)
- **Demo Video:** [YouTube link]

---

## 🛠️ Casper AI Toolkit Used

| Tool | Usage |
|---|---|
| [x402 Protocol](https://docs.cspr.cloud/x402-facilitator-api/reference) | Pay-per-request data fetching |
| [Casper MCP Server](https://docs.cspr.cloud/agentic-tools/mcp-server) | Blockchain queries from AI agent |
| [CSPR.cloud API](https://cspr.cloud) | RPC access + streaming events |
| [Odra Framework](https://odra.dev) | Smart contract development |
| [casper-eip-712](https://github.com/casper-ecosystem/casper-eip-712) | Typed-data signing for x402 payments |

---

## 📋 Compliance Risk Levels

| Level | Score Range | Action |
|---|---|---|
| 🟢 **LOW** | 0–20% | Standard monitoring |
| 🟡 **MEDIUM** | 20–50% | Enhanced due diligence |
| 🟠 **HIGH** | 50–75% | Legal review required |
| 🔴 **CRITICAL** | 75–100% | Reject — AML/sanctions risk |

---

## 🗺️ Roadmap

- [x] MVP agent with x402 data fetching
- [x] AI compliance scoring engine
- [x] On-chain attestation via Odra contract — **confirmed live on testnet** ([tx](https://testnet.cspr.live/deploy/7f317f2d62ff3551f2751800816fcb20837befb50332b90fabb7dacfb9d1831a))
- [x] Web dashboard
- [ ] Real x402 endpoint integration (live data providers)
- [ ] Multi-agent swarm (Risk Agent + Treasury Agent + Legal Agent)
- [ ] ZK-proof of compliance (without revealing underlying data)
- [ ] Mainnet deployment + DAO governance

---

## 👥 Team

Built with ❤️ for the **Casper Agentic Buildathon 2026**

---

## 📄 License

MIT License — open source, ready for ecosystem integration.
