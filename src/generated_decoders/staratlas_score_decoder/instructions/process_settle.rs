use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdfd12b88b648fdfd")]
pub struct ProcessSettle {
    pub staking_bump: u8,
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
}

pub struct ProcessSettleInstructionAccounts {
    pub player_account: solana_sdk::pubkey::Pubkey,
    pub update_authority_account: solana_sdk::pubkey::Pubkey,
    pub ship_staking_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessSettle {
    type ArrangedAccounts = ProcessSettleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [player_account, update_authority_account, ship_staking_account, score_vars_ship_account, score_vars_account, clock, ship_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ProcessSettleInstructionAccounts {
            player_account: player_account.pubkey,
            update_authority_account: update_authority_account.pubkey,
            ship_staking_account: ship_staking_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            clock: clock.pubkey,
            ship_mint: ship_mint.pubkey,
        })
    }
}
