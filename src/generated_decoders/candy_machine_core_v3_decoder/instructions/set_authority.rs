use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x85fa25156ea31a79")]
pub struct SetAuthority {
    pub new_authority: solana_sdk::pubkey::Pubkey,
}

pub struct SetAuthorityInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetAuthority {
    type ArrangedAccounts = SetAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;

        Some(SetAuthorityInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
        })
    }
}
