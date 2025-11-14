use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x728025a747e328b2")]
pub struct ReallocPool {}

pub struct ReallocPoolInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ReallocPool {
    type ArrangedAccounts = ReallocPoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, pool, system_program, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ReallocPoolInstructionAccounts {
            keeper: keeper.pubkey,
            pool: pool.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
