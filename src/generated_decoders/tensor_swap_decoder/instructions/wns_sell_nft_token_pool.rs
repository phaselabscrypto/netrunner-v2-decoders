
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x284ef14eccee2e8f")]
pub struct WnsSellNftTokenPool{
    pub config: PoolConfig,
    pub min_price: u64,
}

pub struct WnsSellNftTokenPoolInstructionAccounts {
    pub shared: solana_sdk::pubkey::Pubkey,
    pub owner_ata_acc: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub approve_account: solana_sdk::pubkey::Pubkey,
    pub distribution: solana_sdk::pubkey::Pubkey,
    pub wns_program: solana_sdk::pubkey::Pubkey,
    pub distribution_program: solana_sdk::pubkey::Pubkey,
    pub extra_metas: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WnsSellNftTokenPool {
    type ArrangedAccounts = WnsSellNftTokenPoolInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let shared = accounts.get(0)?;
        let owner_ata_acc = accounts.get(1)?;
        let token_program = accounts.get(2)?;
        let associated_token_program = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let margin_account = accounts.get(5)?;
        let taker_broker = accounts.get(6)?;
        let approve_account = accounts.get(7)?;
        let distribution = accounts.get(8)?;
        let wns_program = accounts.get(9)?;
        let distribution_program = accounts.get(10)?;
        let extra_metas = accounts.get(11)?;

        Some(WnsSellNftTokenPoolInstructionAccounts {
            shared: shared.pubkey,
            owner_ata_acc: owner_ata_acc.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            margin_account: margin_account.pubkey,
            taker_broker: taker_broker.pubkey,
            approve_account: approve_account.pubkey,
            distribution: distribution.pubkey,
            wns_program: wns_program.pubkey,
            distribution_program: distribution_program.pubkey,
            extra_metas: extra_metas.pubkey,
        })
    }
}