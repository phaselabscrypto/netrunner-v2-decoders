use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0930dc6516f04ec8")]
pub struct GetPoolInfo {}

pub struct GetPoolInfoInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp: solana_sdk::pubkey::Pubkey,
    pub a_vault: solana_sdk::pubkey::Pubkey,
    pub b_vault: solana_sdk::pubkey::Pubkey,
    pub a_vault_lp_mint: solana_sdk::pubkey::Pubkey,
    pub b_vault_lp_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetPoolInfo {
    type ArrangedAccounts = GetPoolInfoInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let a_vault_lp = accounts.get(2)?;
        let b_vault_lp = accounts.get(3)?;
        let a_vault = accounts.get(4)?;
        let b_vault = accounts.get(5)?;
        let a_vault_lp_mint = accounts.get(6)?;
        let b_vault_lp_mint = accounts.get(7)?;

        Some(GetPoolInfoInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
        })
    }
}
