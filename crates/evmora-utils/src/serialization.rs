use serde::{Serialize, Deserialize};

pub fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(value)
}

pub fn from_slice<'a, T: Deserialize<'a>>(slice: &'a [u8]) -> Result<T, serde_json::Error> {
    serde_json::from_slice(slice)
}
