//! Events emitted by the image input component

use super::attachment::ImageAttachment;

/// Events emitted by the image input
#[derive(Debug, Clone)]
pub enum ImageInputEvent {
    /// Images were attached
    ImagesAttached(Vec<ImageAttachment>),
    /// Image was removed
    ImageRemoved(usize),
    /// Clear all images
    ClearImages,
}
