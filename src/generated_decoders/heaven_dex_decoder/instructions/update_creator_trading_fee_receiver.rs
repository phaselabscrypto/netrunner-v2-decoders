use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6e5c84f1f157819")]
pub struct UpdateCreatorTradingFeeReceiver {}

pub struct UpdateCreatorTradingFeeReceiverInstructionAccounts {
    pub swap: solana_sdk::pubkey::Pubkey,
    pub new_receiver: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateCreatorTradingFeeReceiver {
    type ArrangedAccounts = UpdateCreatorTradingFeeReceiverInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let swap = accounts.get(0)?;
        let new_receiver = accounts.get(1)?;

        Some(UpdateCreatorTradingFeeReceiverInstructionAccounts {
            swap: swap.pubkey,
            new_receiver: new_receiver.pubkey,
        })
    }
}
