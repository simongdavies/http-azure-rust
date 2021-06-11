use azure_core::HttpClient;
use azure_storage::{blob::prelude::*, core::prelude::*};
use bytes::Bytes;
use std::{collections::HashMap, error::Error, result, sync::Arc};

pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

pub async fn write_blob(
    container: String,
    blob: String,
    sa: String,
    key: String,
    bytes: Vec<u8>,
    http_client: Arc<Box<dyn HttpClient>>,
) -> Result<()> {
    let blob_client = StorageAccountClient::new_access_key(http_client.clone(), sa, key)
        .as_storage_client()
        .as_container_client(container)
        .as_blob_client(blob);

    println!("Writing {} bytes.", bytes.len());

    blob_client
        .put_block_blob(bytes)
        .content_type("text/plain")
        .execute()
        .await?;

    Ok(())
}

pub async fn read_blob(
    container: String,
    blob: String,
    sa: String,
    key: String,
    http_client: Arc<Box<dyn HttpClient>>,
) -> Result<Bytes> {
    let blob_client = StorageAccountClient::new_access_key(http_client.clone(), sa, key)
        .as_storage_client()
        .as_container_client(container)
        .as_blob_client(blob);

    Ok(Bytes::from(
        blob_client.get().execute().await?.data.to_vec(),
    ))
}

pub fn query_to_hash_map() -> Result<HashMap<String, String>> {
  let mut res = HashMap::new();

  let query = std::env::var("QUERY_STRING")?;
  for pair in query.split("&").collect::<Vec<&str>>() {
      if pair.len() == 0 {
          return Ok(res);
      }
      let q: Vec<&str> = pair.split("=").collect();
      res.insert(q[0].to_string(), q[1].to_string());
  }

  Ok(res)
}