# WAGI sample using Azure Blob storage

This is an _experimental_ repository containing WebAssembly modules running on
top of [WAGI][wagi] (WebAssembly Gateway Interface, which allows you to run
WebAssembly WASI binaries as HTTP handlers) and using Azure Blob storage.

> Note that this repository is built on experimental code, and it is not
> guaranteed to be maintained or supported.

## What does the example do?

 This module exports two functions, one reads the request body and creates/updates a blob in
  [Azure Blob Storage][bs], based on the container and blob name from the
  request's query string. 
  
  The second reads the contents of a blob based on the container and blob name from the
  request's query string.

  This example is a simplifled version of the blob sample in the azure-sdk-for-rust-wasi-samples repo  [here](https://github.com/deislabs/azure-sdk-for-rust-wasi-samples).

## Building 

The following tools are required to build and run the samples:

- Rust and Cargo, with the `wasm32-wasi` target configured.

```
$ rustup target add wasm32-wasi
$ cargo build --release --target wasm32-wasi --bin blob
    Finished release [optimized] target(s) in 0.09s
```

## Running

You can run this module using either the [WAGI server](https://github.com/deislabs/wagi) or [wagi-dotnet] (https://github.com/deislabs/wagi-dotnet).

## wagi-dotnet

The simplehttp example contains a [sample](https://github.com/deislabs/wagi-dotnet/tree/main/examples/simplehttp#readme) that runs this module. 

## WAGI server

In your `modules.toml` for the WAGI server, add the following:

Before running the sample, the modules, the following have to be configured
(In your `modules.toml` for the WAGI server add the values below):

- the Storage account name and key.
- the allowed hosts where the modules are allowed to make outbound
  requests to - these have to be the URL of Azure services (if no allowed hosts
  are defined for a given module, it is not allowed to make any outbound
  request).

```toml
[[module]]
route = "/readblob"
module = "target/wasm32-wasi/release/blob.wasm"
entrypoint = readblob
environment = { STORAGE_ACCOUNT = "<sa>", STORAGE_MASTER_KEY = "<sa-key>" }
allowed_hosts = ["https://<sa>.blob.core.windows.net"]

[[module]]
route = "/writeblob"
module = "target/wasm32-wasi/release/blob.wasm"
entrypoint = writeblob
environment = { STORAGE_ACCOUNT = "<sa>", STORAGE_MASTER_KEY = "<sa-key>" }
allowed_hosts = ["https://<sa>.blob.core.windows.net"]

```

Start the server:

```
$ wagi --config modules.toml
```

Then, from another terminal instance:

```
$  curl -d "Test blob write" 'http://localhost:3000/writeblob?container=wagitest&blob=test' -v
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 3000 (#0)
> POST /writeblob?container=wagitest&blob=test HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.58.0
> Accept: */*
> Content-Length: 15
> Content-Type: application/x-www-form-urlencoded
>
* upload completely sent off: 15 out of 15 bytes
< HTTP/1.1 200 OK
< Date: Fri, 11 Jun 2021 20:43:38 GMT
< Content-Type: text/plain
< Transfer-Encoding: chunked
<
Writing 15 bytes.
* Connection #0 to host localhost left intact
```
This will create or update a blob named test in the container wagitest in the storage account configured in modules.toml (note that the container should exist.)

To read the contents of the blob:

```
$  curl 'http://localhost:3000/readblob?container=wagitest&blob=test' -v
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET /readblob?container=wagitest&blob=test HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.58.0
> Accept: */*
>
< HTTP/1.1 200 OK
< Date: Fri, 11 Jun 2021 20:46:24 GMT
< Content-Type: text/plain
< Transfer-Encoding: chunked
<
Test blob write
* Connection #0 to host localhost left intact
```


[wagi]: https://github.com/deislabs/wagi
[bs]:
  https://docs.microsoft.com/en-us/azure/storage/blobs/storage-blobs-introduction
