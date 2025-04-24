use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9e357ff4c56ff19")]
pub struct BuyCore {
    pub max_amount: u64,
}

pub struct BuyCoreInstructionAccounts {
    pub tcomp: solana_sdk::pubkey::Pubkey,
    pub list_state: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub taker_broker: solana_sdk::pubkey::Pubkey,
    pub maker_broker: solana_sdk::pubkey::Pubkey,
    pub rent_dest: solana_sdk::pubkey::Pubkey,
    pub mpl_core_program: solana_sdk::pubkey::Pubkey,
    pub tcomp_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyCore {
    type ArrangedAccounts = BuyCoreInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tcomp = accounts.get(0)?;
        let list_state = accounts.get(1)?;
        let asset = accounts.get(2)?;
        let collection = accounts.get(3)?;
        let buyer = accounts.get(4)?;
        let payer = accounts.get(5)?;
        let owner = accounts.get(6)?;
        let taker_broker = accounts.get(7)?;
        let maker_broker = accounts.get(8)?;
        let rent_dest = accounts.get(9)?;
        let mpl_core_program = accounts.get(10)?;
        let tcomp_program = accounts.get(11)?;
        let system_program = accounts.get(12)?;

        Some(BuyCoreInstructionAccounts {
            tcomp: tcomp.pubkey,
            list_state: list_state.pubkey,
            asset: asset.pubkey,
            collection: collection.pubkey,
            buyer: buyer.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            rent_dest: rent_dest.pubkey,
            mpl_core_program: mpl_core_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
