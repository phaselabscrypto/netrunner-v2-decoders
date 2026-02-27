use super::OnchainLabsDexV2Decoder;

pub mod claim;
pub mod cpi_event;
pub mod create_token_account;
pub mod create_token_account_with_seed;
pub mod init_token_ledger;
pub mod proxy_swap;
pub mod set_token_ledger;
pub mod swap;
pub mod swap_tob;
pub mod swap_tob_enhanced;
pub mod swap_tob_v2;
pub mod swap_tob_with_receiver;
pub mod swap_tob_with_receiver_token_ledger;
pub mod swap_tob_with_token_ledger;
pub mod swap_toc;
pub mod swap_toc_v2;
pub mod wrap_unwrap;
pub mod wrap_unwrap_with_receiver;

pub use self::claim::*;
pub use self::cpi_event::*;
pub use self::create_token_account::*;
pub use self::create_token_account_with_seed::*;
pub use self::init_token_ledger::*;
pub use self::proxy_swap::*;
pub use self::set_token_ledger::*;
pub use self::swap::*;
pub use self::swap_tob::*;
pub use self::swap_tob_enhanced::*;
pub use self::swap_tob_v2::*;
pub use self::swap_tob_with_receiver::*;
pub use self::swap_tob_with_receiver_token_ledger::*;
pub use self::swap_tob_with_token_ledger::*;
pub use self::swap_toc::*;
pub use self::swap_toc_v2::*;
pub use self::wrap_unwrap::*;
pub use self::wrap_unwrap_with_receiver::*;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum OnchainLabsDexV2Instruction {
    Claim(Claim),
    CreateTokenAccount(CreateTokenAccount),
    CreateTokenAccountWithSeed(CreateTokenAccountWithSeed),
    InitTokenLedger(InitTokenLedger),
    ProxySwap(ProxySwap),
    SetTokenLedger(SetTokenLedger),
    Swap(Swap),
    SwapTob(SwapTob),
    SwapTobEnhanced(SwapTobEnhanced),
    SwapTobV2(SwapTobV2),
    SwapTobWithReceiver(SwapTobWithReceiver),
    SwapTobWithReceiverTokenLedger(SwapTobWithReceiverTokenLedger),
    SwapTobWithTokenLedger(SwapTobWithTokenLedger),
    SwapToc(SwapToc),
    SwapTocV2(SwapTocV2),
    WrapUnwrap(WrapUnwrap),
    WrapUnwrapWithReceiver(WrapUnwrapWithReceiver),
    CpiEvent(CpiEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for OnchainLabsDexV2Decoder {
    type InstructionType = OnchainLabsDexV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            OnchainLabsDexV2Instruction::Claim => claim::Claim,
            OnchainLabsDexV2Instruction::CreateTokenAccount => create_token_account::CreateTokenAccount,
            OnchainLabsDexV2Instruction::CreateTokenAccountWithSeed => create_token_account_with_seed::CreateTokenAccountWithSeed,
            OnchainLabsDexV2Instruction::InitTokenLedger => init_token_ledger::InitTokenLedger,
            OnchainLabsDexV2Instruction::ProxySwap => proxy_swap::ProxySwap,
            OnchainLabsDexV2Instruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            OnchainLabsDexV2Instruction::Swap => swap::Swap,
            OnchainLabsDexV2Instruction::SwapTob => swap_tob::SwapTob,
            OnchainLabsDexV2Instruction::SwapTobEnhanced => swap_tob_enhanced::SwapTobEnhanced,
            OnchainLabsDexV2Instruction::SwapTobV2 => swap_tob_v2::SwapTobV2,
            OnchainLabsDexV2Instruction::SwapTobWithReceiver => swap_tob_with_receiver::SwapTobWithReceiver,
            OnchainLabsDexV2Instruction::SwapTobWithReceiverTokenLedger => swap_tob_with_receiver_token_ledger::SwapTobWithReceiverTokenLedger,
            OnchainLabsDexV2Instruction::SwapTobWithTokenLedger => swap_tob_with_token_ledger::SwapTobWithTokenLedger,
            OnchainLabsDexV2Instruction::SwapToc => swap_toc::SwapToc,
            OnchainLabsDexV2Instruction::SwapTocV2 => swap_toc_v2::SwapTocV2,
            OnchainLabsDexV2Instruction::WrapUnwrap => wrap_unwrap::WrapUnwrap,
            OnchainLabsDexV2Instruction::WrapUnwrapWithReceiver => wrap_unwrap_with_receiver::WrapUnwrapWithReceiver,
        );

        if let Some(decoded) = cpi_event::CpiEvent::decode(&instruction.data) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: OnchainLabsDexV2Instruction::CpiEvent(decoded),
                accounts: instruction.accounts.clone(),
            });
        }

        None
    }
}
