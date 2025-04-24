use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ConfigureConfigArgs {
    pub admin: Option<solana_sdk::pubkey::Pubkey>,
    pub operator: Option<solana_sdk::pubkey::Pubkey>,
    pub pause_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub epochs_to_claim_settlement: Option<u64>,
    pub withdraw_lockup_epochs: Option<u64>,
    pub minimum_stake_lamports: Option<u64>,
    pub slots_to_start_settlement_claiming: Option<u64>,
    pub min_bond_max_stake_wanted: Option<u64>,
}
