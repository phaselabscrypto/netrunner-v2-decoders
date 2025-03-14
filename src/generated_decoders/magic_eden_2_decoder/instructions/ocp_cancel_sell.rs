

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x490437f6259b02a6")]
pub struct OcpCancelSell{
}

pub struct OcpCancelSellInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub token_ata: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub ocp_mint_state: solana_sdk::pubkey::Pubkey,
    pub ocp_policy: solana_sdk::pubkey::Pubkey,
    pub ocp_freeze_authority: solana_sdk::pubkey::Pubkey,
    pub ocp_program: solana_sdk::pubkey::Pubkey,
    pub cmt_program: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OcpCancelSell {
    type ArrangedAccounts = OcpCancelSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let program_as_signer = accounts.get(2)?;
        let token_ata = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let auction_house = accounts.get(6)?;
        let seller_trade_state = accounts.get(7)?;
        let ocp_mint_state = accounts.get(8)?;
        let ocp_policy = accounts.get(9)?;
        let ocp_freeze_authority = accounts.get(10)?;
        let ocp_program = accounts.get(11)?;
        let cmt_program = accounts.get(12)?;
        let instructions = accounts.get(13)?;
        let token_program = accounts.get(14)?;
        let system_program = accounts.get(15)?;
        let rent = accounts.get(16)?;

        Some(OcpCancelSellInstructionAccounts {
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            program_as_signer: program_as_signer.pubkey,
            token_ata: token_ata.pubkey,
            token_mint: token_mint.pubkey,
            metadata: metadata.pubkey,
            auction_house: auction_house.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            ocp_mint_state: ocp_mint_state.pubkey,
            ocp_policy: ocp_policy.pubkey,
            ocp_freeze_authority: ocp_freeze_authority.pubkey,
            ocp_program: ocp_program.pubkey,
            cmt_program: cmt_program.pubkey,
            instructions: instructions.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}