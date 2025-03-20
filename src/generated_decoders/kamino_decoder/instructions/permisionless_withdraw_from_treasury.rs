

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xa724204f61aab76c")]
pub struct PermisionlessWithdrawFromTreasury{
}

pub struct PermisionlessWithdrawFromTreasuryInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermisionlessWithdrawFromTreasury {
    type ArrangedAccounts = PermisionlessWithdrawFromTreasuryInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let global_config = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let treasury_fee_vault = accounts.get(3)?;
        let treasury_fee_vault_authority = accounts.get(4)?;
        let token_account = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(PermisionlessWithdrawFromTreasuryInstructionAccounts {
            signer: signer.pubkey,
            global_config: global_config.pubkey,
            mint: mint.pubkey,
            treasury_fee_vault: treasury_fee_vault.pubkey,
            treasury_fee_vault_authority: treasury_fee_vault_authority.pubkey,
            token_account: token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}