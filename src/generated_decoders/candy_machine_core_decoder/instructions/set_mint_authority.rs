

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x437f9bbb64ae6779")]
pub struct SetMintAuthority{
}

pub struct SetMintAuthorityInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetMintAuthority {
    type ArrangedAccounts = SetMintAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let mint_authority = accounts.get(2)?;

        Some(SetMintAuthorityInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}