

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct InitConfigArgs {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub operator_authority: solana_sdk::pubkey::Pubkey,
    pub epochs_to_claim_settlement: u64,
    pub withdraw_lockup_epochs: u64,
    pub slots_to_start_settlement_claiming: u64,
}
