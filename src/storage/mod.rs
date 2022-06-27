use crate::{error::Error, models::energy::EnergyNFT};

#[derive(Clone)]
pub struct FilesStorageBackend {}

impl FilesStorageBackend {
    pub async fn get_nfts_for_sale(&self) -> Result<Vec<EnergyNFT>, Error> {
        // unimplemented!();
        Ok(vec![])
    }

    pub async fn sell_nft(&self, collection_id: u32, item_id: u32) -> Result<(), Error> {
        //unimplemented!();
        Ok(())
    }
}
