use std::sync::{Arc, Mutex};

use axum::{Router, routing::get};
use tokio;
use dlc_manager::{self, Wallet};
use dlc_manager::{Oracle, SystemTimeProvider};
use bitcoin_rpc_provider::BitcoinCoreProvider;
use dlc_sled_storage_provider;
use p2pd_oracle_client::P2PDOracleClient;

pub(crate) type DlcManager = dlc_manager::manager::Manager<
    Arc<BitcoinCoreProvider>,
    Arc<BitcoinCoreProvider>,
    Box<dlc_sled_storage_provider::SledStorageProvider>,
    Box<P2PDOracleClient>,
    Arc<SystemTimeProvider>,
    Arc<BitcoinCoreProvider>,
>;

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread();
    let router = Router::new();
    let dlc_manager = Arc::new(Mutex::new(DlcManager::new()));
    router.route("/new", get(|| async {} ));
    println!("Hello, world!");
}
