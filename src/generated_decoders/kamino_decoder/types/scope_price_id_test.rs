use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum ScopePriceIdTest {
    SOL,
    ETH,
    BTC,
    SRM,
    RAY,
    FTT,
    MSOL,
    scnSOL_SOL,
    BNB,
    AVAX,
    DaoSOL_SOL,
    SaberMSOL_SOL,
    USDH,
    StSOL,
    CSOL_SOL,
    CETH_ETH,
    CBTC_BTC,
    CMSOL_SOL,
    wstETH,
    LDO,
    USDC,
    CUSDC_USDC,
    USDT,
    ORCA,
    MNDE,
    HBB,
    CORCA_ORCA,
    CSLND_SLND,
    CSRM_SRM,
    CRAY_RAY,
    CFTT_FTT,
    CSTSOL_STSOL,
    SLND,
    DAI,
    JSOL_SOL,
    USH,
    UXD,
    USDH_TWAP,
    USH_TWAP,
    UXD_TWAP,
    HDG,
    DUST,
    USDR,
    USDR_TWAP,
    RATIO,
    UXP,
    KUXDUSDCORCA,
    JITOSOL_SOL,
    SOL_EMA,
    ETH_EMA,
    BTC_EMA,
    SRM_EMA,
    RAY_EMA,
    FTT_EMA,
    MSOL_EMA,
    BNB_EMA,
    AVAX_EMA,
    STSOL_EMA,
    USDC_EMA,
    USDT_EMA,
    SLND_EMA,
    DAI_EMA,
    wstETH_TWAP,
    DUST_TWAP,
    BONK,
    BONK_TWAP,
    SAMO,
    SAMO_TWAP,
    BSOL,
    LaineSOL,
}
