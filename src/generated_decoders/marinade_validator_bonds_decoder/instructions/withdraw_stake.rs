use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9908168a69b05742")]
pub struct WithdrawStake {}

pub struct WithdrawStakeInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub operator_authority: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_to: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawStake {
    type ArrangedAccounts = WithdrawStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let operator_authority = accounts.get(1)?;
        let settlement = accounts.get(2)?;
        let stake_account = accounts.get(3)?;
        let bonds_withdrawer_authority = accounts.get(4)?;
        let withdraw_to = accounts.get(5)?;
        let stake_history = accounts.get(6)?;
        let clock = accounts.get(7)?;
        let stake_program = accounts.get(8)?;
        let event_authority = accounts.get(9)?;
        let program = accounts.get(10)?;

        Some(WithdrawStakeInstructionAccounts {
            config: config.pubkey,
            operator_authority: operator_authority.pubkey,
            settlement: settlement.pubkey,
            stake_account: stake_account.pubkey,
            bonds_withdrawer_authority: bonds_withdrawer_authority.pubkey,
            withdraw_to: withdraw_to.pubkey,
            stake_history: stake_history.pubkey,
            clock: clock.pubkey,
            stake_program: stake_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
