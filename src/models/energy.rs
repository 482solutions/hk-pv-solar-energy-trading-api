use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EnergyNFTMetadata {
    pub power_mw: f32,
    pub owner_addr: String,
    #[serde(rename(serialize = "price_pvse", deserialize = "price"))]
    pub price_pvse: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnergyNFT {
    pub(crate) storage_key: String,
    pub(crate) metadata: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EnergyNFTDTO {
    pub(crate) storage_key: String,
    pub(crate) metadata: EnergyNFTMetadata,
}
