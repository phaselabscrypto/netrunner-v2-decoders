use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2ee829905525aaaf")]
pub struct CreateReferralTokenAccountIdempotent {}

pub struct CreateReferralTokenAccountIdempotentInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub project: solana_sdk::pubkey::Pubkey,
    pub referral_account: solana_sdk::pubkey::Pubkey,
    pub referral_token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub referral_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateReferralTokenAccountIdempotent {
    type ArrangedAccounts = CreateReferralTokenAccountIdempotentInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let project = accounts.get(1)?;
        let referral_account = accounts.get(2)?;
        let referral_token_account = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let referral_program = accounts.get(7)?;

        Some(CreateReferralTokenAccountIdempotentInstructionAccounts {
            payer: payer.pubkey,
            project: project.pubkey,
            referral_account: referral_account.pubkey,
            referral_token_account: referral_token_account.pubkey,
            mint: mint.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            referral_program: referral_program.pubkey,
        })
    }
}
