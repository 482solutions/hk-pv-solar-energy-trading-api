use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnergyNFT {
    pub(crate) collection_id: u32,
    pub(crate) item_id: u32,
    pub(crate) metadata: String,
}
