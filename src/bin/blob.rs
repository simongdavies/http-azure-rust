use azure_core::{HttpClient, WasiHttpClient};
use common::*;
use futures::executor::block_on;
use std::sync::Arc;
use std::str;

fn main() {
    println!("Content-Type: text/plain\n\n Hello from main()\n");
}

#[no_mangle]
pub fn writeblob() {
    init();
    println!("Content-Type: text/plain\n");
    let result = block_on(write());
    match result {
        Ok(_) => {}
        Err(err) => {
            println!("{:#?}", err)
        }
    }
}

#[no_mangle]
pub fn init() {
    extern "C" {
        fn __wasm_call_ctors();
    }
    unsafe { __wasm_call_ctors() };
}

#[no_mangle]
pub fn readblob() {
    init();
    println!("Content-Type: text/plain\n");

    let result = block_on(read());
    match result {
      Ok(_) => {}
      Err(err) => {
          println!("{:#?}", err)
      }
  }
}

pub async fn write() -> Result<()> {
    let (container, blob) = container_and_blob_from_query()?;
    let (sa, sa_key) = keys_from_env()?;
    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(WasiHttpClient {}));

    let mut buf = Vec::new();
    std::io::copy(&mut std::io::stdin(), &mut buf)?;

    write_blob(
        container.clone(),
        blob.clone(),
        sa,
        sa_key,
        buf,
        http_client.clone(),
    )
    .await?;

    Ok(())
}

pub async fn read() -> Result<()> {
    let (container, blob) = container_and_blob_from_query()?;
    let (sa, sa_key) = keys_from_env()?;
    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(WasiHttpClient {}));

    let bytes = read_blob(
        container.clone(),
        blob.clone(),
        sa,
        sa_key,
        http_client.clone(),
    )
    .await?;

    println!("{}",str::from_utf8(&bytes)?);

    Ok(())
}

fn container_and_blob_from_query() -> Result<(String, String)> {
    let qs = query_to_hash_map()?;
    let container = qs.get("container").unwrap().clone();
    let blob = qs.get("blob").unwrap().clone();

    Ok((container, blob))
}

fn keys_from_env() -> Result<(String, String)> {
    Ok((
        std::env::var("STORAGE_ACCOUNT")?,
        std::env::var("STORAGE_MASTER_KEY")?,
    ))
}
