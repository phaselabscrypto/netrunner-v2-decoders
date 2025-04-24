use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PayloadTypeLocal {
    Pubkey(solana_sdk::pubkey::Pubkey),
    Seeds(SeedsVecLocal),
    MerkleProof(ProofInfoLocal),
    Number(u64),
}
