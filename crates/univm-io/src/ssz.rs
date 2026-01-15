use ethereum_ssz::{Decode, DecodeError, Encode};
use thiserror::Error;

use crate::Io;

/// SSZ serialization IO using the `ethereum_ssz` crate.
///
/// SSZ (Simple Serialize) is the serialization format used in Ethereum.
/// This implementation uses the standard `ethereum_ssz` crate and requires
/// types to implement the `Encode` and `Decode` traits.
#[derive(Debug, Clone, Copy, Default)]
pub struct SszIo;

#[derive(Debug, Error)]
#[error("failed to decode ssz data: {0:?}")]
pub struct SszError(DecodeError);

impl From<DecodeError> for SszError {
    fn from(err: DecodeError) -> Self {
        SszError(err)
    }
}

impl<T: Encode + Decode> Io<T> for SszIo {
    type Error = SszError;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        Ok(value.as_ssz_bytes())
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(T::from_ssz_bytes(bytes)?)
    }
}
