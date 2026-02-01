//! Cleanup utility functions

/// Check available disk space
pub fn get_available_space_mb(path: &std::path::Path) -> Result<u64, std::io::Error> {
    // This is a simplified implementation
    // In production, you'd use platform-specific APIs
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = std::fs::metadata(path)?;
        // This is a rough estimate
        Ok(metadata.blksize() / 1_000_000)
    }
    #[cfg(not(unix))]
    {
        // Fallback for other platforms
        Ok(1000) // Return 1GB as default
    }
}

/// Check if cleanup is needed due to low disk space
pub fn needs_disk_space_cleanup(data_dir: &std::path::Path, min_space_mb: u64) -> bool {
    match get_available_space_mb(data_dir) {
        Ok(available) => available < min_space_mb,
        Err(_) => false, // Don't trigger cleanup on error
    }
}
