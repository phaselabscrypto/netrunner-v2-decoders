use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd45233a0e4507423")]
pub struct Redelegate {
    pub stake_index: u32,
    pub source_validator_index: u32,
    pub dest_validator_index: u32,
}

pub struct RedelegateInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_deposit_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub split_stake_account: solana_sdk::pubkey::Pubkey,
    pub split_stake_rent_payer: solana_sdk::pubkey::Pubkey,
    pub dest_validator_account: solana_sdk::pubkey::Pubkey,
    pub redelegate_stake_account: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Redelegate {
    type ArrangedAccounts = RedelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let validator_list = accounts.get(1)?;
        let stake_list = accounts.get(2)?;
        let stake_account = accounts.get(3)?;
        let stake_deposit_authority = accounts.get(4)?;
        let reserve_pda = accounts.get(5)?;
        let split_stake_account = accounts.get(6)?;
        let split_stake_rent_payer = accounts.get(7)?;
        let dest_validator_account = accounts.get(8)?;
        let redelegate_stake_account = accounts.get(9)?;
        let clock = accounts.get(10)?;
        let stake_history = accounts.get(11)?;
        let stake_config = accounts.get(12)?;
        let system_program = accounts.get(13)?;
        let stake_program = accounts.get(14)?;

        Some(RedelegateInstructionAccounts {
            state: state.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_deposit_authority: stake_deposit_authority.pubkey,
            reserve_pda: reserve_pda.pubkey,
            split_stake_account: split_stake_account.pubkey,
            split_stake_rent_payer: split_stake_rent_payer.pubkey,
            dest_validator_account: dest_validator_account.pubkey,
            redelegate_stake_account: redelegate_stake_account.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_config: stake_config.pubkey,
            system_program: system_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}
