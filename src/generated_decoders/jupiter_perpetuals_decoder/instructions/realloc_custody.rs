use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7b3a6d8b8507e1c8")]
pub struct ReallocCustody {}

pub struct ReallocCustodyInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub custody: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReallocCustody {
    type ArrangedAccounts = ReallocCustodyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, custody, system_program, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ReallocCustodyInstructionAccounts {
            keeper: keeper.pubkey,
            custody: custody.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
