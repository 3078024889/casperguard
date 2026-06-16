# CasperGuard — Architecture Deep Dive

## Overview

CasperGuard is a three-layer system: a data layer (x402), an intelligence layer (AI engine), and a trust layer (Casper blockchain).

## Layer 1: x402 Data Layer

The agent uses the HTTP 402 payment protocol to fetch RWA data from providers. This means:

- **No subscriptions** — the agent pays only for what it uses
- **Atomic payment + data** — no payment without data, no data without payment
- **Cryptographic proof** — every payment is signed with the agent's Ed25519 key

The x402 flow on Casper:
1. Agent requests data endpoint
2. Server returns `402 Payment Required` with `PaymentRequirements` (amount in CSPR, recipient address)
3. Agent builds a `PaymentPayload` signed with its private key
4. Agent retries with `X-Payment` header
5. Server forwards to CSPR.cloud x402 Facilitator for on-chain settlement
6. Agent receives data on `200 OK`

## Layer 2: AI Compliance Engine

The compliance engine applies a weighted multi-factor scoring model:

| Factor | Weight | Source |
|--------|--------|--------|
| Jurisdiction risk (FATF lists) | 45% | Off-chain regulatory databases |
| Issuer identity verification | 30% | KYB/KYC data providers |
| AML threshold breach (>$10M) | 15% | Transaction value analysis |
| Data freshness | 10% | Timestamp validation |

Production enhancement: the engine calls an LLM via the Casper MCP Server to analyze regulatory filings and news sentiment for each asset, providing qualitative context alongside the quantitative score.

## Layer 3: Casper Trust Layer

The `ComplianceOracle` contract (Odra/Rust) provides:

- **Immutable attestation records** — every compliance assessment permanently on-chain
- **Oracle reputation** — accuracy scores tracked on-chain, queryable by DeFi protocols
- **Upgradability** — contract can be updated without redeployment (Casper native feature)
- **Multi-oracle support** — multiple AI agents can contribute attestations

## Data Flow

```
External RWA Data          CasperGuard Agent          Casper Testnet
─────────────────          ──────────────────          ─────────────
  
  Asset Registry  ──x402──▶  X402DataFetcher
                                    │
                                    ▼
                             ComplianceAI
                             (risk scoring)
                                    │
                                    ▼
                             CasperPublisher  ──RPC──▶  ComplianceOracle
                                                         (Odra contract)
                                    │
                                    ▼
                             compliance_reports.json
                                    │
                                    ▼
                             frontend/index.html
                             (Web Dashboard)
```

## Smart Contract Design

The `ComplianceOracle` contract is intentionally minimal and upgradable:

```rust
record_attestation(asset_id, risk_level, attestation_hash, timestamp)
get_attestation(asset_id) → AttestationRecord
get_oracle_reputation(oracle) → u32  // 0-100%
authorize_oracle(oracle)
update_oracle_accuracy(oracle, was_correct)
```

The `attestation_hash` is a SHA-256 of the full off-chain report, allowing anyone to verify that the on-chain record matches the detailed report without storing all data on-chain.

## Security Considerations

- Agent private key stored in environment variables, never in source code
- x402 payments are single-use signed payloads (replay protection built-in)
- Contract has owner-only oracle authorization
- Attestation hashes provide tamper-evident audit trail
