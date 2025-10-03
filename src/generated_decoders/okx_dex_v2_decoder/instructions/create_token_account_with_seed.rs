use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7dbfef8c420809e4")]
pub struct CreateTokenAccountWithSeed {
    pub bump: u8,
    pub seed: u32,
}

pub struct CreateTokenAccountWithSeedInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateTokenAccountWithSeed {
    type ArrangedAccounts = CreateTokenAccountWithSeedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let token_account = accounts.get(2)?;
        let token_mint = accounts.get(3)?;
        let token_program = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(CreateTokenAccountWithSeedInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            token_account: token_account.pubkey,
            token_mint: token_mint.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
