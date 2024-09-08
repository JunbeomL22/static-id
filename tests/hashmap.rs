#[cfg(test)]
mod test {
    use static_id::static_id::*;

    #[test]
    fn test_hashmap_16x16() {
        let mut map = std::collections::HashMap::new();
        let id1 = StaticId16x16::from_str("AAPL", "NASDAQ");
        let id2 = StaticId16x16::from_str("AAPL", "NASDAQ");
        let id3 = StaticId16x16::from_str("AAPL", "NYSE");
        map.insert(id1, 100);
        map.insert(id2, 200);
        map.insert(id3, 300);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1), Some(&200));
        assert_eq!(map.get(&id3), Some(&300));
    }

    #[test]
    fn test_hashmap_16x32() {
        let mut map = std::collections::HashMap::new();
        let id1 = StaticId16x32::from_str("AAPL", "NASDAQ");
        let id2 = StaticId16x32::from_str("AAPL", "NASDAQ");
        let id3 = StaticId16x32::from_str("AAPL", "NYSE");
        map.insert(id1, 100);
        map.insert(id2, 200);
        map.insert(id3, 300);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1), Some(&200));
        assert_eq!(map.get(&id3), Some(&300));
    }

    #[test]
    fn test_hashmap_32x32() {
        let mut map = std::collections::HashMap::new();
        let id1 = StaticId32x32::from_str("AAPL", "NASDAQ");
        let id2 = StaticId32x32::from_str("AAPL", "NASDAQ");
        let id3 = StaticId32x32::from_str("AAPL", "NYSE");
        map.insert(id1, 100);
        map.insert(id2, 200);
        map.insert(id3, 300);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1), Some(&200));
        assert_eq!(map.get(&id3), Some(&300));
    }

    #[test]
    fn test_hashmap_32x64() {
        let mut map = std::collections::HashMap::new();
        let id1 = StaticId32x64::from_str("AAPL", "NASDAQ");
        let id2 = StaticId32x64::from_str("AAPL", "NASDAQ");
        let id3 = StaticId32x64::from_str("AAPL", "NYSE");
        map.insert(id1, 100);
        map.insert(id2, 200);
        map.insert(id3, 300);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1), Some(&200));
        assert_eq!(map.get(&id3), Some(&300));
    }

    #[test]
    fn test_hashmap_64x16() {
        let mut map = std::collections::HashMap::new();
        let id1 = StaticId64x16::from_str("AAPL", "NASDAQ");
        let id2 = StaticId64x16::from_str("AAPL", "NASDAQ");
        let id3 = StaticId64x16::from_str("AAPL", "NYSE");
        map.insert(id1, 100);
        map.insert(id2, 200);
        map.insert(id3, 300);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1), Some(&200));
        assert_eq!(map.get(&id3), Some(&300));
    }

    #[test]
    fn test_hashmap_64x32() {
        let mut map = std::collections::HashMap::new();
        let id1 = StaticId64x32::from_str("AAPL", "NASDAQ");
        let id2 = StaticId64x32::from_str("AAPL", "NASDAQ");
        let id3 = StaticId64x32::from_str("AAPL", "NYSE");
        map.insert(id1, 100);
        map.insert(id2, 200);
        map.insert(id3, 300);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1), Some(&200));
        assert_eq!(map.get(&id3), Some(&300));
    }

    #[test]
    fn test_hashmap_64x64() {
        let mut map = std::collections::HashMap::new();
        let id1 = StaticId64x64::from_str("AAPL", "NASDAQ");
        let id2 = StaticId64x64::from_str("AAPL", "NASDAQ");
        let id3 = StaticId64x64::from_str("AAPL", "NYSE");
        map.insert(id1, 100);
        map.insert(id2, 200);
        map.insert(id3, 300);

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&id1), Some(&200));
        assert_eq!(map.get(&id3), Some(&300));
    }
}