use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5a5f6b2acd7c32e1")]
pub struct Unstake {}

pub struct UnstakeInstructionAccounts {
    pub operator: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_info: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Unstake {
    type ArrangedAccounts = UnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [operator, perpetuals, pool, custody, transfer_authority, stake_account, stake_info, clock, stake_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UnstakeInstructionAccounts {
            operator: operator.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            transfer_authority: transfer_authority.pubkey,
            stake_account: stake_account.pubkey,
            stake_info: stake_info.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
