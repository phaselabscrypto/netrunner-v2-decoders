use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum DynamicRouteV1CandidateAction {
    SolFi(SolFiDynamicRouteV1Options),
    Rubicon(RubiconDynamicRouteV1Options),
    TesseraV(TesseraVDynamicRouteV1Options),
    HumidiFi(HumidiFiDynamicRouteV1Options),
    SolFiV2(SolFiV2DynamicRouteV1Options),
    Mozart(MozartDynamicRouteV1Options),
    ObricV2(ObricV2DynamicRouteV1Options),
    Nexus(NexusDynamicRouteV1Options),
}
