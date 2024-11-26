use anyhow::anyhow;
use log::info;
use rsnano_core::{
    work::{WorkPool, WorkPoolImpl, WorkThresholds}, Account, Amount, Block, BlockHash, KeyPair, RawKey, StateBlock
};
use rsnano_rpc_client::NanoRpcClient;
use rsnano_rpc_messages::{AccountInfoArgs, BlockSubTypeDto, ProcessArgs};
use std::time::Duration;
use tokio::task::spawn_blocking;



#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}




/// Generate proof of work for a given hash
/// Returns the work as a hexadecimal string
// #[flutter_rust_bridge::frb(sync)]



pub fn get_pow(hash_str: String) -> Result<String, String> {
    let hash = BlockHash::decode_hex(&hash_str)
        .map_err(|e| format!("Invalid hash format: {}", e))?;

    info!("Starting with PoW generation");
    let work_pool = WorkPoolImpl::new(WorkThresholds::publish_full().clone(), 4, Duration::ZERO);
    let work = work_pool
        .generate(hash.into(), work_pool.threshold_base())
        .ok_or_else(|| "Failed to generate PoW".to_string())
        .map_err(|e| format!("Failed to generate PoW: {}", e))?;
    info!("PoW generation finished");

    // Convert work to hexadecimal string
    Ok(format!("{:016x}", work))
}

// pub async fn get_pow(hash_str: String) -> Result<String, String> {

//     let hash = BlockHash::decode_hex(&hash_str)
//     .map_err(|e| format!("Invalid hash format: {}", e))?;

//     let work = spawn_blocking(move || {
//         info!("Starting with PoW generation");
//         let work_pool =
//             WorkPoolImpl::new(WorkThresholds::publish_full().clone(), 4, Duration::ZERO);
//         let work = work_pool
//             .generate(hash.into(), work_pool.threshold_base())
//             .unwrap();
//         info!("PoW generation finished");
//         work
//     })
//     .await.unwrap();

//     let work: u64 = 1000000;
    
//     // Convert work to hexadecimal string
//     Ok(format!("{:016x}", work))
// }

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
