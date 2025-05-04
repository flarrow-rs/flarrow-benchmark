use iridis_message::prelude::{thirdparty::arrow_array::*, *};

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

pub const SIZES: [usize; 10] = [
    1,
    8,
    64,
    512,
    2048,
    4096,
    4 * 4096,
    10 * 4096,
    100 * 4096,
    1000 * 4096,
];

pub const BENCH_LEN: usize = 100;
