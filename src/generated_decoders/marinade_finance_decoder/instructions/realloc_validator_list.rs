use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd73bda855d8a3c7b")]
pub struct ReallocValidatorList {
    pub capacity: u32,
}

pub struct ReallocValidatorListInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
    pub rent_funds: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReallocValidatorList {
    type ArrangedAccounts = ReallocValidatorListInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let admin_authority = accounts.get(1)?;
        let validator_list = accounts.get(2)?;
        let rent_funds = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(ReallocValidatorListInstructionAccounts {
            state: state.pubkey,
            admin_authority: admin_authority.pubkey,
            validator_list: validator_list.pubkey,
            rent_funds: rent_funds.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
