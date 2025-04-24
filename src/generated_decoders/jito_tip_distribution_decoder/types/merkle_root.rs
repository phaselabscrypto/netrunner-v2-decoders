use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MerkleRoot {
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub max_num_nodes: u64,
    pub total_funds_claimed: u64,
    pub num_nodes_claimed: u64,
}
