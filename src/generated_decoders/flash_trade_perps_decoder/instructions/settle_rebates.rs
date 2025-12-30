use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0cc0edff514411c1")]
pub struct SettleRebates {}

pub struct SettleRebatesInstructionAccounts {
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub reward_custody_oracle_account: solana_sdk::pubkey::Pubkey,
    pub reward_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub rebate_vault: solana_sdk::pubkey::Pubkey,
    pub rebate_token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleRebates {
    type ArrangedAccounts = SettleRebatesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [transfer_authority, perpetuals, pool, reward_custody, reward_custody_oracle_account, reward_custody_token_account, rebate_vault, rebate_token_account, token_mint, token_program, event_authority, program, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SettleRebatesInstructionAccounts {
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            reward_custody: reward_custody.pubkey,
            reward_custody_oracle_account: reward_custody_oracle_account.pubkey,
            reward_custody_token_account: reward_custody_token_account.pubkey,
            rebate_vault: rebate_vault.pubkey,
            rebate_token_account: rebate_token_account.pubkey,
            token_mint: token_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
