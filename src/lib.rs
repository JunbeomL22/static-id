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
//! - The bound of `code` and `venue` are fixed: For StaticIdNxM, the maximum length of `code` and `venue` is N and M respectively. The exceeding characters will be truncated.
//! - The given structs are:
//!   * StaticId16x16
//!   * StaticId16x32
//!   * StaticId16x64
//!   * StaticId32x16
//!   * StaticId32x32 (=StaticId)
//!   * StaticId32x64
//!   * StaticId64x16
//!   * StaticId64x32
//!   * StaticId64x64
//! 
//! ## Usage
//!
//! ```rust
//! use static_id::prelude::*;
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
pub mod static_id;
pub mod prelude;

pub use symbol::Symbol;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::mem::size_of;
    use std::collections::HashMap;
    use serde_json;

    #[test]
    fn test_serde() {
        let id = StaticId::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);

        let mut map = HashMap::new();
        map.insert(StaticId::from_str("AAPL", "NASDAQ"), 100);

        let serialized = serde_json::to_string(&map).unwrap();
        println!("{}", serialized);
        let deserde: HashMap<StaticId, i32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(map, deserde);
    }
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
    fn test_default() {
        let id = StaticId::default();
        println!("{:?}", id);
        let expected_id = StaticId::from_str("", "");
        assert_eq!(id, expected_id);
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