use super::OnchainLabsDexV1Decoder;

pub mod claim;
pub mod cpi_event;
pub mod create_token_account;
pub mod create_token_account_with_seed;
pub mod proxy_swap;
pub mod swap;
pub mod swap_tob_v3;
pub mod swap_tob_v3_enhanced;
pub mod swap_tob_v3_with_receiver;
pub mod swap_v3;
pub mod swap_v3_with_cpi_event;
pub mod wrap_unwrap_v3;
pub mod wrap_unwrap_v3_with_receiver;

pub use self::claim::*;
pub use self::cpi_event::*;
pub use self::create_token_account::*;
pub use self::create_token_account_with_seed::*;
pub use self::proxy_swap::*;
pub use self::swap::*;
pub use self::swap_tob_v3::*;
pub use self::swap_tob_v3_enhanced::*;
pub use self::swap_tob_v3_with_receiver::*;
pub use self::swap_v3::*;
pub use self::swap_v3_with_cpi_event::*;
pub use self::wrap_unwrap_v3::*;
pub use self::wrap_unwrap_v3_with_receiver::*;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum OnchainLabsDexV1Instruction {
    Claim(Claim),
    CreateTokenAccount(CreateTokenAccount),
    CreateTokenAccountWithSeed(CreateTokenAccountWithSeed),
    ProxySwap(ProxySwap),
    Swap(Swap),
    SwapTobV3(SwapTobV3),
    SwapTobV3Enhanced(SwapTobV3Enhanced),
    SwapTobV3WithReceiver(SwapTobV3WithReceiver),
    SwapV3(SwapV3),
    SwapV3WithCpiEvent(SwapV3WithCpiEvent),
    WrapUnwrapV3(WrapUnwrapV3),
    WrapUnwrapV3WithReceiver(WrapUnwrapV3WithReceiver),
    CpiEvent(CpiEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for OnchainLabsDexV1Decoder {
    type InstructionType = OnchainLabsDexV1Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            OnchainLabsDexV1Instruction::Claim => claim::Claim,
            OnchainLabsDexV1Instruction::CreateTokenAccount => create_token_account::CreateTokenAccount,
            OnchainLabsDexV1Instruction::CreateTokenAccountWithSeed => create_token_account_with_seed::CreateTokenAccountWithSeed,
            OnchainLabsDexV1Instruction::ProxySwap => proxy_swap::ProxySwap,
            OnchainLabsDexV1Instruction::Swap => swap::Swap,
            OnchainLabsDexV1Instruction::SwapTobV3 => swap_tob_v3::SwapTobV3,
            OnchainLabsDexV1Instruction::SwapTobV3Enhanced => swap_tob_v3_enhanced::SwapTobV3Enhanced,
            OnchainLabsDexV1Instruction::SwapTobV3WithReceiver => swap_tob_v3_with_receiver::SwapTobV3WithReceiver,
            OnchainLabsDexV1Instruction::SwapV3 => swap_v3::SwapV3,
            OnchainLabsDexV1Instruction::SwapV3WithCpiEvent => swap_v3_with_cpi_event::SwapV3WithCpiEvent,
            OnchainLabsDexV1Instruction::WrapUnwrapV3 => wrap_unwrap_v3::WrapUnwrapV3,
            OnchainLabsDexV1Instruction::WrapUnwrapV3WithReceiver => wrap_unwrap_v3_with_receiver::WrapUnwrapV3WithReceiver,
        );

        if let Some(decoded) = cpi_event::CpiEvent::decode(&instruction.data) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: OnchainLabsDexV1Instruction::CpiEvent(decoded),
                accounts: instruction.accounts.clone(),
            });
        }

        None
    }
}
