use serde::{Deserialize, Serialize};
use std::ptr::copy_nonoverlapping;
use std::str::from_utf8_unchecked;

const END_MARK: u8 = 255;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol<const N: usize> {
    symbol: [u8; N],
}

impl<const N: usize> std::fmt::Debug for Symbol<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> Default for Symbol<N> {
    fn default() -> Self {
        Self { symbol: [END_MARK; N] }
    }
}

impl<const N: usize> Serialize for Symbol<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de, const N: usize> Deserialize<'de> for Symbol<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        if s.len() > N {
            return Err(serde::de::Error::custom(format!(
                "String is too long. Expected at most {} characters, found {}",
                N, s.len()
            )));
        }
        Ok(Self::from(s.as_str()))
    }
}

impl<const N: usize> From<&[u8]> for Symbol<N> {
    #[inline]
    fn from(slice: &[u8]) -> Self {
        let mut symbol = [END_MARK; N];
        let len = slice.len().min(N);
        unsafe {
            copy_nonoverlapping(slice.as_ptr(), symbol.as_mut_ptr(), len);
        }
        Self { symbol }
    }
}

impl<const N: usize> From<&str> for Symbol<N> {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes())
    }
}

impl<const N: usize> std::fmt::Display for Symbol<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> Symbol<N> {
    #[inline]
    pub fn len(&self) -> usize {
        self.symbol.iter().position(|&c| c == END_MARK).unwrap_or(N)
    }

    #[inline]
    pub fn upper_bound(&self) -> usize {
        self.symbol.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.symbol[0] == END_MARK
    }
    
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        let len = self.symbol.iter().position(|&c| c == END_MARK).unwrap_or(32);
        unsafe { from_utf8_unchecked(&self.symbol[..len]) }
    }

    /// # Safety
    /// The caller must ensure that the slice has at most `N` elements.
    pub unsafe fn copy_from_slice(&mut self, slice: &[u8], length: usize) {
        copy_nonoverlapping(slice.as_ptr(), self.symbol.as_mut_ptr(), length);
    }
}