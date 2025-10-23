use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9bb38297c48bfda3")]
pub struct TransferToSponsor {
    pub amount: u64,
}

pub struct TransferToSponsorInstructionAccounts {
    pub user_token_authority: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub sponsor: solana_sdk::pubkey::Pubkey,
    pub sponsor_token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferToSponsor {
    type ArrangedAccounts = TransferToSponsorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let user_token_authority = accounts.get(0)?;
        let user_token_account = accounts.get(1)?;
        let sponsor = accounts.get(2)?;
        let sponsor_token_account = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let associated_token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(TransferToSponsorInstructionAccounts {
            user_token_authority: user_token_authority.pubkey,
            user_token_account: user_token_account.pubkey,
            sponsor: sponsor.pubkey,
            sponsor_token_account: sponsor_token_account.pubkey,
            mint: mint.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
