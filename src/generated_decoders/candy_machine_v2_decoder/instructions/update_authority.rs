use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x202e401c954bf358")]
pub struct UpdateAuthority {
    pub new_authority: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateAuthorityInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAuthority {
    type ArrangedAccounts = UpdateAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let wallet = accounts.get(2)?;

        Some(UpdateAuthorityInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            authority: authority.pubkey,
            wallet: wallet.pubkey,
        })
    }
}
