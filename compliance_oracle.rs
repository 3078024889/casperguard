// CasperGuard — ComplianceOracle Smart Contract
// Built with Odra Framework for Casper Network
// Casper Agentic Buildathon 2026
//
// This upgradable contract stores AI-generated compliance attestations
// on-chain, maintaining a verifiable audit trail for RWA assets.

use odra::prelude::*;
use odra::casper_types::U256;

/// A single compliance attestation record stored on-chain
#[odra::odra_type]
pub struct AttestationRecord {
    pub asset_id: String,
    pub risk_level: String,       // "LOW" | "MEDIUM" | "HIGH" | "CRITICAL"
    pub attestation_hash: String, // SHA-256 of off-chain report
    pub timestamp: u64,           // Unix epoch seconds
    pub oracle_address: Address,  // The AI agent's on-chain identity
}

/// Main ComplianceOracle contract module
#[odra::module]
pub struct ComplianceOracle {
    /// Owner/admin of the contract (can upgrade oracle agents)
    owner: Var<Address>,

    /// Authorized oracle agents allowed to submit attestations
    authorized_oracles: Mapping<Address, bool>,

    /// All attestation records keyed by asset_id
    attestations: Mapping<String, AttestationRecord>,

    /// Total attestations submitted
    total_attestations: Var<u32>,

    /// Per-oracle reputation score (correct attestations / total)
    oracle_accuracy: Mapping<Address, u32>,
    oracle_total: Mapping<Address, u32>,
}

#[odra::module]
impl ComplianceOracle {
    /// Initialize the contract with the deployer as owner
    pub fn init(&mut self) {
        let caller = self.env().caller();
        self.owner.set(caller);
        self.authorized_oracles.set(&caller, true);
        self.total_attestations.set(0u32);
    }

    // ── Admin ─────────────────────────────────────────────────────────────

    /// Authorize a new AI oracle agent to submit attestations
    pub fn authorize_oracle(&mut self, oracle: Address) {
        self.only_owner();
        self.authorized_oracles.set(&oracle, true);
        self.env().emit_event(OracleAuthorized { oracle });
    }

    /// Revoke an oracle's authorization
    pub fn revoke_oracle(&mut self, oracle: Address) {
        self.only_owner();
        self.authorized_oracles.set(&oracle, false);
        self.env().emit_event(OracleRevoked { oracle });
    }

    // ── Oracle Actions ────────────────────────────────────────────────────

    /// Record a new compliance attestation on-chain
    /// Called by the CasperGuard AI agent after completing its assessment
    pub fn record_attestation(
        &mut self,
        asset_id: String,
        risk_level: String,
        attestation_hash: String,
        timestamp: u64,
    ) {
        let caller = self.env().caller();
        self.only_authorized_oracle(&caller);

        let record = AttestationRecord {
            asset_id: asset_id.clone(),
            risk_level: risk_level.clone(),
            attestation_hash: attestation_hash.clone(),
            timestamp,
            oracle_address: caller,
        };

        self.attestations.set(&asset_id, record);

        let prev = self.total_attestations.get_or_default();
        self.total_attestations.set(prev + 1);

        // Increment oracle's total submission count for reputation tracking
        let prev_total = self.oracle_total.get_or_default(&caller);
        self.oracle_total.set(&caller, prev_total + 1);

        self.env().emit_event(AttestationRecorded {
            asset_id,
            risk_level,
            attestation_hash,
            oracle: caller,
            timestamp,
        });
    }

    /// Update the accuracy score of an oracle (called by owner after verification)
    pub fn update_oracle_accuracy(&mut self, oracle: Address, was_correct: bool) {
        self.only_owner();
        if was_correct {
            let prev = self.oracle_accuracy.get_or_default(&oracle);
            self.oracle_accuracy.set(&oracle, prev + 1);
        }
    }

    // ── Queries ───────────────────────────────────────────────────────────

    /// Get the latest attestation for an asset
    pub fn get_attestation(&self, asset_id: String) -> Option<AttestationRecord> {
        self.attestations.get(&asset_id)
    }

    /// Get the total number of attestations recorded
    pub fn get_total_attestations(&self) -> u32 {
        self.total_attestations.get_or_default()
    }

    /// Get oracle reputation score as a percentage (0–100)
    pub fn get_oracle_reputation(&self, oracle: Address) -> u32 {
        let correct = self.oracle_accuracy.get_or_default(&oracle);
        let total = self.oracle_total.get_or_default(&oracle);
        if total == 0 {
            return 100u32; // New oracles start at 100%
        }
        (correct * 100) / total
    }

    /// Check if an address is an authorized oracle
    pub fn is_authorized_oracle(&self, oracle: Address) -> bool {
        self.authorized_oracles.get_or_default(&oracle)
    }

    // ── Internal Guards ────────────────────────────────────────────────────

    fn only_owner(&self) {
        let caller = self.env().caller();
        let owner = self.owner.get().unwrap();
        if caller != owner {
            self.env().revert(Error::NotOwner);
        }
    }

    fn only_authorized_oracle(&self, caller: &Address) {
        if !self.authorized_oracles.get_or_default(caller) {
            self.env().revert(Error::NotAuthorizedOracle);
        }
    }
}

// ── Events ────────────────────────────────────────────────────────────────────
#[odra::odra_type]
pub struct AttestationRecorded {
    pub asset_id: String,
    pub risk_level: String,
    pub attestation_hash: String,
    pub oracle: Address,
    pub timestamp: u64,
}

#[odra::odra_type]
pub struct OracleAuthorized {
    pub oracle: Address,
}

#[odra::odra_type]
pub struct OracleRevoked {
    pub oracle: Address,
}

// ── Error Codes ───────────────────────────────────────────────────────────────
#[odra::odra_error]
pub enum Error {
    NotOwner = 1,
    NotAuthorizedOracle = 2,
    AssetNotFound = 3,
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::{Deployer, HostEnv, HostRef};

    #[test]
    fn test_record_and_query_attestation() {
        let env = odra_test::env();
        let mut contract = ComplianceOracleHostRef::deploy(&env, NoArgs);

        // Owner is the default caller; record an attestation
        contract.record_attestation(
            "RWA-001".to_string(),
            "LOW".to_string(),
            "abc123def456".to_string(),
            1_700_000_000u64,
        );

        assert_eq!(contract.get_total_attestations(), 1u32);

        let record = contract.get_attestation("RWA-001".to_string()).unwrap();
        assert_eq!(record.risk_level, "LOW");
        assert_eq!(record.attestation_hash, "abc123def456");
    }

    #[test]
    fn test_oracle_reputation_starts_at_100() {
        let env = odra_test::env();
        let contract = ComplianceOracleHostRef::deploy(&env, NoArgs);
        let owner = env.get_account(0);
        assert_eq!(contract.get_oracle_reputation(owner), 100u32);
    }

    #[test]
    fn test_unauthorized_oracle_reverts() {
        let env = odra_test::env();
        let mut contract = ComplianceOracleHostRef::deploy(&env, NoArgs);

        // Switch to a non-authorized account
        env.set_caller(env.get_account(1));

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            contract.record_attestation(
                "RWA-002".to_string(),
                "HIGH".to_string(),
                "deadbeef".to_string(),
                1_700_000_001u64,
            );
        }));
        assert!(result.is_err(), "Should revert for unauthorized oracle");
    }
}
