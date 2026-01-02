//! SMART 功能模块

pub mod attributes;
pub mod blob;
pub mod data;
pub mod parse;
pub mod statistics;

pub use blob::{identify_from_blob, read_blob_from_file, smart_info_from_blob, BlobData};

pub(crate) use attributes::*;
pub(crate) use data::*;
pub(crate) use parse::*;
