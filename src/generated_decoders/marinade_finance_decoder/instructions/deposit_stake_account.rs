

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6e827329a466023b")]
pub struct DepositStakeAccount{
    pub validator_index: u32,
}

pub struct DepositStakeAccountInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub stake_list: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_authority: solana_sdk::pubkey::Pubkey,
    pub duplication_flag: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub mint_to: solana_sdk::pubkey::Pubkey,
    pub msol_mint_authority: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositStakeAccount {
    type ArrangedAccounts = DepositStakeAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let validator_list = accounts.get(1)?;
        let stake_list = accounts.get(2)?;
        let stake_account = accounts.get(3)?;
        let stake_authority = accounts.get(4)?;
        let duplication_flag = accounts.get(5)?;
        let rent_payer = accounts.get(6)?;
        let msol_mint = accounts.get(7)?;
        let mint_to = accounts.get(8)?;
        let msol_mint_authority = accounts.get(9)?;
        let clock = accounts.get(10)?;
        let rent = accounts.get(11)?;
        let system_program = accounts.get(12)?;
        let token_program = accounts.get(13)?;
        let stake_program = accounts.get(14)?;

        Some(DepositStakeAccountInstructionAccounts {
            state: state.pubkey,
            validator_list: validator_list.pubkey,
            stake_list: stake_list.pubkey,
            stake_account: stake_account.pubkey,
            stake_authority: stake_authority.pubkey,
            duplication_flag: duplication_flag.pubkey,
            rent_payer: rent_payer.pubkey,
            msol_mint: msol_mint.pubkey,
            mint_to: mint_to.pubkey,
            msol_mint_authority: msol_mint_authority.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            stake_program: stake_program.pubkey,
        })
    }
}