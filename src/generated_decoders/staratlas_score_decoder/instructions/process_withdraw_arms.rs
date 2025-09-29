use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2294b7deccdd5bdb")]
pub struct ProcessWithdrawArms {
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
    pub escrow_auth_bump: u8,
    pub escrow_bump: u8,
}

pub struct ProcessWithdrawArmsInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub arms_token_account_escrow: solana_sdk::pubkey::Pubkey,
    pub arms_token_account_return: solana_sdk::pubkey::Pubkey,
    pub arms_mint: solana_sdk::pubkey::Pubkey,
    pub escrow_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessWithdrawArms {
    type ArrangedAccounts = ProcessWithdrawArmsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [player_account, ship_staking_account, score_vars_account, score_vars_ship_account, arms_token_account_escrow, arms_token_account_return, arms_mint, escrow_authority, token_program, clock, ship_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ProcessWithdrawArmsInstructionAccounts {
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            arms_token_account_escrow: arms_token_account_escrow.pubkey,
            arms_token_account_return: arms_token_account_return.pubkey,
            arms_mint: arms_mint.pubkey,
            escrow_authority: escrow_authority.pubkey,
            token_program: token_program.pubkey,
            clock: clock.pubkey,
            ship_mint: ship_mint.pubkey,
        })
    }
}
