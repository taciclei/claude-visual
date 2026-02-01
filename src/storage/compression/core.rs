use super::{
    algorithms,
    types::{CompressedData, CompressionAlgorithm, CompressionConfig, CompressionError},
    utils::crc32,
};

/// Message compressor
pub struct Compressor {
    pub(crate) config: CompressionConfig,
}

impl Default for Compressor {
    fn default() -> Self {
        Self::new(CompressionConfig::default())
    }
}

impl Compressor {
    /// Create a new compressor
    pub fn new(config: CompressionConfig) -> Self {
        Self { config }
    }

    /// Compress data
    pub fn compress(&self, data: &[u8]) -> Result<CompressedData, CompressionError> {
        // Skip compression for small data
        if data.len() < self.config.min_size {
            return Ok(CompressedData {
                algorithm: CompressionAlgorithm::None,
                original_size: data.len(),
                compressed_size: data.len(),
                data: data.to_vec(),
                checksum: crc32(data),
            });
        }

        let compressed = match self.config.algorithm {
            CompressionAlgorithm::None => data.to_vec(),
            CompressionAlgorithm::Lz4 => algorithms::compress_lz4(data)?,
            CompressionAlgorithm::Zstd => algorithms::compress_zstd(data)?,
            CompressionAlgorithm::Deflate => algorithms::compress_deflate(data)?,
        };

        let checksum = crc32(&compressed);

        Ok(CompressedData {
            algorithm: self.config.algorithm,
            original_size: data.len(),
            compressed_size: compressed.len(),
            data: compressed,
            checksum,
        })
    }

    /// Decompress data
    pub fn decompress(&self, compressed: &CompressedData) -> Result<Vec<u8>, CompressionError> {
        // Verify checksum
        if !compressed.verify() {
            return Err(CompressionError::ChecksumMismatch);
        }

        match compressed.algorithm {
            CompressionAlgorithm::None => Ok(compressed.data.clone()),
            CompressionAlgorithm::Lz4 => {
                algorithms::decompress_lz4(&compressed.data, compressed.original_size)
            }
            CompressionAlgorithm::Zstd => algorithms::decompress_zstd(&compressed.data),
            CompressionAlgorithm::Deflate => algorithms::decompress_deflate(&compressed.data),
        }
    }

    /// Compress string (convenience method)
    pub fn compress_string(&self, text: &str) -> Result<CompressedData, CompressionError> {
        self.compress(text.as_bytes())
    }

    /// Decompress to string
    pub fn decompress_string(
        &self,
        compressed: &CompressedData,
    ) -> Result<String, CompressionError> {
        let bytes = self.decompress(compressed)?;
        String::from_utf8(bytes).map_err(|_| CompressionError::InvalidUtf8)
    }
}
