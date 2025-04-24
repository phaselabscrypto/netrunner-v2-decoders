use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01a633aa7f208dce")]
pub struct Resume {}

pub struct ResumeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub pause_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Resume {
    type ArrangedAccounts = ResumeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let pause_authority = accounts.get(1)?;

        Some(ResumeInstructionAccounts {
            state: state.pubkey,
            pause_authority: pause_authority.pubkey,
        })
    }
}
