use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe19ba7aa1d91a55a")]
pub struct InitProtocolFee {}

pub struct InitProtocolFeeInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitProtocolFee {
    type ArrangedAccounts = InitProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let protocol_fee_account = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(InitProtocolFeeInstructionAccounts {
            payer: payer.pubkey,
            protocol_fee_account: protocol_fee_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
