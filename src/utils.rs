use std::hash;
use std::hash::Hash;

pub fn get_qpi_key_headers(api_key: &str) -> vec {
    return ["X-Hexlet-Api-Key", api_key]
}
