

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1f013257ed656184")]
pub struct SetFeeAuthority{
}

pub struct SetFeeAuthorityInstructionAccounts {
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub new_fee_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeeAuthority {
    type ArrangedAccounts = SetFeeAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let fee_authority = accounts.get(0)?;
        let pool_account = accounts.get(1)?;
        let new_fee_authority = accounts.get(2)?;

        Some(SetFeeAuthorityInstructionAccounts {
            fee_authority: fee_authority.pubkey,
            pool_account: pool_account.pubkey,
            new_fee_authority: new_fee_authority.pubkey,
        })
    }
}