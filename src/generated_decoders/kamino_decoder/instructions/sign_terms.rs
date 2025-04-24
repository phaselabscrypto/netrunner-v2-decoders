use carbon_core::{borsh, CarbonDeserialize};
use serde_with::{serde_as, Bytes};

#[serde_as]
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe22aae8f909f8b01")]
pub struct SignTerms {
    #[serde_as(as = "Bytes")]
    pub signature: [u8; 64],
}

pub struct SignTermsInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub owner_signature_state: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SignTerms {
    type ArrangedAccounts = SignTermsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let owner_signature_state = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let rent = accounts.get(3)?;

        Some(SignTermsInstructionAccounts {
            owner: owner.pubkey,
            owner_signature_state: owner_signature_state.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
