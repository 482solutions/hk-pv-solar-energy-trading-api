use crate::blockchain::westmint::WestmintApi;
use crate::{error::Error, models::energy::EnergyNFT};

#[derive(Clone)]
pub struct FilesStorageBackend {
    api: WestmintApi,
}

impl FilesStorageBackend {
    pub async fn new() -> FilesStorageBackend {
        FilesStorageBackend {
            api: WestmintApi::new().await,
        }
    }

    pub async fn get_nfts_for_sale(&self) -> Result<Vec<EnergyNFT>, Error> {
        match self.api.get_nft_metadata_all(482).await {
            Ok(response) => Ok(vec![]),
            _ => Err(Error::InternalError),
        }
    }

    pub async fn sell_nft(&self, collection_id: u32, item_id: u32) -> Result<(), Error> {
        //unimplemented!();
        Ok(())
    }
}
