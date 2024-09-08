use crate::symbol::Symbol;
//use crate::IdCore;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use std::{
    hash::Hash, 
    hash::Hasher,
    ptr::eq as ptr_eq
};

use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use serde::{Serializer, Deserializer};

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Debug, Default)]
pub struct IdCoreNxM<const N: usize, const M: usize> {
    pub code: Symbol<N>,
    pub venue: Symbol<M>,
}

#[derive(Debug, Clone, Copy)]
pub struct StaticIdNxM<const N: usize, const M: usize> {
    id_ptr: &'static IdCoreNxM<N, M>,
}


impl<const N: usize, const M: usize> std::fmt::Display for StaticIdNxM<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.id_ptr.code, self.id_ptr.venue)
    }
}

impl<const N: usize, const M: usize> PartialEq for StaticIdNxM<N, M> {
    fn eq(&self, other: &Self) -> bool {
        ptr_eq(self.id_ptr, other.id_ptr)
    }
}

impl<const N: usize, const M: usize> Eq for StaticIdNxM<N, M> {}

impl<const N: usize, const M: usize> Hash for StaticIdNxM<N, M> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id_ptr.hash(state);
    }
}

impl<const N: usize, const M: usize> StaticIdNxM<N, M> {

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

impl<const N: usize, const M: usize> Serialize for StaticIdNxM<N, M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type IdCore16x16 = IdCoreNxM<16, 16>;
pub type IdCore16x32 = IdCoreNxM<16, 32>;
pub type IdCore16x64 = IdCoreNxM<16, 64>;
pub type IdCore32x16 = IdCoreNxM<32, 16>;
pub type IdCore32x32 = IdCoreNxM<32, 32>;
pub type IdCore32x64 = IdCoreNxM<32, 64>;
pub type IdCore64x16 = IdCoreNxM<64, 16>;
pub type IdCore64x32 = IdCoreNxM<64, 32>;
pub type IdCore64x64 = IdCoreNxM<64, 64>;
pub type IdCore = IdCoreNxM<32, 32>;

pub type StaticId16x16 = StaticIdNxM<16, 16>;
pub type StaticId16x32 = StaticIdNxM<16, 32>;
pub type StaticId16x64 = StaticIdNxM<16, 64>;
pub type StaticId32x16 = StaticIdNxM<32, 16>;
pub type StaticId32x32 = StaticIdNxM<32, 32>;
pub type StaticId32x64 = StaticIdNxM<32, 64>;
pub type StaticId64x16 = StaticIdNxM<64, 16>;
pub type StaticId64x32 = StaticIdNxM<64, 32>;
pub type StaticId64x64 = StaticIdNxM<64, 64>;
pub type StaticId = StaticIdNxM<32, 32>;

static ID_CACHE_16X16: Lazy<Mutex<FxHashMap<IdCore16x16, &'static IdCore16x16>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_16X16: Lazy<StaticId16x16> = Lazy::new(|| StaticId16x16::from_str("", ""));

static ID_CACHE_16X32: Lazy<Mutex<FxHashMap<IdCore16x32, &'static IdCore16x32>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_16X32: Lazy<StaticId16x32> = Lazy::new(|| StaticId16x32::from_str("", ""));

static ID_CACHE_16X64: Lazy<Mutex<FxHashMap<IdCore16x64, &'static IdCore16x64>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_16X64: Lazy<StaticId16x64> = Lazy::new(|| StaticId16x64::from_str("", ""));

static ID_CACHE_32X16: Lazy<Mutex<FxHashMap<IdCore32x16, &'static IdCore32x16>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_32X16: Lazy<StaticId32x16> = Lazy::new(|| StaticId32x16::from_str("", ""));

static ID_CACHE_32X32: Lazy<Mutex<FxHashMap<IdCore32x32, &'static IdCore32x32>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_32X32: Lazy<StaticId32x32> = Lazy::new(|| StaticId32x32::from_str("", ""));

static ID_CACHE_32X64: Lazy<Mutex<FxHashMap<IdCore32x64, &'static IdCore32x64>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_32X64: Lazy<StaticId32x64> = Lazy::new(|| StaticId32x64::from_str("", ""));

static ID_CACHE_64X16: Lazy<Mutex<FxHashMap<IdCore64x16, &'static IdCore64x16>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_64X16: Lazy<StaticId64x16> = Lazy::new(|| StaticId64x16::from_str("", ""));

static ID_CACHE_64X32: Lazy<Mutex<FxHashMap<IdCore64x32, &'static IdCore64x32>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_64X32: Lazy<StaticId64x32> = Lazy::new(|| StaticId64x32::from_str("", ""));

static ID_CACHE_64X64: Lazy<Mutex<FxHashMap<IdCore64x64, &'static IdCore64x64>>> = Lazy::new(|| Mutex::new(FxHashMap::default()));
static DEFAULT_ID_64X64: Lazy<StaticId64x64> = Lazy::new(|| StaticId64x64::from_str("", ""));

impl StaticId16x16 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore16x16 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_16X16.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId16x16 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore16x16 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_16X16.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId16x16 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_16X16.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore16x16 {
        self.id_ptr
    }
}

