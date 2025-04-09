use flarrow_message::prelude::*;

use arrow_array::{Array, UInt8Array};

#[derive(Debug, ArrowMessage)]
pub struct Metadata {
    pub name: Option<String>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, ArrowMessage)]
pub struct Image {
    pub data: UInt8Array,
    pub metadata: Option<Metadata>,
}
