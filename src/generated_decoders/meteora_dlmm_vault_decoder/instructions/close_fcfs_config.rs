use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x30b2d465178ae95a")]
pub struct CloseFcfsConfig {}

pub struct CloseFcfsConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub rent_receiver: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseFcfsConfig {
    type ArrangedAccounts = CloseFcfsConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let rent_receiver = accounts.get(2)?;

        Some(CloseFcfsConfigInstructionAccounts {
            config: config.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
        })
    }
}
