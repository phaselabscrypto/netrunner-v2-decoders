use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb39271221e5c1a13")]
pub struct FundSettlement {}

pub struct FundSettlementInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub operator_authority: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub settlement_staker_authority: solana_sdk::pubkey::Pubkey,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub split_stake_account: solana_sdk::pubkey::Pubkey,
    pub split_stake_rent_payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub stake_config: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FundSettlement {
    type ArrangedAccounts = FundSettlementInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let vote_account = accounts.get(2)?;
        let settlement = accounts.get(3)?;
        let operator_authority = accounts.get(4)?;
        let stake_account = accounts.get(5)?;
        let settlement_staker_authority = accounts.get(6)?;
        let bonds_withdrawer_authority = accounts.get(7)?;
        let split_stake_account = accounts.get(8)?;
        let split_stake_rent_payer = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let stake_history = accounts.get(11)?;
        let clock = accounts.get(12)?;
        let rent = accounts.get(13)?;
        let stake_program = accounts.get(14)?;
        let stake_config = accounts.get(15)?;
        let event_authority = accounts.get(16)?;
        let program = accounts.get(17)?;

        Some(FundSettlementInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            vote_account: vote_account.pubkey,
            settlement: settlement.pubkey,
            operator_authority: operator_authority.pubkey,
            stake_account: stake_account.pubkey,
            settlement_staker_authority: settlement_staker_authority.pubkey,
            bonds_withdrawer_authority: bonds_withdrawer_authority.pubkey,
            split_stake_account: split_stake_account.pubkey,
            split_stake_rent_payer: split_stake_rent_payer.pubkey,
            system_program: system_program.pubkey,
            stake_history: stake_history.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            stake_program: stake_program.pubkey,
            stake_config: stake_config.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
