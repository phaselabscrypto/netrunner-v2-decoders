use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09755de6dd04c7d4")]
pub struct ListT22 {
    pub price: u64,
}

pub struct ListT22InstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub nft_source: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub single_listing: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ListT22 {
    type ArrangedAccounts = ListT22InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let nft_source = accounts.get(1)?;
        let nft_mint = accounts.get(2)?;
        let nft_escrow = accounts.get(3)?;
        let single_listing = accounts.get(4)?;
        let owner = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let payer = accounts.get(8)?;

        Some(ListT22InstructionAccounts {
            tswap: tswap.pubkey,
            nft_source: nft_source.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_escrow: nft_escrow.pubkey,
            single_listing: single_listing.pubkey,
            owner: owner.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            payer: payer.pubkey,
        })
    }
}
