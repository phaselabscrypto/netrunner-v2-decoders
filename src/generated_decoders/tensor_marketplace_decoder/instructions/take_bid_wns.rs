

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x58057a58fa8b23d8")]
pub struct TakeBidWns{
    pub min_amount: u64,
}

pub struct TakeBidWnsInstructionAccounts {
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub maker_broker: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub nft_seller_acc: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub owner_ata_acc: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub tensorswap_program: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub mint_proof: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
    pub approve_account: solana_sdk::pubkey::Pubkey,
    pub distribution: solana_sdk::pubkey::Pubkey,
    pub wns_program: solana_sdk::pubkey::Pubkey,
    pub distribution_program: solana_sdk::pubkey::Pubkey,
    pub extra_metas: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidWns {
    type ArrangedAccounts = TakeBidWnsInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let tcomp = accounts.get(0)?;
        let seller = accounts.get(1)?;
        let bid_state = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let taker_broker = accounts.get(4)?;
        let maker_broker = accounts.get(5)?;
        let margin_account = accounts.get(6)?;
        let whitelist = accounts.get(7)?;
        let nft_seller_acc = accounts.get(8)?;
        let nft_mint = accounts.get(9)?;
        let owner_ata_acc = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let system_program = accounts.get(13)?;
        let tcomp_program = accounts.get(14)?;
        let tensorswap_program = accounts.get(15)?;
        let cosigner = accounts.get(16)?;
        let mint_proof = accounts.get(17)?;
        let rent_dest = accounts.get(18)?;
        let approve_account = accounts.get(19)?;
        let distribution = accounts.get(20)?;
        let wns_program = accounts.get(21)?;
        let distribution_program = accounts.get(22)?;
        let extra_metas = accounts.get(23)?;

        Some(TakeBidWnsInstructionAccounts {
            tcomp: tcomp.pubkey,
            seller: seller.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            margin_account: margin_account.pubkey,
            whitelist: whitelist.pubkey,
            nft_seller_acc: nft_seller_acc.pubkey,
            nft_mint: nft_mint.pubkey,
            owner_ata_acc: owner_ata_acc.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tensorswap_program: tensorswap_program.pubkey,
            cosigner: cosigner.pubkey,
            mint_proof: mint_proof.pubkey,
            rent_dest: rent_dest.pubkey,
            approve_account: approve_account.pubkey,
            distribution: distribution.pubkey,
            wns_program: wns_program.pubkey,
            distribution_program: distribution_program.pubkey,
            extra_metas: extra_metas.pubkey,
        })
    }
}