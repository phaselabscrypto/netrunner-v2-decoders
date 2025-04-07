

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1960d39ba10ea8bc")]
pub struct RemoveValidator{
    pub index: u32,
    pub validator_vote: solana_sdk::pubkey::Pubkey,
}

pub struct RemoveValidatorInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub manager_authority: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub duplication_flag: solana_sdk::pubkey::Pubkey,
    pub operational_sol_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveValidator {
    type ArrangedAccounts = RemoveValidatorInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let manager_authority = accounts.get(1)?;
        let validator_list = accounts.get(2)?;
        let duplication_flag = accounts.get(3)?;
        let operational_sol_account = accounts.get(4)?;

        Some(RemoveValidatorInstructionAccounts {
            state: state.pubkey,
            manager_authority: manager_authority.pubkey,
            validator_list: validator_list.pubkey,
            duplication_flag: duplication_flag.pubkey,
            operational_sol_account: operational_sol_account.pubkey,
        })
    }
}