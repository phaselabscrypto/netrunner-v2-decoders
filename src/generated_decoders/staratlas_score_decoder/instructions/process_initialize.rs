use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb85814ad11335185")]
pub struct ProcessInitialize {
    pub bump: u8,
    pub treasury_bump: u8,
    pub treasury_auth_bump: u8,
}

pub struct ProcessInitializeInstructionAccounts {
    pub update_authority_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub treasury_token_account: solana_sdk::pubkey::Pubkey,
    pub treasury_authority_account: solana_sdk::pubkey::Pubkey,
    pub atlas_mint: solana_sdk::pubkey::Pubkey,
    pub fuel_mint: solana_sdk::pubkey::Pubkey,
    pub food_mint: solana_sdk::pubkey::Pubkey,
    pub arms_mint: solana_sdk::pubkey::Pubkey,
    pub toolkit_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessInitialize {
    type ArrangedAccounts = ProcessInitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [update_authority_account, score_vars_account, token_program, system_program, rent, treasury_token_account, treasury_authority_account, atlas_mint, fuel_mint, food_mint, arms_mint, toolkit_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ProcessInitializeInstructionAccounts {
            update_authority_account: update_authority_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            treasury_token_account: treasury_token_account.pubkey,
            treasury_authority_account: treasury_authority_account.pubkey,
            atlas_mint: atlas_mint.pubkey,
            fuel_mint: fuel_mint.pubkey,
            food_mint: food_mint.pubkey,
            arms_mint: arms_mint.pubkey,
            toolkit_mint: toolkit_mint.pubkey,
        })
    }
}
