use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5bf90fa51a81fe7d")]
pub struct SetActivationPoint {
    pub activation_point: u64,
}

pub struct SetActivationPointInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetActivationPoint {
    type ArrangedAccounts = SetActivationPointInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetActivationPointInstructionAccounts {
            distributor: distributor.pubkey,
            admin: admin.pubkey,
        })
    }
}
