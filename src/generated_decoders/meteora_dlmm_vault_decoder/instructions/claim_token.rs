use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x74ce1bbfa6130049")]
pub struct ClaimToken {}

pub struct ClaimTokenInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub token_out_vault: solana_sdk::pubkey::Pubkey,
    pub destination_token: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimToken {
    type ArrangedAccounts = ClaimTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let escrow = accounts.get(1)?;
        let token_out_vault = accounts.get(2)?;
        let destination_token = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let owner = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(ClaimTokenInstructionAccounts {
            vault: vault.pubkey,
            escrow: escrow.pubkey,
            token_out_vault: token_out_vault.pubkey,
            destination_token: destination_token.pubkey,
            token_mint: token_mint.pubkey,
            token_program: token_program.pubkey,
            owner: owner.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
