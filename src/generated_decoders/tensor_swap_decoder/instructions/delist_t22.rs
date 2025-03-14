

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd8484912cc527b1a")]
pub struct DelistT22{
}

pub struct DelistT22InstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub nft_dest: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub single_listing: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DelistT22 {
    type ArrangedAccounts = DelistT22InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let nft_dest = accounts.get(1)?;
        let nft_mint = accounts.get(2)?;
        let nft_escrow = accounts.get(3)?;
        let single_listing = accounts.get(4)?;
        let owner = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let payer = accounts.get(10)?;

        Some(DelistT22InstructionAccounts {
            tswap: tswap.pubkey,
            nft_dest: nft_dest.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_escrow: nft_escrow.pubkey,
            single_listing: single_listing.pubkey,
            owner: owner.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            associated_token_program: associated_token_program.pubkey,
            payer: payer.pubkey,
        })
    }
}