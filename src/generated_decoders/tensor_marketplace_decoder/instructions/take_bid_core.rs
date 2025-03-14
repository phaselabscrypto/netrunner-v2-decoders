

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfa29f8143da11b8d")]
pub struct TakeBidCore{
    pub min_amount: u64,
}

pub struct TakeBidCoreInstructionAccounts {
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub bid_state: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub maker_broker: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub tensorswap_program: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub mint_proof: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidCore {
    type ArrangedAccounts = TakeBidCoreInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let tcomp = accounts.get(0)?;
        let seller = accounts.get(1)?;
        let bid_state = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let taker_broker = accounts.get(4)?;
        let maker_broker = accounts.get(5)?;
        let margin_account = accounts.get(6)?;
        let whitelist = accounts.get(7)?;
        let asset = accounts.get(8)?;
        let collection = accounts.get(9)?;
        let mpl_core_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let tcomp_program = accounts.get(12)?;
        let tensorswap_program = accounts.get(13)?;
        let cosigner = accounts.get(14)?;
        let mint_proof = accounts.get(15)?;
        let rent_dest = accounts.get(16)?;

        Some(TakeBidCoreInstructionAccounts {
            tcomp: tcomp.pubkey,
            seller: seller.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            margin_account: margin_account.pubkey,
            whitelist: whitelist.pubkey,
            asset: asset.pubkey,
            collection: collection.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tensorswap_program: tensorswap_program.pubkey,
            cosigner: cosigner.pubkey,
            mint_proof: mint_proof.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}