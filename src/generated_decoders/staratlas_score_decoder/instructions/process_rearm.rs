use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x42871ec29ddb21db")]
pub struct ProcessRearm {
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
    pub escrow_auth_bump: u8,
    pub escrow_bump: u8,
    pub arms_quantity: u64,
}

pub struct ProcessRearmInstructionAccounts {
    pub token_owner_account: solana_sdk::pubkey::Pubkey,
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub escrow_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
    pub arms_mint: solana_sdk::pubkey::Pubkey,
    pub arms_token_account_source: solana_sdk::pubkey::Pubkey,
    pub arms_token_account_escrow: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessRearm {
    type ArrangedAccounts = ProcessRearmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_owner_account, player_account, ship_staking_account, score_vars_account, score_vars_ship_account, escrow_authority, system_program, token_program, clock, rent, ship_mint, arms_mint, arms_token_account_source, arms_token_account_escrow, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ProcessRearmInstructionAccounts {
            token_owner_account: token_owner_account.pubkey,
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            escrow_authority: escrow_authority.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            ship_mint: ship_mint.pubkey,
            arms_mint: arms_mint.pubkey,
            arms_token_account_source: arms_token_account_source.pubkey,
            arms_token_account_escrow: arms_token_account_escrow.pubkey,
        })
    }
}
