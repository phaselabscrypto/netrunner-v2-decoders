use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xee9965a9f3832401")]
pub struct SetOperator {
    pub new_operator: solana_sdk::pubkey::Pubkey,
}

pub struct SetOperatorInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetOperator {
    type ArrangedAccounts = SetOperatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetOperatorInstructionAccounts {
            distributor: distributor.pubkey,
            admin: admin.pubkey,
        })
    }
}
