#[cfg(test)]
mod tests {
    use static_id::static_id::*;

    #[test]
    fn test_serde_16x16() {
        let id = StaticId16x16::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId16x16 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);
    }

    #[test]
    fn test_serde_16x32() {
        let id = StaticId16x32::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId16x32 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);
    }

    #[test]
    fn test_serde_32x32() {
        let id = StaticId32x32::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId32x32 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);
    }

    #[test]
    fn test_serde_32x64() {
        let id = StaticId32x64::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId32x64 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);
    }

    #[test]
    fn test_serde_64x16() {
        let id = StaticId64x64::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId64x64 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);
    }

    #[test]
    fn test_serde_64x32() {
        let id = StaticId64x32::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId64x32 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);
    }

    #[test]
    fn test_serde_64x64() {
        let id = StaticId64x64::from_str("AAPL", "NASDAQ");
        let serialized = serde_json::to_string(&id).unwrap();
        println!("{}", serialized);
        let deserde: StaticId64x64 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserde);
    }

}