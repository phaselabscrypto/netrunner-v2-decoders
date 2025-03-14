

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x54d702acf100f5db")]
pub struct UpdateAuctionHouse{
    pub seller_fee_basis_points: Option<u16>,
    pub buyer_referral_bp: Option<u16>,
    pub seller_referral_bp: Option<u16>,
    pub requires_notary: Option<bool>,
    pub nprob: Option<u8>,
}

pub struct UpdateAuctionHouseInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub new_authority: solana_sdk::pubkey::Pubkey,
    pub treasury_withdrawal_destination: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAuctionHouse {
    type ArrangedAccounts = UpdateAuctionHouseInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let authority = accounts.get(2)?;
        let new_authority = accounts.get(3)?;
        let treasury_withdrawal_destination = accounts.get(4)?;
        let auction_house = accounts.get(5)?;
        let system_program = accounts.get(6)?;

        Some(UpdateAuctionHouseInstructionAccounts {
            payer: payer.pubkey,
            notary: notary.pubkey,
            authority: authority.pubkey,
            new_authority: new_authority.pubkey,
            treasury_withdrawal_destination: treasury_withdrawal_destination.pubkey,
            auction_house: auction_house.pubkey,
            system_program: system_program.pubkey,
        })
    }
}