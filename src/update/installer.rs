//! Update Installer
//!
//! Handles downloading and installing updates.

use anyhow::{anyhow, Result};
use std::path::PathBuf;

use super::UpdateInfo;

/// Progress during update installation
#[derive(Debug, Clone)]
pub enum InstallProgress {
    /// Starting download
    Starting,
    /// Downloading update (progress percentage)
    Downloading(f32),
    /// Download complete, verifying
    Verifying,
    /// Installing update
    Installing,
    /// Update complete, restart required
    Complete,
    /// Installation failed
    Failed(String),
}

/// Update installer
pub struct UpdateInstaller {
    /// Directory for downloaded updates
    download_dir: PathBuf,
    /// Current update being installed
    current_update: Option<UpdateInfo>,
}

impl UpdateInstaller {
    /// Create a new update installer
    pub fn new() -> Result<Self> {
        let download_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow!("Could not find cache directory"))?
            .join("claude-visual")
            .join("updates");

        std::fs::create_dir_all(&download_dir)?;

        Ok(Self {
            download_dir,
            current_update: None,
        })
    }

    /// Download and install an update
    pub async fn install(&mut self, update: UpdateInfo) -> Result<()> {
        self.current_update = Some(update.clone());

        // Step 1: Download the update
        let download_path = self.download(&update).await?;

        // Step 2: Verify the download
        self.verify(&download_path)?;

        // Step 3: Install the update
        self.install_update(&download_path)?;

        Ok(())
    }

    /// Download the update file
    async fn download(&self, update: &UpdateInfo) -> Result<PathBuf> {
        let filename = update
            .download_url
            .split('/')
            .last()
            .unwrap_or("update.dmg");
        let download_path = self.download_dir.join(filename);

        // In a real implementation, use reqwest with streaming to download
        // and report progress

        // Placeholder - would download file here
        // let response = reqwest::get(&update.download_url).await?;
        // let mut file = File::create(&download_path)?;
        // let content = response.bytes().await?;
        // file.write_all(&content)?;

        Ok(download_path)
    }

    /// Verify the downloaded update
    fn verify(&self, _path: &PathBuf) -> Result<()> {
        // In production, verify checksum/signature
        // For now, just check file exists
        Ok(())
    }

    /// Install the update on macOS
    #[cfg(target_os = "macos")]
    fn install_update(&self, dmg_path: &PathBuf) -> Result<()> {
        use std::process::Command;

        // Mount the DMG
        let mount_output = Command::new("hdiutil")
            .args(["attach", dmg_path.to_str().unwrap(), "-nobrowse", "-quiet"])
            .output()?;

        if !mount_output.status.success() {
            return Err(anyhow!("Failed to mount DMG"));
        }

        // Find the mounted volume
        let volume_path = PathBuf::from("/Volumes/Claude Visual");

        // Get the app bundle from the volume
        let app_source = volume_path.join("Claude Visual.app");
        let app_dest = PathBuf::from("/Applications/Claude Visual.app");

        // Remove old version and copy new
        if app_dest.exists() {
            std::fs::remove_dir_all(&app_dest)?;
        }

        // Use ditto for proper resource fork handling
        let copy_output = Command::new("ditto")
            .args([
                app_source.to_str().unwrap(),
                app_dest.to_str().unwrap(),
            ])
            .output()?;

        if !copy_output.status.success() {
            return Err(anyhow!("Failed to copy application"));
        }

        // Unmount the DMG
        Command::new("hdiutil")
            .args(["detach", volume_path.to_str().unwrap(), "-quiet"])
            .output()?;

        // Clean up downloaded DMG
        std::fs::remove_file(dmg_path)?;

        Ok(())
    }

    /// Install the update on Linux
    #[cfg(target_os = "linux")]
    fn install_update(&self, archive_path: &PathBuf) -> Result<()> {
        use std::process::Command;

        // For Linux, assume it's a tar.gz archive
        let install_dir = dirs::data_local_dir()
            .ok_or_else(|| anyhow!("Could not find data directory"))?
            .join("claude-visual");

        std::fs::create_dir_all(&install_dir)?;

        // Extract the archive
        let output = Command::new("tar")
            .args([
                "-xzf",
                archive_path.to_str().unwrap(),
                "-C",
                install_dir.to_str().unwrap(),
            ])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Failed to extract update"));
        }

        // Clean up
        std::fs::remove_file(archive_path)?;

        Ok(())
    }

    /// Request application restart
    pub fn request_restart(&self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            // Get the path to the current executable
            let current_exe = std::env::current_exe()?;
            let app_path = current_exe
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .ok_or_else(|| anyhow!("Could not determine app path"))?;

            // Launch the new version
            Command::new("open")
                .args(["-n", app_path.to_str().unwrap()])
                .spawn()?;
        }

        Ok(())
    }
}

impl Default for UpdateInstaller {
    fn default() -> Self {
        Self::new().expect("Failed to create update installer")
    }
}
