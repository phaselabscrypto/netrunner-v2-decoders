

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdd42f29ff9ce86f1")]
pub struct CreateAuctionHouse{
    pub bump: u8,
    pub treasury_bump: u8,
    pub seller_fee_basis_points: u16,
    pub buyer_referral_bp: u16,
    pub seller_referral_bp: u16,
    pub requires_notary: bool,
    pub create_auction_house_nonce: u64,
}

pub struct CreateAuctionHouseInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub treasury_withdrawal_destination: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub auction_house_treasury: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateAuctionHouse {
    type ArrangedAccounts = CreateAuctionHouseInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let treasury_withdrawal_destination = accounts.get(3)?;
        let auction_house = accounts.get(4)?;
        let auction_house_treasury = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(CreateAuctionHouseInstructionAccounts {
            payer: payer.pubkey,
            notary: notary.pubkey,
            authority: authority.pubkey,
            treasury_withdrawal_destination: treasury_withdrawal_destination.pubkey,
            auction_house: auction_house.pubkey,
            auction_house_treasury: auction_house_treasury.pubkey,
            system_program: system_program.pubkey,
        })
    }
}