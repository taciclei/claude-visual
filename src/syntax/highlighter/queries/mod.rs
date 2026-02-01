//! Tree-sitter highlight queries for supported languages

mod bash;
mod javascript;
mod json;
mod python;
mod rust;
mod toml;
mod typescript;

pub(crate) use bash::BASH_HIGHLIGHTS;
pub(crate) use javascript::JS_HIGHLIGHTS;
pub(crate) use json::JSON_HIGHLIGHTS;
pub(crate) use python::PYTHON_HIGHLIGHTS;
pub(crate) use rust::RUST_HIGHLIGHTS;
pub(crate) use toml::TOML_HIGHLIGHTS;
pub(crate) use typescript::TS_HIGHLIGHTS;
