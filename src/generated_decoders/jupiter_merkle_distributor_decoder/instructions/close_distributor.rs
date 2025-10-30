use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xca38b48f2e686a70")]
pub struct CloseDistributor {}

pub struct CloseDistributorInstructionAccounts {
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseDistributor {
    type ArrangedAccounts = CloseDistributorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [distributor, token_vault, admin, destination_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseDistributorInstructionAccounts {
            distributor: distributor.pubkey,
            token_vault: token_vault.pubkey,
            admin: admin.pubkey,
            destination_token_account: destination_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
