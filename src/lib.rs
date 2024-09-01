//! # StaticId
//!
//! This library provides an extremely memory-efficient implementation of `StaticId`
//! for handling interned identifiers with optimal performance.
//!
//! ## Features
//!
//! - `StaticId`: A highly optimized, interned identifier type combining a code and a venue.
//! - Exceptional memory efficiency: Each `StaticId` is represented by a single 64-bit pointer.
//! - Ultra-fast comparisons: Equality checks and hashing operations only compare 8 bytes,
//!   regardless of the actual string length.
//! - Lazy evaluation: The actual string data is only accessed during serialization.
//!
//! ## Limitations
//!
//! - The `code` component of a `StaticId` cannot exceed 32 bytes.
//! - The `venue` component of a `StaticId` cannot exceed 16 bytes.
//! 
//! ## Usage
//!
//! ```rust
//! use static_id::StaticId;
//!
//! let id = StaticId::from_str("AAPL", "NASDAQ");
//! assert_eq!(id.get_id().code.as_str(), "AAPL");
//! assert_eq!(id.get_id().venue.as_str(), "NASDAQ");
//! 
//! assert_eq!(id.code_str(), "AAPL");
//! assert_eq!(id.venue_str(), "NASDAQ");
//!
//! // Fast equality check (compares only 8 bytes)
//! let id2 = StaticId::from_str("AAPL", "NASDAQ");
//! assert_eq!(id, id2);
//! println!("ID: {}", id); // => AAPL@NASDAQ
//! ```
//!
pub mod symbol;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use std::{
    hash::Hash, 
    hash::Hasher,
    ptr::eq as ptr_eq
};

use std::sync::Mutex;
use serde::{Serialize, Deserialize};
//use dashmap::{
//    DashMap,
//    mapref::entry::Entry as DashEntry,
//};

pub use symbol::Symbol;

pub type Code = Symbol<32>;
pub type Venue = Symbol<16>;

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Debug, Default)]
pub struct IdCore {
    pub code: Code,
    pub venue: Venue,
}

#[derive(Debug, Clone, Copy)]
pub struct StaticId {
    id_ptr: &'static IdCore,
}

impl Default for StaticId {
    fn default() -> Self {
        *DEFAULT_ID
    }
}

impl std::fmt::Display for StaticId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.id_ptr.code, self.id_ptr.venue)
    }
}

impl PartialEq for StaticId {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self.id_ptr, other.id_ptr)
    }
}

impl Eq for StaticId {}

impl Hash for StaticId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id_ptr.hash(state);
    }
}


//static ID_CACHE: Lazy<DashMap<IdCore, &'static IdCore>> = Lazy::new(DashMap::new);
static ID_CACHE: Lazy<Mutex<FxHashMap<IdCore, &'static IdCore>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));

static DEFAULT_ID: Lazy<StaticId> = Lazy::new(|| StaticId::from_str("", ""));

impl StaticId {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };

        let mut cache = ID_CACHE.lock().unwrap();
        
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));

        StaticId { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };

        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId { id_ptr: interned }
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore {
        self.id_ptr
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.id_ptr.code.len() + self.id_ptr.venue.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.id_ptr.code.is_empty() && self.id_ptr.venue.is_empty()
    }

    #[inline]
    pub fn upper_bound_len(&self) -> usize {
        self.id_ptr.code.upper_bound() + self.id_ptr.venue.upper_bound()    
    }

    #[inline]
    #[must_use]
    pub fn code_str(&self) -> &str {
        self.id_ptr.code.as_str()
    }

    #[inline]
    #[must_use]
    pub fn venue_str(&self) -> &str {
        self.id_ptr.venue.as_str()
    }
}

impl Serialize for StaticId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        IdCore {
            code: self.id_ptr.code,
            venue: self.id_ptr.venue,
        }.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StaticId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id: IdCore = IdCore::deserialize(deserializer)?;
        Ok(StaticId::from_str(id.code.as_str(), id.venue.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_static_id_equality() {
        let id1 = StaticId::from_str("ABC", "NYSE");
        let id2 = StaticId::from_str("ABC", "NYSE");
        let id3 = StaticId::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_static_id_reuse() {
        let id1 = StaticId::from_str("ABC", "NYSE");
        let id2 = StaticId::from_str("ABC", "NYSE");

        assert!(std::ptr::eq(id1.id_ptr, id2.id_ptr));
    }

    #[test]
    fn test_serialization() {
        let id = StaticId::from_str("ABC", "NYSE");
        let serialized = serde_json::to_string(&id).unwrap();
       
        let deserialized: StaticId = serde_json::from_str(&serialized).unwrap();

        assert_eq!(id, deserialized);
    }

    #[test]
    fn test_static_id_size() {
        let size = size_of::<StaticId>();
        println!("Size of StaticId: {} bytes", size);
        
        #[cfg(target_pointer_width = "64")]
        assert_eq!(size, 8, "On 64-bit systems, StaticId should be 8 bytes");
        
        #[cfg(target_pointer_width = "32")]
        assert_eq!(size, 4, "On 32-bit systems, StaticId should be 4 bytes");
    }

    #[test]
    fn test_debug() {
        let id = StaticId::from_str("ABC", "NYSE");
        println!("{:?}", id);
    }

    #[test]
    fn test_multi_threaded() {
        use std::thread;
        use std::sync::Arc;

        std::thread::sleep(std::time::Duration::from_secs(3));
        ID_CACHE.lock().unwrap().clear();

        let id = StaticId::from_str("ABC", "NYSE");
        let map = Arc::new(Mutex::new(FxHashMap::default()));

        map.lock().unwrap().insert(id, 1);
        let arc_id = Arc::new(id);

        let mut threads = Vec::new();
        for _ in 0..10 {
            let id_clone = arc_id.clone();
            let map_clone = map.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..100_000 {
                    let id_thd = StaticId::from_str("ABC", "NYSE");
                    assert_eq!(*id_clone, id_thd);

                    let map_locked = map_clone.lock().unwrap();
                    let x = map_locked.get(&id_thd).unwrap();
                    assert_eq!(*x, 1);
                }
            }));
        }

        for t in threads {
            t.join().unwrap();
        }
        
        assert_eq!(StaticId::cache_len(), 1);
    }

    #[test]
    fn test_default() {
        let id = StaticId::default();
        println!("{:?}", id);
        assert_eq!(id, *DEFAULT_ID);
    }

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(StaticId::from_str("AAPL", "NASDAQ"), 100);
        map.insert(StaticId::from_str("GOOGL", "NASDAQ"), 200);

        assert_eq!(map.get(&StaticId::from_str("AAPL", "NASDAQ")), Some(&100));
        assert_eq!(map.get(&StaticId::from_str("GOOGL", "NASDAQ")), Some(&200));
    }

    #[test]
    fn test_as_str() {
        let id = StaticId::from_str("AAPL", "NASDAQ");
        assert_eq!(id.code_str(), "AAPL");
        assert_eq!(id.venue_str(), "NASDAQ");
    }

    #[test]
    fn test_display() {
        let id = StaticId::from_str("AAPL", "NASDAQ");
        println!("{}", id);
        assert_eq!(id.to_string(), "AAPL@NASDAQ");
    }
}