use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4b97bb7d32040b47")]
pub struct CheckExpectedVaultsBalances {
    pub token_a_ata_balance: u64,
    pub token_b_ata_balance: u64,
}

pub struct CheckExpectedVaultsBalancesInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_a_ata: solana_sdk::pubkey::Pubkey,
    pub token_b_ata: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CheckExpectedVaultsBalances {
    type ArrangedAccounts = CheckExpectedVaultsBalancesInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [user, token_a_ata, token_b_ata, _remaining @ ..] = accounts.as_slice() else {
            return None;
        };

        Some(CheckExpectedVaultsBalancesInstructionAccounts {
            user: user.pubkey,
            token_a_ata: token_a_ata.pubkey,
            token_b_ata: token_b_ata.pubkey,
        })
    }
}
