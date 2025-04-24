use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5826ecd41fb912a6")]
pub struct EditSingleListing {
    pub price: u64,
}

pub struct EditSingleListingInstructionAccounts {
    pub single_listing: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EditSingleListing {
    type ArrangedAccounts = EditSingleListingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let single_listing = accounts.get(0)?;
        let nft_mint = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(EditSingleListingInstructionAccounts {
            single_listing: single_listing.pubkey,
            nft_mint: nft_mint.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
