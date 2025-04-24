use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub token_amount: u64,
    pub minimum_lp_token_amount: u64,
}

pub struct DepositInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub user_token: solana_sdk::pubkey::Pubkey,
    pub user_lp: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let token_vault = accounts.get(1)?;
        let lp_mint = accounts.get(2)?;
        let user_token = accounts.get(3)?;
        let user_lp = accounts.get(4)?;
        let user = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(DepositInstructionAccounts {
            vault: vault.pubkey,
            token_vault: token_vault.pubkey,
            lp_mint: lp_mint.pubkey,
            user_token: user_token.pubkey,
            user_lp: user_lp.pubkey,
            user: user.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
