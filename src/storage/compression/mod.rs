//! Message Compression
//!
//! Provides compression for archived messages to reduce storage space.
//! Uses LZ4 for fast compression and optional ZSTD for better ratios.

mod algorithms;
mod core;
mod types;
mod utils;

pub use core::Compressor;
pub use types::{
    CompressionAlgorithm, CompressionConfig, CompressionError, CompressionStats, CompressedData,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_roundtrip() {
        let compressor = Compressor::default();
        let original = "Hello, World! This is a test message that should be compressed.".repeat(10);

        let compressed = compressor.compress_string(&original).unwrap();
        let decompressed = compressor.decompress_string(&compressed).unwrap();

        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_compression_small_data() {
        let compressor = Compressor::default();
        let small = "Hi";

        let compressed = compressor.compress_string(small).unwrap();
        assert_eq!(compressed.algorithm, CompressionAlgorithm::None);
        assert_eq!(compressed.original_size, compressed.compressed_size);
    }

    #[test]
    fn test_compression_ratio() {
        let compressor = Compressor::default();
        // Highly compressible data
        let data = "a".repeat(10000);

        let compressed = compressor.compress_string(&data).unwrap();
        assert!(compressed.ratio() < 0.5); // Should compress well
    }

    #[test]
    fn test_crc32() {
        let data = b"hello";
        let checksum1 = utils::crc32(data);
        let checksum2 = utils::crc32(data);
        assert_eq!(checksum1, checksum2);

        let different = b"world";
        let checksum3 = utils::crc32(different);
        assert_ne!(checksum1, checksum3);
    }

    #[test]
    fn test_compression_stats() {
        let mut stats = CompressionStats::new();

        let compressed = CompressedData {
            algorithm: CompressionAlgorithm::Lz4,
            original_size: 1000,
            compressed_size: 500,
            data: vec![],
            checksum: 0,
        };

        stats.update(&compressed);
        assert_eq!(stats.items_compressed, 1);
        assert_eq!(stats.space_saved(), 500);
        assert!(stats.savings_percent() > 40.0);
    }

    #[test]
    fn test_algorithm_names() {
        assert_eq!(CompressionAlgorithm::Lz4.name(), "lz4");
        assert_eq!(CompressionAlgorithm::from_name("zstd"), Some(CompressionAlgorithm::Zstd));
        assert_eq!(CompressionAlgorithm::from_name("unknown"), None);
    }
}
