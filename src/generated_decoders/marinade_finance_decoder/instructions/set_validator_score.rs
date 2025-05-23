use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6529ce21d86f194e")]
pub struct SetValidatorScore {
    pub index: u32,
    pub validator_vote: solana_sdk::pubkey::Pubkey,
    pub score: u32,
}

pub struct SetValidatorScoreInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub manager_authority: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetValidatorScore {
    type ArrangedAccounts = SetValidatorScoreInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let manager_authority = accounts.get(1)?;
        let validator_list = accounts.get(2)?;

        Some(SetValidatorScoreInstructionAccounts {
            state: state.pubkey,
            manager_authority: manager_authority.pubkey,
            validator_list: validator_list.pubkey,
        })
    }
}
