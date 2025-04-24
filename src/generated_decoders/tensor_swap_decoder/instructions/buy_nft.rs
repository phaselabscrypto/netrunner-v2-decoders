use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x60001cbe316b53de")]
pub struct BuyNft {
    pub config: PoolConfig,
    pub max_price: u64,
    pub rules_acc_present: bool,
    pub authorization_data: Option<AuthorizationDataLocal>,
    pub optional_royalty_pct: Option<u16>,
}

pub struct BuyNftInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub nft_buyer_acc: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_metadata: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub sol_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub nft_edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub dest_token_record: solana_sdk::pubkey::Pubkey,
    pub pnft_shared: solana_sdk::pubkey::Pubkey,
    pub auth_rules: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyNft {
    type ArrangedAccounts = BuyNftInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let fee_vault = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let whitelist = accounts.get(3)?;
        let nft_buyer_acc = accounts.get(4)?;
        let nft_mint = accounts.get(5)?;
        let nft_metadata = accounts.get(6)?;
        let nft_escrow = accounts.get(7)?;
        let nft_receipt = accounts.get(8)?;
        let sol_escrow = accounts.get(9)?;
        let owner = accounts.get(10)?;
        let buyer = accounts.get(11)?;
        let token_program = accounts.get(12)?;
        let associated_token_program = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let rent = accounts.get(15)?;
        let nft_edition = accounts.get(16)?;
        let owner_token_record = accounts.get(17)?;
        let dest_token_record = accounts.get(18)?;
        let pnft_shared = accounts.get(19)?;
        let auth_rules = accounts.get(20)?;
        let margin_account = accounts.get(21)?;
        let taker_broker = accounts.get(22)?;

        Some(BuyNftInstructionAccounts {
            tswap: tswap.pubkey,
            fee_vault: fee_vault.pubkey,
            pool: pool.pubkey,
            whitelist: whitelist.pubkey,
            nft_buyer_acc: nft_buyer_acc.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_metadata: nft_metadata.pubkey,
            nft_escrow: nft_escrow.pubkey,
            nft_receipt: nft_receipt.pubkey,
            sol_escrow: sol_escrow.pubkey,
            owner: owner.pubkey,
            buyer: buyer.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            pnft_shared: pnft_shared.pubkey,
            auth_rules: auth_rules.pubkey,
            margin_account: margin_account.pubkey,
            taker_broker: taker_broker.pubkey,
        })
    }
}
