use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd39f7a3dc3f6840f")]
pub struct CollectRebate {}

pub struct CollectRebateInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub rebate_vault: solana_sdk::pubkey::Pubkey,
    pub rebate_token_account: solana_sdk::pubkey::Pubkey,
    pub token_stake_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub receiving_token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectRebate {
    type ArrangedAccounts = CollectRebateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_token_account, perpetuals, transfer_authority, rebate_vault, rebate_token_account, token_stake_account, token_program, event_authority, program, receiving_token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectRebateInstructionAccounts {
            owner: owner.pubkey,
            receiving_token_account: receiving_token_account.pubkey,
            perpetuals: perpetuals.pubkey,
            transfer_authority: transfer_authority.pubkey,
            rebate_vault: rebate_vault.pubkey,
            rebate_token_account: rebate_token_account.pubkey,
            token_stake_account: token_stake_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            receiving_token_mint: receiving_token_mint.pubkey,
        })
    }
}
