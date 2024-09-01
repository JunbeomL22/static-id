# StaticId

This Rust library provides an extremely memory-efficient implementation of `StaticId` for handling interned identifiers with optimal performance.

## Features

- `StaticId`: A highly optimized, interned identifier type combining a code and a venue.
- Exceptional memory efficiency: Each `StaticId` is represented by a single 64-bit pointer.
- Ultra-fast comparisons: Equality checks and hashing operations only compare 8 bytes, regardless of the actual string length.
- Lazy evaluation: The actual string data is only accessed during serialization.
- Serialization and deserialization support using Serde.

## Memory Efficiency and Performance

The key features of `StaticId` are its memory efficiency and performance:

1. **Compact Representation**: Each `StaticId` is stored as a single 64-bit pointer, regardless of the length of the underlying strings.
2. **Fast Comparisons**: Equality checks and hash computations only compare the 64-bit pointers, making these operations extremely fast and constant-time.
3. **Lazy Evaluation**: The actual string data is only accessed when necessary (e.g., during serialization), minimizing unnecessary memory access.

## Limitations

- The `code` component of a `StaticId` cannot exceed 32 bytes.
- The `venue` component of a `StaticId` cannot exceed 16 bytes.
- Attempting to create a `StaticId` with components exceeding these limits will result in truncation.


## Usage

`StaticId` combines a `Code` (up to 32 bytes) and a `Venue` (up to 16 bytes) into an interned identifier:

```rust
use crate::StaticId;

// Create from string slices
let id = StaticId::from_str("AAPL", "NASDAQ");

// Create from byte slices
let id_bytes = StaticId::from_bytes(b"AAPL", b"NASDAQ");

assert_eq!(id.get_id().code.as_str(), "AAPL");
assert_eq!(id.get_id().venue.as_str(), "NASDAQ");

// Get the length of the combined code and venue
println!("Length: {}", id.len());  // Outputs the sum of code and venue lengths

// Get the number of unique StaticIds in the cache
println!("Cache size: {}", StaticId::cache_len());

// Fast equality check (compares only 8 bytes)
let id2 = StaticId::from_str("AAPL", "NASDAQ");
assert_eq!(id, id2);

// Memory usage
println!("Size of StaticId: {} bytes", std::mem::size_of::<StaticId>());  // Outputs: 8 bytes
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
static_id = "0.1.0"
```

## Performance Considerations

- The first creation of a `StaticId` with unique content will involve allocation and interning. Subsequent creations of `StaticId`s with the same content will reuse the interned value.
- While creation and interning have some overhead, subsequent operations like equality checks and hashing are extremely fast.
- The library uses a global cache for interning, which is protected by a mutex. In highly concurrent scenarios, this could potentially become a bottleneck.
- The `cache_len()` method allows you to monitor the size of the intern cache, which can be useful for understanding memory usage in your application.

## License

This project is licensed under [LICENSE NAME] - see the [LICENSE.md](LICENSE.md) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.