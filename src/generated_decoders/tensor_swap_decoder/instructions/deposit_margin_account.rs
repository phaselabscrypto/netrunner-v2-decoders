use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbe55f23c775121c0")]
pub struct DepositMarginAccount {
    pub lamports: u64,
}

pub struct DepositMarginAccountInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositMarginAccount {
    type ArrangedAccounts = DepositMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let margin_account = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(DepositMarginAccountInstructionAccounts {
            tswap: tswap.pubkey,
            margin_account: margin_account.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
