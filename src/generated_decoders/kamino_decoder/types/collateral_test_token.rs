

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum CollateralTestToken {
    USDC,
    USDH,
    SOL,
    ETH,
    BTC,
    MSOL,
    STSOL,
    USDT,
    ORCA,
    MNDE,
    HBB,
    JSOL,
    USH,
    DAI,
    LDO,
    SCNSOL,
    UXD,
    HDG,
    DUST,
    USDR,
    RATIO,
    UXP,
    JITOSOL,
    RAY,
    BONK,
    SAMO,
    LaineSOL,
    BSOL,
}


