
use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d68a1064d52ec0472")]
pub struct FundSettlementEvent{
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub funding_amount: u64,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub lamports_funded: u64,
    pub lamports_claimed: u64,
    pub merkle_nodes_claimed: u64,
    pub split_stake_account: Option<SplitStakeData>,
    pub split_rent_collector: Option<solana_sdk::pubkey::Pubkey>,
    pub split_rent_amount: u64,
}
