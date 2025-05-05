use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01b6873b9b1963df")]
pub struct ClosePositionWithTokenExtensions {}

pub struct ClosePositionWithTokenExtensionsInstructionAccounts {
    pub position_authority: solana_sdk::pubkey::Pubkey,
    pub receiver: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token2022_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePositionWithTokenExtensions {
    type ArrangedAccounts = ClosePositionWithTokenExtensionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let position_authority = accounts.get(0)?;
        let receiver = accounts.get(1)?;
        let position = accounts.get(2)?;
        let position_mint = accounts.get(3)?;
        let position_token_account = accounts.get(4)?;
        let token2022_program = accounts.get(5)?;

        Some(ClosePositionWithTokenExtensionsInstructionAccounts {
            position_authority: position_authority.pubkey,
            receiver: receiver.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_token_account: position_token_account.pubkey,
            token2022_program: token2022_program.pubkey,
        })
    }
}
