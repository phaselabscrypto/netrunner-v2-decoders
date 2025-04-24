use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x95ea1f671a24a631")]
pub struct SellNftTokenPoolT22 {
    pub config: PoolConfig,
    pub min_price: u64,
}

pub struct SellNftTokenPoolT22InstructionAccounts {
    pub shared: solana_sdk::pubkey::Pubkey,
    pub owner_ata_acc: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SellNftTokenPoolT22 {
    type ArrangedAccounts = SellNftTokenPoolT22InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let shared = accounts.get(0)?;
        let owner_ata_acc = accounts.get(1)?;
        let token_program = accounts.get(2)?;
        let associated_token_program = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let margin_account = accounts.get(5)?;
        let taker_broker = accounts.get(6)?;

        Some(SellNftTokenPoolT22InstructionAccounts {
            shared: shared.pubkey,
            owner_ata_acc: owner_ata_acc.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            margin_account: margin_account.pubkey,
            taker_broker: taker_broker.pubkey,
        })
    }
}
