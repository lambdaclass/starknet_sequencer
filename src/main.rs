use jsonrpsee::server::{ServerHandle, ServerBuilder};
use log::info;
use rpc::starknet_backend::StarknetBackend;
use crate::rpc::StarknetRpcApiServer;

mod rpc;
mod store;

const DB_PATH: &str = "store";

#[tokio::main]
async fn main() {
    // TODO: pass ports through args
    let port = 1234;
    let handle = start_rpc_server(port).await;

    match handle {
        Ok(handle) => {
            info!("Starknet Sequencer RPC Server started, listening on port {}", port);
            handle.stopped().await;
        }
        Err(e) => println!("Error creating RPC server: {}", e),
    };
}

pub async fn start_rpc_server(port: u16) -> Result<ServerHandle, jsonrpsee::core::Error> {
    let server = ServerBuilder::default()
        .build(format!("0.0.0.0:{}", port))
        .await?;
    let server_handle = server.start(StarknetBackend::new(DB_PATH).into_rpc())?;

    Ok(server_handle)
}
