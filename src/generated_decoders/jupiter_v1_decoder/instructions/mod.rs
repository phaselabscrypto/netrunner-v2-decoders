use super::JupiterDecoder;
pub mod create_open_orders;
pub mod initialize_token_ledger;
pub mod mercurial_exchange;
pub mod raydium_swap;
pub mod saber_exchange;
pub mod serum_swap;
pub mod set_token_ledger;
pub mod token_swap;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum JupiterInstruction {
    MercurialExchange(mercurial_exchange::MercurialExchange),
    SaberExchange(saber_exchange::SaberExchange),
    SerumSwap(serum_swap::SerumSwap),
    TokenSwap(token_swap::TokenSwap),
    RaydiumSwap(raydium_swap::RaydiumSwap),
    InitializeTokenLedger(initialize_token_ledger::InitializeTokenLedger),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    CreateOpenOrders(create_open_orders::CreateOpenOrders),
}

impl carbon_core::instruction::InstructionDecoder<'_> for JupiterDecoder {
    type InstructionType = JupiterInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterInstruction::MercurialExchange => mercurial_exchange::MercurialExchange,
            JupiterInstruction::SaberExchange => saber_exchange::SaberExchange,
            JupiterInstruction::SerumSwap => serum_swap::SerumSwap,
            JupiterInstruction::TokenSwap => token_swap::TokenSwap,
            JupiterInstruction::RaydiumSwap => raydium_swap::RaydiumSwap,
            JupiterInstruction::InitializeTokenLedger => initialize_token_ledger::InitializeTokenLedger,
            JupiterInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            JupiterInstruction::CreateOpenOrders => create_open_orders::CreateOpenOrders,
        )
    }
}
