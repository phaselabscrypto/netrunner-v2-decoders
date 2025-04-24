use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd02206935fda31a0")]
pub struct DepositNftT22 {}

pub struct DepositNftT22InstructionAccounts {
    pub transfer: solana_sdk::pubkey::Pubkey,
    pub t22: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub owner_ta: solana_sdk::pubkey::Pubkey,
    pub pool_ta: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositNftT22 {
    type ArrangedAccounts = DepositNftT22InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let transfer = accounts.get(0)?;
        let t22 = accounts.get(1)?;
        let nft_receipt = accounts.get(2)?;
        let owner_ta = accounts.get(3)?;
        let pool_ta = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let associated_token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(DepositNftT22InstructionAccounts {
            transfer: transfer.pubkey,
            t22: t22.pubkey,
            nft_receipt: nft_receipt.pubkey,
            owner_ta: owner_ta.pubkey,
            pool_ta: pool_ta.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
