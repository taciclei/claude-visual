//! Async helper functions

use std::sync::Arc;
use tokio::sync::oneshot;

use super::loader::ThemeLoader;
use super::types::{ThemeLoadCallback, ThemeLoadResult};

/// Start loading a theme in the background with callback
pub fn load_theme_async(loader: Arc<ThemeLoader>, name: String, callback: ThemeLoadCallback) {
    tokio::spawn(async move {
        let result = loader.load_theme(&name).await;
        callback(result);
    });
}

/// Start loading a theme with oneshot channel
pub fn load_theme_with_channel(
    loader: Arc<ThemeLoader>,
    name: String,
) -> oneshot::Receiver<ThemeLoadResult> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let result = loader.load_theme(&name).await;
        let _ = tx.send(result);
    });

    rx
}
