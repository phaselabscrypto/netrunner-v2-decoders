use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7372186f0e3571fe")]
pub struct CreateProtocolConfig {
    pub version: u16,
    pub params: ProtocolConfigParams,
}

pub struct CreateProtocolConfigInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub protocol_config_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
    pub msol_token_vault: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateProtocolConfig {
    type ArrangedAccounts = CreateProtocolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_program = accounts.get(0)?;
        let associated_token_program = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let protocol_config_state = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let protocol_owner_state = accounts.get(6)?;
        let msol_token_vault = accounts.get(7)?;
        let msol_mint = accounts.get(8)?;

        Some(CreateProtocolConfigInstructionAccounts {
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            system_program: system_program.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            msol_token_vault: msol_token_vault.pubkey,
            msol_mint: msol_mint.pubkey,
        })
    }
}
