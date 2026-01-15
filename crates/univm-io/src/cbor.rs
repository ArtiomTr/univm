use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Io;

/// CBOR serialization IO.
///
/// CBOR (Concise Binary Object Representation) is a binary format designed
/// for small message size and extensibility. It requires types to implement
/// serde's `Serialize` and `Deserialize` traits.
#[derive(Debug, Clone, Copy, Default)]
pub struct CborIo;

#[derive(Debug, Error)]
pub enum CborError {
    #[error("failed to serialize cbor data: {0}")]
    Serialize(#[from] ciborium::ser::Error<std::io::Error>),

    #[error("failed to deserialize cbor data: {0}")]
    Deserialize(#[from] ciborium::de::Error<std::io::Error>),
}

impl<T> Io<T> for CborIo
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = CborError;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        let mut buf = Vec::new();
        ciborium::into_writer(&value, &mut buf)?;
        Ok(buf)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(ciborium::from_reader(bytes)?)
    }
}
