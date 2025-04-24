use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7a920410d90846d6")]
pub struct ExtCancelSell {}

pub struct ExtCancelSellInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub escrow_ata: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExtCancelSell {
    type ArrangedAccounts = ExtCancelSellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let wallet = accounts.get(1)?;
        let notary = accounts.get(2)?;
        let program_as_signer = accounts.get(3)?;
        let token_account = accounts.get(4)?;
        let escrow_ata = accounts.get(5)?;
        let token_mint = accounts.get(6)?;
        let auction_house = accounts.get(7)?;
        let seller_trade_state = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let associated_token_program = accounts.get(11)?;

        Some(ExtCancelSellInstructionAccounts {
            payer: payer.pubkey,
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            program_as_signer: program_as_signer.pubkey,
            token_account: token_account.pubkey,
            escrow_ata: escrow_ata.pubkey,
            token_mint: token_mint.pubkey,
            auction_house: auction_house.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
