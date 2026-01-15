use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::Io;

/// Bincode serialization IO.
///
/// Bincode is a compact binary format that is fast and produces small output.
/// It requires types to implement serde's `Serialize` and `Deserialize` traits.
#[derive(Debug, Clone, Copy, Default)]
pub struct BincodeIo;

#[derive(Debug, Error)]
#[error("failed to process bincode data: {0}")]
pub struct BincodeError(#[from] bincode::Error);

impl<T> Io<T> for BincodeIo
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    type Error = BincodeError;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        Ok(bincode::serialize(&value)?)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(bincode::deserialize(bytes)?)
    }
}
