//! QUANTA L1 Node RPC module
//!
//! JSON-RPC server using jsonrpsee for Substrate-compatible endpoints.

use std::sync::Arc;
use parking_lot::Mutex;
use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};

use crate::storage::DevStorage;

// ---------------------------------------------------------------------------
// Health & Header response types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub is_syncing: bool,
    pub peers: u32,
    pub should_have_peers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub number: u32,
    pub parent_hash: String,
    pub state_root: String,
    pub extrinsics_root: String,
    pub digest: Vec<String>,
}

// ---------------------------------------------------------------------------
// JSON-RPC trait definition
// ---------------------------------------------------------------------------

#[rpc(server)]
pub trait NodeRpcApi {
    #[method(name = "system_name")]
    fn system_name(&self) -> RpcResult<String>;

    #[method(name = "system_version")]
    fn system_version(&self) -> RpcResult<String>;

    #[method(name = "system_health")]
    fn system_health(&self) -> RpcResult<Health>;

    #[method(name = "chain_getBlockNumber")]
    fn chain_get_block_number(&self) -> RpcResult<u32>;

    #[method(name = "chain_getHeader")]
    fn chain_get_header(&self, hash: Option<String>) -> RpcResult<BlockHeader>;

    #[method(name = "state_getStorage")]
    fn state_get_storage(&self, key: String, hash: Option<String>) -> RpcResult<Option<String>>;

    #[method(name = "engine_createBlock")]
    fn engine_create_block(&self) -> RpcResult<bool>;
}

// ---------------------------------------------------------------------------
// RPC implementation
// ---------------------------------------------------------------------------

pub struct NodeRpcImpl {
    pub storage: Arc<Mutex<DevStorage>>,
}

impl NodeRpcImpl {
    pub fn new(storage: Arc<Mutex<DevStorage>>) -> Self {
        Self { storage }
    }
}

impl NodeRpcApiServer for NodeRpcImpl {
    fn system_name(&self) -> RpcResult<String> {
        Ok("quanta-l1".to_string())
    }

    fn system_version(&self) -> RpcResult<String> {
        Ok("0.1.0".to_string())
    }

    fn system_health(&self) -> RpcResult<Health> {
        Ok(Health {
            is_syncing: false,
            peers: 0,
            should_have_peers: false,
        })
    }

    fn chain_get_block_number(&self) -> RpcResult<u32> {
        let s = self.storage.lock();
        let key = b"SystemNumber".to_vec();
        Ok(s.get(&key)
            .map(|v| u32::from_le_bytes(v.as_slice().try_into().unwrap_or([0; 4])))
            .unwrap_or(0))
    }

    fn chain_get_header(&self, hash: Option<String>) -> RpcResult<BlockHeader> {
        let _ = hash;
        let s = self.storage.lock();

        let block_number = {
            let key = b"SystemNumber".to_vec();
            s.get(&key)
                .map(|v| u32::from_le_bytes(v.as_slice().try_into().unwrap_or([0; 4])))
                .unwrap_or(0)
        };

        let parent_hash_key = format!("HeaderParent{}", block_number);
        let parent_hash = s
            .get(parent_hash_key.as_bytes())
            .map(|v| hex::encode(v))
            .unwrap_or_else(|| "0x".repeat(64));

        let state_root_key = format!("HeaderState{}", block_number);
        let state_root = s
            .get(state_root_key.as_bytes())
            .map(|v| hex::encode(v))
            .unwrap_or_else(|| "0x".repeat(64));

        let ext_root_key = format!("HeaderExt{}", block_number);
        let ext_root = s
            .get(ext_root_key.as_bytes())
            .map(|v| hex::encode(v))
            .unwrap_or_else(|| "0x".repeat(64));

        Ok(BlockHeader {
            number: block_number,
            parent_hash,
            state_root,
            extrinsics_root: ext_root,
            digest: vec![],
        })
    }

    fn state_get_storage(&self, key: String, _hash: Option<String>) -> RpcResult<Option<String>> {
        let key_bytes = hex_decode(&key);
        let s = self.storage.lock();
        Ok(s.get(&key_bytes).map(|v| hex::encode(v)))
    }

    fn engine_create_block(&self) -> RpcResult<bool> {
        let mut s = self.storage.lock();

        let num_key = b"SystemNumber".to_vec();
        let current = s
            .get(&num_key)
            .map(|v| u32::from_le_bytes(v.as_slice().try_into().unwrap_or([0u8; 4])))
            .unwrap_or(0);
        let new_number = current + 1;
        s.insert(num_key, new_number.to_le_bytes().to_vec());

        let parent_key = format!("HeaderParent{}", new_number);
        let parent_hash = format!("{:064x}", current);
        s.insert(parent_key.as_bytes().to_vec(), hex_decode(&parent_hash));

        let state_key = format!("HeaderState{}", new_number);
        let state_root = format!("{:064x}", new_number);
        s.insert(state_key.as_bytes().to_vec(), hex_decode(&state_root));

        let ext_key = format!("HeaderExt{}", new_number);
        let ext_root = format!("{:064x}", 0u32);
        s.insert(ext_key.as_bytes().to_vec(), hex_decode(&ext_root));

        Ok(true)
    }
}

