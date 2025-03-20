

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum GlobalConfigOption {
    EmergencyMode,
    BlockDeposit,
    BlockInvest,
    BlockWithdraw,
    BlockCollectFees,
    BlockCollectRewards,
    BlockSwapRewards,
    BlockSwapUnevenVaults,
    WithdrawalFeeBps,
    SwapDiscountBps,
    ActionsAuthority,
    DeprecatedTreasuryFeeVaults,
    AdminAuthority,
    BlockEmergencySwap,
    BlockLocalAdmin,
    UpdateTokenInfos,
    ScopeProgramId,
    ScopePriceId,
    MinPerformanceFeeBps,
    MinSwapUnevenSlippageToleranceBps,
    MinReferencePriceSlippageToleranceBps,
    ActionsAfterRebalanceDelaySeconds,
    TreasuryFeeVaultReceiver,
}


