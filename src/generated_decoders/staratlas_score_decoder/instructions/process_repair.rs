use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2ddd658f139a413c")]
pub struct ProcessRepair {
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
    pub toolkit_quantity: u64,
}

pub struct ProcessRepairInstructionAccounts {
    pub token_owner_account: solana_sdk::pubkey::Pubkey,
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
    pub toolkit_mint: solana_sdk::pubkey::Pubkey,
    pub toolkit_token_account_source: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessRepair {
    type ArrangedAccounts = ProcessRepairInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_owner_account, player_account, ship_staking_account, score_vars_account, score_vars_ship_account, system_program, token_program, clock, rent, ship_mint, toolkit_mint, toolkit_token_account_source, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ProcessRepairInstructionAccounts {
            token_owner_account: token_owner_account.pubkey,
            player_account: player_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            ship_mint: ship_mint.pubkey,
            toolkit_mint: toolkit_mint.pubkey,
            toolkit_token_account_source: toolkit_token_account_source.pubkey,
        })
    }
}
