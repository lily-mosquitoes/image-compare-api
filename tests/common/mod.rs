#![allow(dead_code)]

use chrono::{
    DateTime,
    Utc,
};
use rand::RngCore;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ErrResponse<E> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) error: E,
}

#[derive(Deserialize)]
pub(crate) struct OkResponse<T> {
    pub(crate) request_id: usize,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) data: T,
}

const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

pub(crate) fn generate_random_hex_string(size: usize) -> String {
    let upper_nibble = |b| HEX_CHARS[(b >> 4) as usize] as char;
    let lower_nibble = |b| HEX_CHARS[(b & 0x0f) as usize] as char;
    let as_hex = |b| format!("{}{}", upper_nibble(b), lower_nibble(b));

    let mut bytes = vec![0; size];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| as_hex(b)).collect()
}