// ---------------------------------------------------------------------------
// Hex helpers
// ---------------------------------------------------------------------------

fn hex_encode(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

fn hex_decode(s: &str) -> Vec<u8> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    hex::decode(s).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Backward-compatible types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NodeInfo {
    pub spec_name: String,
    pub impl_name: String,
    pub spec_version: u32,
    pub impl_version: u32,
    pub chain_type: String,
}

impl NodeInfo {
    pub fn new() -> Self {
        use quanta_l1_runtime::VERSION;
        Self {
            spec_name: VERSION.spec_name.to_string(),
            impl_name: VERSION.impl_name.to_string(),
            spec_version: VERSION.spec_version,
            impl_version: VERSION.impl_version,
            chain_type: "Development".to_string(),
        }
    }
}

#[allow(dead_code)]
pub struct NodeRpcServer;

impl NodeRpcServer {
    pub fn new() -> Self {
        Self
    }

    pub fn system_info(&self) -> NodeInfo {
        NodeInfo::new()
    }

    pub fn system_health(&self) -> Health {
        Health {
            is_syncing: false,
            peers: 0,
            should_have_peers: false,
        }
    }

    pub fn block_number(&self) -> u32 {
        0
    }

    pub fn chain(&self) -> String {
        quanta_l1_runtime::VERSION.spec_name.to_string()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::DevStorage;

    #[test]
    fn node_info_correct() {
        let info = NodeInfo::new();
        assert_eq!(info.spec_name, "quanta-l1");
        assert_eq!(info.impl_name, "quanta-l1");
        assert_eq!(info.spec_version, 1);
        assert_eq!(info.chain_type, "Development");
    }

    #[test]
    fn health_ok() {
        let h = Health {
            is_syncing: false,
            peers: 0,
            should_have_peers: false,
        };
        assert!(!h.is_syncing);
        assert_eq!(h.peers, 0);
    }

    #[test]
    fn rpc_server_creates() {
        let server = NodeRpcServer::new();
        assert_eq!(server.block_number(), 0);
        assert_eq!(server.chain(), "quanta-l1");
    }

    #[test]
    fn rpc_impl_system_name() {
        let storage = Arc::new(Mutex::new(DevStorage::new()));
        let rpc = NodeRpcImpl::new(storage);
        assert_eq!(rpc.system_name().unwrap(), "quanta-l1");
    }

    #[test]
    fn rpc_impl_system_version() {
        let storage = Arc::new(Mutex::new(DevStorage::new()));
        let rpc = NodeRpcImpl::new(storage);
        assert_eq!(rpc.system_version().unwrap(), "0.1.0");
    }

    #[test]
    fn rpc_impl_system_health() {
        let storage = Arc::new(Mutex::new(DevStorage::new()));
        let rpc = NodeRpcImpl::new(storage);
        let h = rpc.system_health().unwrap();
        assert!(!h.is_syncing);
        assert_eq!(h.peers, 0);
        assert!(!h.should_have_peers);
    }

    #[test]
    fn rpc_impl_block_number_starts_at_zero() {
        let storage = Arc::new(Mutex::new(DevStorage::new()));
        let rpc = NodeRpcImpl::new(storage);
        assert_eq!(rpc.chain_get_block_number().unwrap(), 0);
    }

    #[test]
    fn rpc_impl_engine_create_block() {
        let storage = Arc::new(Mutex::new(DevStorage::new()));
        let rpc = NodeRpcImpl::new(storage);
        assert!(rpc.engine_create_block().unwrap());
        assert_eq!(rpc.chain_get_block_number().unwrap(), 1);
        assert!(rpc.engine_create_block().unwrap());
        assert_eq!(rpc.chain_get_block_number().unwrap(), 2);
    }

    #[test]
    fn rpc_impl_get_header() {
        let storage = Arc::new(Mutex::new(DevStorage::new()));
        let rpc = NodeRpcImpl::new(storage);
        rpc.engine_create_block().unwrap();
        let header = rpc.chain_get_header(None).unwrap();
        assert_eq!(header.number, 1);
        assert!(!header.parent_hash.is_empty());
    }

    #[test]
    fn rpc_impl_state_get_storage() {
        let mut storage_inner = DevStorage::new();
        storage_inner.insert(b"TestKey".to_vec(), b"TestValue".to_vec());
        let storage = Arc::new(Mutex::new(storage_inner));
        let rpc = NodeRpcImpl::new(storage);
        let result = rpc
            .state_get_storage("0x546573744b6579".to_string(), None)
            .unwrap();
        assert!(result.is_some());
        let val = hex_decode(result.as_ref().unwrap());
        assert_eq!(val, b"TestValue");
    }
}
