use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x99d92214131de54b")]
pub struct SetClawbackReceiver {}

pub struct SetClawbackReceiverInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub new_clawback_account: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetClawbackReceiver {
    type ArrangedAccounts = SetClawbackReceiverInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, new_clawback_account, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetClawbackReceiverInstructionAccounts {
            distributor: distributor.pubkey,
            new_clawback_account: new_clawback_account.pubkey,
            admin: admin.pubkey,
        })
    }
}
