use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00a4564c38480caa")]
pub struct WithdrawFromTreasury {
    pub amount: u64,
}

pub struct WithdrawFromTreasuryInstructionAccounts {
    pub treasury_withdrawal_destination: solana_sdk::pubkey::Pubkey,
    pub auction_house_treasury: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFromTreasury {
    type ArrangedAccounts = WithdrawFromTreasuryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let treasury_withdrawal_destination = accounts.get(0)?;
        let auction_house_treasury = accounts.get(1)?;
        let auction_house = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(WithdrawFromTreasuryInstructionAccounts {
            treasury_withdrawal_destination: treasury_withdrawal_destination.pubkey,
            auction_house_treasury: auction_house_treasury.pubkey,
            auction_house: auction_house.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
