use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1be58069737db497")]
pub struct WithdrawTswapFees {
    pub amount: u64,
}

pub struct WithdrawTswapFeesInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawTswapFees {
    type ArrangedAccounts = WithdrawTswapFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let cosigner = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let destination = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(WithdrawTswapFeesInstructionAccounts {
            tswap: tswap.pubkey,
            cosigner: cosigner.pubkey,
            owner: owner.pubkey,
            destination: destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
