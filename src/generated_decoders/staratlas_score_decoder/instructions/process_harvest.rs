use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbf466629e2247fa0")]
pub struct ProcessHarvest {
    pub staking_bump: u8,
    pub scorevars_ship_bump: u8,
    pub treasury_bump: u8,
    pub treasury_auth_bump: u8,
}

pub struct ProcessHarvestInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub player_atlas_token_account: solana_sdk::pubkey::Pubkey,
    pub treasury_token_account: solana_sdk::pubkey::Pubkey,
    pub treasury_authority_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessHarvest {
    type ArrangedAccounts = ProcessHarvestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [player_account, ship_staking_account, score_vars_ship_account, player_atlas_token_account, treasury_token_account, treasury_authority_account, token_program, clock, ship_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ProcessHarvestInstructionAccounts {
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            player_atlas_token_account: player_atlas_token_account.pubkey,
            treasury_token_account: treasury_token_account.pubkey,
            treasury_authority_account: treasury_authority_account.pubkey,
            token_program: token_program.pubkey,
            clock: clock.pubkey,
            ship_mint: ship_mint.pubkey,
        })
    }
}
