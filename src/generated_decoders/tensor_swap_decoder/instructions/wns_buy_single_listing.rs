

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1c0e84cfd4f879c7")]
pub struct WnsBuySingleListing{
    pub max_price: u64,
}

pub struct WnsBuySingleListingInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub single_listing: solana_sdk::pubkey::Pubkey,
    pub nft_buyer_acc: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub approve_account: solana_sdk::pubkey::Pubkey,
    pub distribution: solana_sdk::pubkey::Pubkey,
    pub wns_program: solana_sdk::pubkey::Pubkey,
    pub distribution_program: solana_sdk::pubkey::Pubkey,
    pub extra_metas: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WnsBuySingleListing {
    type ArrangedAccounts = WnsBuySingleListingInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let fee_vault = accounts.get(1)?;
        let single_listing = accounts.get(2)?;
        let nft_buyer_acc = accounts.get(3)?;
        let nft_mint = accounts.get(4)?;
        let nft_escrow = accounts.get(5)?;
        let owner = accounts.get(6)?;
        let buyer = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let taker_broker = accounts.get(11)?;
        let approve_account = accounts.get(12)?;
        let distribution = accounts.get(13)?;
        let wns_program = accounts.get(14)?;
        let distribution_program = accounts.get(15)?;
        let extra_metas = accounts.get(16)?;

        Some(WnsBuySingleListingInstructionAccounts {
            tswap: tswap.pubkey,
            fee_vault: fee_vault.pubkey,
            single_listing: single_listing.pubkey,
            nft_buyer_acc: nft_buyer_acc.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_escrow: nft_escrow.pubkey,
            owner: owner.pubkey,
            buyer: buyer.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            taker_broker: taker_broker.pubkey,
            approve_account: approve_account.pubkey,
            distribution: distribution.pubkey,
            wns_program: wns_program.pubkey,
            distribution_program: distribution_program.pubkey,
            extra_metas: extra_metas.pubkey,
        })
    }
}