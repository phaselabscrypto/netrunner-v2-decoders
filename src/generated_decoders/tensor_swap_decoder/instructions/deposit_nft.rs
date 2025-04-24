use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5de284a68d093065")]
pub struct DepositNft {
    pub config: PoolConfig,
    pub authorization_data: Option<AuthorizationDataLocal>,
    pub rules_acc_present: bool,
}

pub struct DepositNftInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub nft_source: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_escrow: solana_sdk::pubkey::Pubkey,
    pub nft_receipt: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub nft_metadata: solana_sdk::pubkey::Pubkey,
    pub mint_proof: solana_sdk::pubkey::Pubkey,
    pub nft_edition: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub dest_token_record: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub pnft_shared: solana_sdk::pubkey::Pubkey,
    pub auth_rules: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositNft {
    type ArrangedAccounts = DepositNftInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let whitelist = accounts.get(2)?;
        let nft_source = accounts.get(3)?;
        let nft_mint = accounts.get(4)?;
        let nft_escrow = accounts.get(5)?;
        let nft_receipt = accounts.get(6)?;
        let owner = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let rent = accounts.get(10)?;
        let nft_metadata = accounts.get(11)?;
        let mint_proof = accounts.get(12)?;
        let nft_edition = accounts.get(13)?;
        let owner_token_record = accounts.get(14)?;
        let dest_token_record = accounts.get(15)?;
        let associated_token_program = accounts.get(16)?;
        let pnft_shared = accounts.get(17)?;
        let auth_rules = accounts.get(18)?;

        Some(DepositNftInstructionAccounts {
            tswap: tswap.pubkey,
            pool: pool.pubkey,
            whitelist: whitelist.pubkey,
            nft_source: nft_source.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_escrow: nft_escrow.pubkey,
            nft_receipt: nft_receipt.pubkey,
            owner: owner.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            nft_metadata: nft_metadata.pubkey,
            mint_proof: mint_proof.pubkey,
            nft_edition: nft_edition.pubkey,
            owner_token_record: owner_token_record.pubkey,
            dest_token_record: dest_token_record.pubkey,
            associated_token_program: associated_token_program.pubkey,
            pnft_shared: pnft_shared.pubkey,
            auth_rules: auth_rules.pubkey,
        })
    }
}
