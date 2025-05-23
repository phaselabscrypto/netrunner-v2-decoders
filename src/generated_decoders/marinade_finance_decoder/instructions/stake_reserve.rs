use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x57d917b3cd197181")]
pub struct StakeReserve {
    pub validator_index: u32,
}

pub struct StakeReserveInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub validator_vote: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_deposit_authority: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub epoch_schedule: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StakeReserve {
    type ArrangedAccounts = StakeReserveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let validator_list = accounts.get(1)?;
        let stake_list = accounts.get(2)?;
        let validator_vote = accounts.get(3)?;
        let reserve_pda = accounts.get(4)?;
        let stake_account = accounts.get(5)?;
        let stake_deposit_authority = accounts.get(6)?;
        let rent_payer = accounts.get(7)?;
        let clock = accounts.get(8)?;
        let epoch_schedule = accounts.get(9)?;
        let rent = accounts.get(10)?;
        let stake_history = accounts.get(11)?;
        let stake_config = accounts.get(12)?;
        let system_program = accounts.get(13)?;
        let stake_program = accounts.get(14)?;

        Some(StakeReserveInstructionAccounts {
            state: state.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            validator_vote: validator_vote.pubkey,
            reserve_pda: reserve_pda.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            rent_payer: rent_payer.pubkey,
            clock: clock.pubkey,
            epoch_schedule: epoch_schedule.pubkey,
            rent: rent.pubkey,
            stake_history: stake_history.pubkey,
            stake_config: stake_config.pubkey,
            system_program: system_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
