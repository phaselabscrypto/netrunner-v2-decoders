use super::AmmRoutingDecoder;
pub mod close_pool_config;
pub mod create_pool_config;
pub mod routing_v6;
pub mod smart_swap_v3;
pub mod update_pool_config;

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
pub enum AmmRoutingInstruction {
    ClosePoolConfig(close_pool_config::ClosePoolConfig),
    CreatePoolConfig(create_pool_config::CreatePoolConfig),
    RoutingV6(routing_v6::RoutingV6),
    SmartSwapV3(smart_swap_v3::SmartSwapV3),
    UpdatePoolConfig(update_pool_config::UpdatePoolConfig),
}

impl carbon_core::instruction::InstructionDecoder<'_> for AmmRoutingDecoder {
    type InstructionType = AmmRoutingInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            AmmRoutingInstruction::ClosePoolConfig => close_pool_config::ClosePoolConfig,
            AmmRoutingInstruction::CreatePoolConfig => create_pool_config::CreatePoolConfig,
            AmmRoutingInstruction::RoutingV6 => routing_v6::RoutingV6,
            AmmRoutingInstruction::SmartSwapV3 => smart_swap_v3::SmartSwapV3,
            AmmRoutingInstruction::UpdatePoolConfig => update_pool_config::UpdatePoolConfig,
        )
    }
}
