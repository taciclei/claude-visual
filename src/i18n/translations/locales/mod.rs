//! Locale-specific translation bundles

pub mod de;
pub mod en;
pub mod es;
pub mod fr;
pub mod ja;
pub mod pt;
pub mod zh;

pub use de::german_bundle;
pub use en::english_bundle;
pub use es::spanish_bundle;
pub use fr::french_bundle;
pub use ja::japanese_bundle;
pub use pt::portuguese_bundle;
pub use zh::chinese_bundle;
