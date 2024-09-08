#[cfg(test)]
mod tests{
    use static_id::prelude::*;

    #[test]
    fn test_static_id_equality() {
        let id1 = StaticId::from_str("ABC", "NYSE");
        let id2 = StaticId::from_str("ABC", "NYSE");
        let id3 = StaticId::from_str("XYZ", "NASDAQ");
        //
        let id1_16x64 = StaticId16x64::from_str("ABC", "NYSE");
        let id2_16x64 = StaticId16x64::from_str("ABC", "NYSE");
        let id3_16x64 = StaticId16x64::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);

        assert_eq!(id1_16x64, id2_16x64);
        assert_ne!(id1_16x64, id3_16x64);

        let cache_len = StaticId::cache_len();
        let cache_len_16x64 = StaticId16x64::cache_len();

        assert_eq!(cache_len, 2);
        assert_eq!(cache_len_16x64, 2);
    }

    #[test]
    fn test_static_id_equality_16x16() {
        let id1 = StaticId16x16::from_str("ABC", "NYSE");
        let id2 = StaticId16x16::from_str("ABC", "NYSE");
        let id3 = StaticId16x16::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_static_id_equality_16x32() {
        let id1 = StaticId16x32::from_str("ABC", "NYSE");
        let id2 = StaticId16x32::from_str("ABC", "NYSE");
        let id3 = StaticId16x32::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_static_id_equality_32x32() {
        let id1 = StaticId32x32::from_str("ABC", "NYSE");
        let id2 = StaticId32x32::from_str("ABC", "NYSE");
        let id3 = StaticId32x32::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_static_id_equality_32x64() {
        let id1 = StaticId32x64::from_str("ABC", "NYSE");
        let id2 = StaticId32x64::from_str("ABC", "NYSE");
        let id3 = StaticId32x64::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_static_id_equality_64x16() {
        let id1 = StaticId64x16::from_str("ABC", "NYSE");
        let id2 = StaticId64x16::from_str("ABC", "NYSE");
        let id3 = StaticId64x16::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_static_id_equality_64x32() {
        let id1 = StaticId64x32::from_str("ABC", "NYSE");
        let id2 = StaticId64x32::from_str("ABC", "NYSE");
        let id3 = StaticId64x32::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_static_id_equality_64x64() {
        let id1 = StaticId64x64::from_str("ABC", "NYSE");
        let id2 = StaticId64x64::from_str("ABC", "NYSE");
        let id3 = StaticId64x64::from_str("XYZ", "NASDAQ");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}