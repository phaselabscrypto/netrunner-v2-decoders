use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x512ab398dd01b578")]
pub struct RiskCheckAndFee {
    pub minimum_out_amount: u64,
    pub platform_fee_bps: u8,
}

pub struct RiskCheckAndFeeInstructionAccounts {
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub user_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RiskCheckAndFee {
    type ArrangedAccounts = RiskCheckAndFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_ledger = accounts.get(0)?;
        let user_destination_token_account = accounts.get(1)?;
        let user_transfer_authority = accounts.get(2)?;
        let token_program = accounts.get(3)?;

        Some(RiskCheckAndFeeInstructionAccounts {
            token_ledger: token_ledger.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
