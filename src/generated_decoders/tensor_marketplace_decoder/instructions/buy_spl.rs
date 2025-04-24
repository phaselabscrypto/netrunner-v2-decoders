use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4188feff3b82eaae")]
pub struct BuySpl {
    pub nonce: u64,
    pub index: u32,
    pub root: [u8; 32],
    pub meta_hash: [u8; 32],
    pub creator_shares: Vec<u8>,
    pub creator_verified: Vec<bool>,
    pub seller_fee_basis_points: u16,
    pub max_amount: u64,
    pub optional_royalty_pct: Option<u16>,
}

pub struct BuySplInstructionAccounts {
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub tcomp_ata: solana_sdk::pubkey::Pubkey,
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
    pub compression_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub bubblegum_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub payer_source: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub owner_dest: solana_sdk::pubkey::Pubkey,
    pub currency: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub taker_broker_ata: solana_sdk::pubkey::Pubkey,
    pub maker_broker: solana_sdk::pubkey::Pubkey,
    pub maker_broker_ata: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuySpl {
    type ArrangedAccounts = BuySplInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tcomp = accounts.get(0)?;
        let tcomp_ata = accounts.get(1)?;
        let tree_authority = accounts.get(2)?;
        let merkle_tree = accounts.get(3)?;
        let log_wrapper = accounts.get(4)?;
        let compression_program = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let bubblegum_program = accounts.get(7)?;
        let tcomp_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let list_state = accounts.get(11)?;
        let buyer = accounts.get(12)?;
        let payer = accounts.get(13)?;
        let payer_source = accounts.get(14)?;
        let owner = accounts.get(15)?;
        let owner_dest = accounts.get(16)?;
        let currency = accounts.get(17)?;
        let taker_broker = accounts.get(18)?;
        let taker_broker_ata = accounts.get(19)?;
        let maker_broker = accounts.get(20)?;
        let maker_broker_ata = accounts.get(21)?;
        let rent_dest = accounts.get(22)?;
        let rent_payer = accounts.get(23)?;

        Some(BuySplInstructionAccounts {
            tcomp: tcomp.pubkey,
            tcomp_ata: tcomp_ata.pubkey,
            tree_authority: tree_authority.pubkey,
            merkle_tree: merkle_tree.pubkey,
            log_wrapper: log_wrapper.pubkey,
            compression_program: compression_program.pubkey,
            system_program: system_program.pubkey,
            bubblegum_program: bubblegum_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            list_state: list_state.pubkey,
            buyer: buyer.pubkey,
            payer: payer.pubkey,
            payer_source: payer_source.pubkey,
            owner: owner.pubkey,
            owner_dest: owner_dest.pubkey,
            currency: currency.pubkey,
            taker_broker: taker_broker.pubkey,
            taker_broker_ata: taker_broker_ata.pubkey,
            maker_broker: maker_broker.pubkey,
            maker_broker_ata: maker_broker_ata.pubkey,
            rent_dest: rent_dest.pubkey,
            rent_payer: rent_payer.pubkey,
        })
    }
}
