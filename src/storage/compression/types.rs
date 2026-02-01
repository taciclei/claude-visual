use serde::{Deserialize, Serialize};

/// Compression algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// No compression
    None,
    /// LZ4 - Fast compression, moderate ratio
    Lz4,
    /// ZSTD - Better ratio, slightly slower
    Zstd,
    /// Deflate - Good compatibility
    Deflate,
}

impl Default for CompressionAlgorithm {
    fn default() -> Self {
        Self::Lz4
    }
}

impl CompressionAlgorithm {
    /// Get algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Lz4 => "lz4",
            Self::Zstd => "zstd",
            Self::Deflate => "deflate",
        }
    }

    /// Parse from name
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "none" => Some(Self::None),
            "lz4" => Some(Self::Lz4),
            "zstd" => Some(Self::Zstd),
            "deflate" => Some(Self::Deflate),
            _ => None,
        }
    }
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Algorithm to use
    pub algorithm: CompressionAlgorithm,
    /// Compression level (1-22 for zstd, 1-16 for lz4)
    pub level: u32,
    /// Minimum size to compress (bytes)
    pub min_size: usize,
    /// Dictionary size for better ratio
    pub dict_size: Option<usize>,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Lz4,
            level: 3,
            min_size: 1024, // Don't compress small messages
            dict_size: None,
        }
    }
}

impl CompressionConfig {
    /// Fast compression (low CPU usage)
    pub fn fast() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Lz4,
            level: 1,
            min_size: 512,
            dict_size: None,
        }
    }

    /// Best compression (smaller files)
    pub fn best() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Zstd,
            level: 19,
            min_size: 256,
            dict_size: Some(65536),
        }
    }

    /// Balanced compression
    pub fn balanced() -> Self {
        Self {
            algorithm: CompressionAlgorithm::Zstd,
            level: 6,
            min_size: 1024,
            dict_size: None,
        }
    }
}

/// Compressed data with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedData {
    /// Compression algorithm used
    pub algorithm: CompressionAlgorithm,
    /// Original size in bytes
    pub original_size: usize,
    /// Compressed size in bytes
    pub compressed_size: usize,
    /// Compressed bytes
    pub data: Vec<u8>,
    /// Checksum (CRC32)
    pub checksum: u32,
}

impl CompressedData {
    /// Get compression ratio (0.0 - 1.0)
    pub fn ratio(&self) -> f32 {
        if self.original_size == 0 {
            return 0.0;
        }
        self.compressed_size as f32 / self.original_size as f32
    }

    /// Get space savings percentage
    pub fn savings(&self) -> f32 {
        (1.0 - self.ratio()) * 100.0
    }

    /// Check if compression was effective
    pub fn is_effective(&self) -> bool {
        self.ratio() < 0.9 // At least 10% savings
    }

    /// Verify checksum
    pub fn verify(&self) -> bool {
        let computed = super::utils::crc32(&self.data);
        computed == self.checksum
    }
}

/// Compression errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum CompressionError {
    #[error("Compression failed")]
    CompressionFailed,
    #[error("Decompression failed")]
    DecompressionFailed,
    #[error("Checksum mismatch")]
    ChecksumMismatch,
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
    #[error("Data too large: {0} bytes")]
    DataTooLarge(usize),
    #[error("Unknown algorithm: {0}")]
    UnknownAlgorithm(String),
}

/// Compression statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompressionStats {
    /// Total items compressed
    pub items_compressed: usize,
    /// Total original bytes
    pub original_bytes: u64,
    /// Total compressed bytes
    pub compressed_bytes: u64,
    /// Best ratio achieved
    pub best_ratio: f32,
    /// Worst ratio achieved
    pub worst_ratio: f32,
    /// Average ratio
    pub avg_ratio: f32,
}

impl CompressionStats {
    /// Create new stats
    pub fn new() -> Self {
        Self {
            best_ratio: 1.0,
            worst_ratio: 0.0,
            ..Default::default()
        }
    }

    /// Update stats with a compression result
    pub fn update(&mut self, compressed: &CompressedData) {
        self.items_compressed += 1;
        self.original_bytes += compressed.original_size as u64;
        self.compressed_bytes += compressed.compressed_size as u64;

        let ratio = compressed.ratio();
        self.best_ratio = self.best_ratio.min(ratio);
        self.worst_ratio = self.worst_ratio.max(ratio);
        self.avg_ratio = self.compressed_bytes as f32 / self.original_bytes.max(1) as f32;
    }

    /// Get total space saved
    pub fn space_saved(&self) -> u64 {
        self.original_bytes.saturating_sub(self.compressed_bytes)
    }

    /// Get space saved percentage
    pub fn savings_percent(&self) -> f32 {
        (1.0 - self.avg_ratio) * 100.0
    }
}
