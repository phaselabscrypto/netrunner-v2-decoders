use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x208b70ab0002e19b")]
pub struct NewDistributor {
    pub params: NewDistributorParams,
}

pub struct NewDistributorInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub base: solana_sdk::pubkey::Pubkey,
    pub clawback_receiver: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for NewDistributor {
    type ArrangedAccounts = NewDistributorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, base, clawback_receiver, mint, token_vault, admin, system_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(NewDistributorInstructionAccounts {
            distributor: distributor.pubkey,
            base: base.pubkey,
            clawback_receiver: clawback_receiver.pubkey,
            mint: mint.pubkey,
            token_vault: token_vault.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
