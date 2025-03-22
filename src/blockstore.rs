use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// A [BlockStore] key.
///
/// key_version: u8,
/// user_id: u32,
/// write_space: u16,
/// version_id: u32,
///
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct BlockStoreKey([u8; 11]);

/// An abstraction for any storage for encrypted blocks.
pub trait BlockStore: Debug {
    fn get(&self, key: &BlockStoreKey) -> Option<&Vec<u8>>;
    fn insert(&mut self, key: BlockStoreKey, block: Vec<u8>);
}

#[derive(Debug)]
pub struct InMemoryBlockStore {
    hashmap: HashMap<BlockStoreKey, Vec<u8>>,
}

impl InMemoryBlockStore {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
}

impl BlockStore for InMemoryBlockStore {
    fn get(&self, key: &BlockStoreKey) -> Option<&Vec<u8>> {
        self.hashmap.get(key)
    }

    fn insert(&mut self, key: BlockStoreKey, block: Vec<u8>) {
        self.hashmap.insert(key, block);
    }
}
