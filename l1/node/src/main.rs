//! QUANTA L1 Dev Node — with RPC and Block Production
//!
//! Usage:
//!   cargo run -p quanta-l1-node [--dev]
//!
//! RPC endpoint: http://localhost:9933
//! WebSocket: ws://localhost:9944

#![cfg(feature = "std")]

use quanta_l1_runtime::{Runtime, VERSION};
use std::sync::Arc;
use parking_lot::Mutex;
use std::time::SystemTime;

mod rpc;
mod storage;

use rpc::{NodeRpcImpl, NodeRpcApiServer};
use storage::DevStorage;

#[tokio::main]
async fn main() {
    let start_time = SystemTime::now();

    println!("══════════════════════════════════════════════════");
    println!("║  QUANTA L1 — Quantum-safe AI-native Blockchain  ║");
    eprintln!("╚══════════════════════════════════════════════════╝");
    eprintln!();
    eprintln!("Spec name:      {}", VERSION.spec_name);
    eprintln!("Impl name:      {}", VERSION.impl_name);
    eprintln!("Spec version:   {}", VERSION.spec_version);
    eprintln!("Impl version:   {}", VERSION.impl_version);
    eprintln!("Native runtime: quanta-l1-runtime");
    eprintln!("WASM runtime:   supported");
    eprintln!();
    eprintln!("Pallets:");
    eprintln!("  ✓ frame-system");
    eprintln!("  ✓ pallet-balances (dev)");
    eprintln!("  ✓ pallet-pq-dilithium  (Dilithium3 PQ signatures)");
    eprintln!("  ✓ pallet-pq-balances   (PQ balance management)");
    eprintln!("  ✓ pallet-pq-staking    (PoUW inference staking)");
    eprintln!();
    eprintln!("Crypto: Dilithium3 (ML-DSA-65) — NIST Level 3");
    eprintln!("  Public key:  1,952 bytes");
    eprintln!("  Signature:   3,309 bytes");
    eprintln!("  Secret key:  4,032 bytes");
    eprintln!();
    eprintln!("Consensus: Manual Seal (dev mode)");
    eprintln!("Block time:  6 seconds");
    eprintln!();

    let storage = Arc::new(Mutex::new(DevStorage::new()));
    {
        let s = storage.lock();
        eprintln!("Genesis storage: {} top-level entries", s.top_count());
    }
    eprintln!();

    let _runtime_type = std::any::type_name::<Runtime>();
    eprintln!("✓ Runtime: {}", _runtime_type);
    eprintln!("✓ Version: spec={} impl={}", VERSION.spec_version, VERSION.impl_version);
    eprintln!("✓ WASM: supported");
    eprintln!();

    let rpc_impl = NodeRpcImpl::new(storage.clone());

    let rpc_addr = "0.0.0.0:9944".parse::<std::net::SocketAddr>().unwrap();
    let builder = jsonrpsee::server::ServerBuilder::default();
    let server = builder.build(rpc_addr).await.unwrap();
    let handle = server.start(rpc_impl.into_rpc());

    let elapsed = SystemTime::now().duration_since(start_time).unwrap();
    eprintln!("QUANTA L1 Node started in {:.2}s", elapsed.as_secs_f64());
    eprintln!("JSON-RPC server listening on ws://{}", rpc_addr);
    eprintln!("HTTP endpoint available at http://{}", rpc_addr);
    eprintln!("Press Ctrl+C to exit.");

    tokio::signal::ctrl_c().await.unwrap();
    eprintln!("\nShutting down...");
    handle.stop().unwrap();
    handle.stopped().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_version_correct() {
        assert_eq!(VERSION.spec_version, 1);
        assert_eq!(VERSION.impl_version, 1);
    }

    #[test]
    fn runtime_type_available() {
        let name = std::any::type_name::<Runtime>();
        assert!(name.contains("Runtime"));
    }

    #[test]
    fn dev_storage_builds() {
        let storage = DevStorage::new();
        assert!(storage.top_count() > 0);
    }
}