impl Default for StaticId16x16 {
    fn default() -> Self {
        *DEFAULT_ID_16X16
    }
}

impl<'de> Deserialize<'de> for StaticId16x16 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId16x16, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId16x16::from_combined_str(&s))
    }
}

// 16x32

impl StaticId16x32 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore16x32 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_16X32.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId16x32 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore16x32 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_16X32.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId16x32 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_16X32.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore16x32 {
        self.id_ptr
    }
}

impl Default for StaticId16x32 {
    fn default() -> Self {
        *DEFAULT_ID_16X32
    }
}

impl<'de> Deserialize<'de> for StaticId16x32 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId16x32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId16x32::from_combined_str(&s))
    }
}
// 16x64
impl StaticId16x64 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore16x64 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_16X64.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId16x64 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore16x64 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_16X64.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId16x64 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_16X64.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore16x64 {
        self.id_ptr
    }
}

impl Default for StaticId16x64 {
    fn default() -> Self {
        *DEFAULT_ID_16X64
    }
}

impl<'de> Deserialize<'de> for StaticId16x64 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId16x64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId16x64::from_combined_str(&s))
    }
}

// 32x16
impl StaticId32x16 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore32x16 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_32X16.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId32x16 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore32x16 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_32X16.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId32x16 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_32X16.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore32x16 {
        self.id_ptr
    }
}

impl Default for StaticId32x16 {
    fn default() -> Self {
        *DEFAULT_ID_32X16
    }
}

impl<'de> Deserialize<'de> for StaticId32x16 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId32x16, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId32x16::from_combined_str(&s))
    }
}

// 32x32

impl StaticId32x32 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore32x32 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_32X32.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId32x32 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore32x32 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_32X32.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId32x32 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_32X32.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore32x32 {
        self.id_ptr
    }
}

impl Default for StaticId32x32 {
    fn default() -> Self {
        *DEFAULT_ID_32X32
    }
}

impl<'de> Deserialize<'de> for StaticId32x32 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId32x32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId32x32::from_combined_str(&s))
    }
}

// 32x64

impl StaticId32x64 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore32x64 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_32X64.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId32x64 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore32x64 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_32X64.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId32x64 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_32X64.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore32x64 {
        self.id_ptr
    }
}

impl Default for StaticId32x64 {
    fn default() -> Self {
        *DEFAULT_ID_32X64
    }
}

impl<'de> Deserialize<'de> for StaticId32x64 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId32x64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId32x64::from_combined_str(&s))
    }
}

// 64x16

impl StaticId64x16 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore64x16 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_64X16.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId64x16 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore64x16 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_64X16.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId64x16 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_64X16.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore64x16 {
        self.id_ptr
    }
}

impl Default for StaticId64x16 {
    fn default() -> Self {
        *DEFAULT_ID_64X16
    }
}

impl<'de> Deserialize<'de> for StaticId64x16 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId64x16, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId64x16::from_combined_str(&s))
    }
}

// 64x32

impl StaticId64x32 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore64x32 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_64X32.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId64x32 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore64x32 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_64X32.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId64x32 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_64X32.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore64x32 {
        self.id_ptr
    }
}

impl Default for StaticId64x32 {
    fn default() -> Self {
        *DEFAULT_ID_64X32
    }
}

impl<'de> Deserialize<'de> for StaticId64x32 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId64x32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId64x32::from_combined_str(&s))
    }
}

// 64x64

impl StaticId64x64 {
    #[inline]
    #[must_use]
    pub fn from_str(code: &str, venue: &str) -> Self {
        let id = IdCore64x64 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_64X64.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId64x64 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_bytes(code: &[u8], venue: &[u8]) -> Self {
        let id = IdCore64x64 {
            code: Symbol::from(code),
            venue: Symbol::from(venue),
        };
        let mut cache = ID_CACHE_64X64.lock().unwrap();
        let interned = cache.entry(id.clone()).or_insert_with(|| Box::leak(Box::new(id)));
        StaticId64x64 { id_ptr: interned }
    }

    #[inline]
    #[must_use]
    pub fn from_combined_str(combined: &str) -> Self {
        let (code, venue) = combined.split_at(combined.find('@').unwrap());
        let venue = &venue[1..];
        Self::from_str(code, venue)
    }

    #[inline]
    pub fn cache_len() -> usize {
        ID_CACHE_64X64.lock().unwrap().len()
    }

    #[inline]
    pub fn get_id(&self) -> &IdCore64x64 {
        self.id_ptr
    }
}

impl Default for StaticId64x64 {
    fn default() -> Self {
        *DEFAULT_ID_64X64
    }
}

impl<'de> Deserialize<'de> for StaticId64x64 {
    fn deserialize<D>(deserializer: D) -> Result<StaticId64x64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StaticId64x64::from_combined_str(&s))
    }
}
