use crate::models::westmint::{PalletUniquesItemDetails, PalletUniquesItemMetadata};
use codec::Decode;
use subxt::storage::{StorageEntryKey, StorageKeyPrefix, StorageMapKey};
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};

const API_URL_WSS: &str = "wss://westmint-rpc.polkadot.io:443";

#[subxt::subxt(runtime_metadata_path = "artifacts/metadata.scale")]
pub mod polkadot {}

#[derive(Clone)]
pub struct WestmintApi {
    api: polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>,
}

impl WestmintApi {
    pub async fn new() -> WestmintApi {
        let mut client = ClientBuilder::new();
        client = client.set_url(API_URL_WSS);
        let api = client.build()
            .await
            .unwrap()
            .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

        WestmintApi { api }
    }

    pub async fn get_nft_details_all(
        &self,
        collection_id: u32,
    ) -> Result<Vec<PalletUniquesItemDetails>, Box<dyn std::error::Error>> {
        // Obtain the prefixed `twox_128("uniques") ++ twox_128("Asset")`
        let prefix = StorageKeyPrefix::new::<polkadot::uniques::storage::Asset>();
        let entry_key = StorageEntryKey::Map(vec![StorageMapKey::new(
            &collection_id,
            ::subxt::StorageHasher::Blake2_128Concat,
        )]);

        // The final query key is:
        let query_key = entry_key.final_key(prefix);
        println!("Query key: 0x{}", hex::encode(&query_key));

        // TODO : Here we need to use start_key + count to allow loading of some particular items
        // For now we just loading 100 entries
        let keys = self
            .api
            .client
            .rpc()
            .storage_keys_paged(Some(query_key), 100, None, None)
            .await?;

        let mut data: Vec<PalletUniquesItemDetails> = Vec::new();
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = self
                .api
                .client
                .storage()
                .fetch_raw(key.clone(), None)
                .await?
            {
                let item: PalletUniquesItemDetails = Decode::decode(&mut &storage_data.0[..])?;
                println!("value: {:?}", item);
                data.push(item);
            }
        }

        Ok(data)
    }

    pub async fn get_nft_details_by_id(
        &self,
        collection_id: u32,
        item_id: u32,
    ) -> Result<PalletUniquesItemDetails, Box<dyn std::error::Error>> {
        // Obtain the prefixed `twox_128("uniques") ++ twox_128("Asset")`
        let prefix = StorageKeyPrefix::new::<polkadot::uniques::storage::Asset>();
        let entry_key = StorageEntryKey::Map(vec![
            StorageMapKey::new(&collection_id, ::subxt::StorageHasher::Blake2_128Concat),
            StorageMapKey::new(&item_id, ::subxt::StorageHasher::Blake2_128Concat),
        ]);

        // The final query key is:
        let query_key = entry_key.final_key(prefix);
        println!("Query key: 0x{}", hex::encode(&query_key));

        let storage_data = self
            .api
            .client
            .rpc()
            .storage(&query_key, None)
            .await?
            .unwrap();
        let data: PalletUniquesItemDetails = Decode::decode(&mut &storage_data.0[..])?;
        println!("Value data {:?}", data);

        Ok(data)
    }

    pub async fn get_nft_metadata_all(
        &self,
        collection_id: u32,
    ) -> Result<Vec<PalletUniquesItemMetadata>, Box<dyn std::error::Error>> {
        // Obtain the prefixed `twox_128("uniques") ++ twox_128("InstanceMetadataOf")`
        let prefix = StorageKeyPrefix::new::<polkadot::uniques::storage::InstanceMetadataOf>();
        let entry_key = StorageEntryKey::Map(vec![StorageMapKey::new(
            &collection_id,
            ::subxt::StorageHasher::Blake2_128Concat,
        )]);

        // The final query key is:
        let query_key = entry_key.final_key(prefix);
        println!("Query key: 0x{}", hex::encode(&query_key));

        // TODO : Here we need to use start_key + count to allow loading of some particular items
        // For now we just loading 100 entries
        let keys = self
            .api
            .client
            .rpc()
            .storage_keys_paged(Some(query_key), 100, None, None)
            .await?;

        let mut data: Vec<PalletUniquesItemMetadata> = Vec::new();
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = self
                .api
                .client
                .storage()
                .fetch_raw(key.clone(), None)
                .await?
            {
                let item: PalletUniquesItemMetadata = Decode::decode(&mut &storage_data.0[..])?;
                println!("value: {:?}", item);
                data.push(item);
            }
        }

        Ok(data)
    }

    pub async fn get_nft_metadata_by_id(
        &self,
        collection_id: u32,
        item_id: u32,
    ) -> Result<PalletUniquesItemMetadata, Box<dyn std::error::Error>> {
        // Obtain the prefixed `twox_128("uniques") ++ twox_128("InstanceMetadataOf")`
        let prefix = StorageKeyPrefix::new::<polkadot::uniques::storage::InstanceMetadataOf>();
        let entry_key = StorageEntryKey::Map(vec![
            StorageMapKey::new(&collection_id, ::subxt::StorageHasher::Blake2_128Concat),
            StorageMapKey::new(&item_id, ::subxt::StorageHasher::Blake2_128Concat),
        ]);

        // The final query key is:
        let query_key = entry_key.final_key(prefix);
        println!("Query key: 0x{}", hex::encode(&query_key));

        let storage_data = self
            .api
            .client
            .rpc()
            .storage(&query_key, None)
            .await?
            .unwrap();
        let data: PalletUniquesItemMetadata = Decode::decode(&mut &storage_data.0[..])?;
        println!("Value data {:?}", data);

        Ok(data)
    }
}
