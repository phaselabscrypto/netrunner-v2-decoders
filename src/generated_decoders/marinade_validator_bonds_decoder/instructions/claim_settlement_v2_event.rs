
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d72c98386b6a5ed2f")]
pub struct ClaimSettlementV2Event{
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub settlement_lamports_claimed: U64ValueChange,
    pub settlement_merkle_nodes_claimed: u64,
    pub stake_account_to: solana_sdk::pubkey::Pubkey,
    pub stake_account_withdrawer: solana_sdk::pubkey::Pubkey,
    pub stake_account_staker: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub index: u64,
}
