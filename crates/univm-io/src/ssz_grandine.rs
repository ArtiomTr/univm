use ssz_grandine::{ReadError, SszReadDefault, SszWrite, WriteError};
use thiserror::Error;

use crate::Io;

/// SSZ serialization IO using Grandine's implementation.
///
/// SSZ (Simple Serialize) is the serialization format used in Ethereum.
/// This implementation uses Grandine's SSZ library and requires types
/// to implement `SszReadDefault` and `SszWrite` traits.
#[derive(Debug, Clone, Copy, Default)]
pub struct SszGrandineIo;

#[derive(Debug, Error)]
pub enum SszGrandineError {
    #[error("failed to read ssz data: {0:?}")]
    Read(#[from] ReadError),

    #[error("failed to write ssz data: {0:?}")]
    Write(#[from] WriteError),
}

impl<T: SszReadDefault + SszWrite> Io<T> for SszGrandineIo {
    type Error = SszGrandineError;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        Ok(value.to_ssz()?)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(T::from_ssz_default(bytes)?)
    }
}
