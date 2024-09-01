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
//! // Fast equality check (compares only 8 bytes)
//! let id2 = StaticId::from_str("AAPL", "NASDAQ");
//! assert_eq!(id, id2);
//! ```
//!
pub mod symbol;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
pub use symbol::Symbol;

pub type Code = Symbol<32>;
pub type Venue = Symbol<16>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct IdCore {
    pub code: Code,
    pub venue: Venue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticId {
    id_ptr: &'static IdCore,
}

static ID_CACHE: Lazy<Mutex<FxHashMap<IdCore, &'static IdCore>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
//static LARGE_ID_CACHE: Lazy<Mutex<FxHashMap<LargeIdCore, &'static LargeIdCore>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));

impl StaticId {
    #[inline]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };

        let mut cache = ID_CACHE.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id.clone())));

        StaticId { id_ptr: *interned }
    }

    #[inline]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };

        let mut cache = ID_CACHE.lock().unwrap();

        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id.clone())));

        StaticId { id_ptr: *interned }
    }

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
    fn test_lengths() {
        let id1 = StaticId::from_str("ABC", "NYSE");
        let id2 = StaticId::from_str("XYZ", "NASDAQ");
        let id3 = StaticId::from_str("ABC", "NASDAQ");
        let id4 = StaticId::from_str("XYZ", "NASDAQ");

        assert_eq!(id1.len(), 7);
        assert_eq!(id2.len(), 9);
        assert_eq!(id3.len(), 9);
        assert_eq!(id4.len(), 9);

        assert_eq!(StaticId::cache_len(), 3);
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
}