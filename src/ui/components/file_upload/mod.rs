//! File upload and dropzone components

mod types;
mod dropzone;
mod preview;
mod upload_list;
mod image;

pub use types::*;
pub use dropzone::FileDropzone;
pub use preview::FilePreview;
pub use upload_list::FileUploadList;
pub use image::ImageUpload;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_size_format() {
        assert_eq!(FilePreview::format_size(500), "500 B");
        assert_eq!(FilePreview::format_size(2048), "2.0 KB");
        assert_eq!(FilePreview::format_size(1048576), "1.0 MB");
        assert_eq!(FilePreview::format_size(1073741824), "1.00 GB");
    }

    #[test]
    fn test_file_size_limit() {
        assert_eq!(FileSizeLimit::kb(100).max_bytes, 102400);
        assert_eq!(FileSizeLimit::mb(5).max_bytes, 5242880);
        assert_eq!(FileSizeLimit::gb(1).max_bytes, 1073741824);
    }

    #[test]
    fn test_file_dropzone() {
        let dropzone = FileDropzone::new()
            .label("Upload files")
            .accept(vec!["pdf", "doc"])
            .size_limit(FileSizeLimit::mb(25));

        assert_eq!(dropzone.label, "Upload files");
        assert_eq!(dropzone.accepted_types.len(), 2);
    }

    #[test]
    fn test_file_preview() {
        let preview = FilePreview::new("document.pdf", 1024000)
            .file_type("pdf")
            .progress(50.0);

        assert_eq!(preview.filename, "document.pdf");
        assert_eq!(preview.progress, Some(50.0));
    }
}
