use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd42f5f5c726683fa")]
pub struct OpenPositionWithTokenExtensions {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub with_token_metadata_extension: bool,
}

pub struct OpenPositionWithTokenExtensionsInstructionAccounts {
    pub funder: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token2022_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_update_auth: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPositionWithTokenExtensions {
    type ArrangedAccounts = OpenPositionWithTokenExtensionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let funder = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let position = accounts.get(2)?;
        let position_mint = accounts.get(3)?;
        let position_token_account = accounts.get(4)?;
        let whirlpool = accounts.get(5)?;
        let token2022_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let associated_token_program = accounts.get(8)?;
        let metadata_update_auth = accounts.get(9)?;

        Some(OpenPositionWithTokenExtensionsInstructionAccounts {
            funder: funder.pubkey,
            owner: owner.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_token_account: position_token_account.pubkey,
            whirlpool: whirlpool.pubkey,
            token2022_program: token2022_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_update_auth: metadata_update_auth.pubkey,
        })
    }
}
