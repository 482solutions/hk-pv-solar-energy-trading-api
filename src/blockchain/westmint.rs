use crate::models::westmint::{PalletUniquesItemDetails, PalletUniquesItemMetadata};
use codec::Decode;
use subxt::storage::{StorageEntryKey, StorageKeyPrefix, StorageMapKey};
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};

const API_URL_WSS: &str = "wss://westmint-rpc.polkadot.io:443";

#[subxt::subxt(runtime_metadata_path = "artifacts/westmint_metadata.scale")]
pub mod polkadot {}

#[derive(Clone)]
pub struct WestmintApi();

impl WestmintApi {
    pub async fn get_nft_details_all(
        collection_id: u32,
    ) -> Result<Vec<PalletUniquesItemDetails>, Box<dyn std::error::Error>> {
        let mut client = ClientBuilder::new();
        client = client.set_url(API_URL_WSS);
        let api = client.build()
            .await
            .unwrap()
            .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

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
        let keys = api
            .client
            .rpc()
            .storage_keys_paged(Some(query_key), 100, None, None)
            .await?;

        let mut data: Vec<PalletUniquesItemDetails> = Vec::new();
        for key in keys.iter() {
            println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = api.client.storage().fetch_raw(key.clone(), None).await? {
                let item: PalletUniquesItemDetails = Decode::decode(&mut &storage_data.0[..])?;
                println!("value: {:?}", item);
                data.push(item);
            }
        }

        Ok(data)
    }

    pub async fn get_nft_details_by_id(
        collection_id: u32,
        item_id: u32,
    ) -> Result<PalletUniquesItemDetails, Box<dyn std::error::Error>> {
        let mut client = ClientBuilder::new();
        client = client.set_url(API_URL_WSS);
        let api = client.build()
            .await
            .unwrap()
            .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

        // Obtain the prefixed `twox_128("uniques") ++ twox_128("Asset")`
        let prefix = StorageKeyPrefix::new::<polkadot::uniques::storage::Asset>();
        let entry_key = StorageEntryKey::Map(vec![
            StorageMapKey::new(&collection_id, ::subxt::StorageHasher::Blake2_128Concat),
            StorageMapKey::new(&item_id, ::subxt::StorageHasher::Blake2_128Concat),
        ]);

        // The final query key is:
        let query_key = entry_key.final_key(prefix);
        println!("Query key: 0x{}", hex::encode(&query_key));

        let storage_data = api.client.rpc().storage(&query_key, None).await?.unwrap();
        let data: PalletUniquesItemDetails = Decode::decode(&mut &storage_data.0[..])?;
        println!("Value data {:?}", data);

        Ok(data)
    }

    pub async fn get_nft_metadata_all(
        collection_id: u32,
    ) -> Result<Vec<(String, PalletUniquesItemMetadata)>, Box<dyn std::error::Error>> {
        let mut client = ClientBuilder::new();
        client = client.set_url(API_URL_WSS);
        let api = client.build()
            .await
            .unwrap()
            .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

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
        let keys = api
            .client
            .rpc()
            .storage_keys_paged(Some(query_key), 100, None, None)
            .await?;

        let mut data: Vec<(String, PalletUniquesItemMetadata)> = Vec::new();
        for key in keys.iter() {
            // println!("Key: 0x{}", hex::encode(&key));

            if let Some(storage_data) = api.client.storage().fetch_raw(key.clone(), None).await? {
                let item: PalletUniquesItemMetadata = Decode::decode(&mut &storage_data.0[..])?;
                // println!("value: {:?}", item);
                data.push((hex::encode(&key), item));
            }
        }

        Ok(data)
    }

    pub async fn get_nft_metadata_by_id(
        collection_id: u32,
        item_id: u32,
    ) -> Result<(String, PalletUniquesItemMetadata), Box<dyn std::error::Error>> {
        let mut client = ClientBuilder::new();
        client = client.set_url(API_URL_WSS);
        let api = client.build()
            .await
            .unwrap()
            .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

        // Obtain the prefixed `twox_128("uniques") ++ twox_128("InstanceMetadataOf")`
        let prefix = StorageKeyPrefix::new::<polkadot::uniques::storage::InstanceMetadataOf>();
        let entry_key = StorageEntryKey::Map(vec![
            StorageMapKey::new(&collection_id, ::subxt::StorageHasher::Blake2_128Concat),
            StorageMapKey::new(&item_id, ::subxt::StorageHasher::Blake2_128Concat),
        ]);

        // The final query key is:
        let query_key = entry_key.final_key(prefix);
        // println!("Query key: 0x{}", hex::encode(&query_key));

        let storage_data = api.client.rpc().storage(&query_key, None).await?.unwrap();
        let data: PalletUniquesItemMetadata = Decode::decode(&mut &storage_data.0[..])?;
        // println!("Value data {:?}", data);

        Ok((hex::encode(&query_key), data))
    }
}
