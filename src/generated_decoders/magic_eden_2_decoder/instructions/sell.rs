use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x33e685a4017f83ad")]
pub struct Sell {
    pub seller_state_bump: u8,
    pub program_as_signer_bump: u8,
    pub buyer_price: u64,
    pub token_size: u64,
    pub seller_state_expiry: i64,
}

pub struct SellInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_ata: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub seller_referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub ata_program: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let token_account = accounts.get(2)?;
        let token_ata = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let authority = accounts.get(6)?;
        let auction_house = accounts.get(7)?;
        let seller_trade_state = accounts.get(8)?;
        let seller_referral = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let ata_program = accounts.get(12)?;
        let program_as_signer = accounts.get(13)?;
        let rent = accounts.get(14)?;

        Some(SellInstructionAccounts {
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            token_account: token_account.pubkey,
            token_ata: token_ata.pubkey,
            token_mint: token_mint.pubkey,
            metadata: metadata.pubkey,
            authority: authority.pubkey,
            auction_house: auction_house.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            seller_referral: seller_referral.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            ata_program: ata_program.pubkey,
            program_as_signer: program_as_signer.pubkey,
            rent: rent.pubkey,
        })
    }
}
