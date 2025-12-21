use ssz::{ReadError, SszReadDefault, SszWrite, WriteError};
use thiserror::Error;

use crate::Io;

pub struct SszIo;

#[derive(Debug, Error)]
pub enum SszError {
    #[error("failed to read ssz data: {0:?}")]
    Read(#[from] ReadError),

    #[error("failed to write ssz data: {0:?}")]
    Write(#[from] WriteError),
}

impl<T: SszReadDefault + SszWrite> Io<T> for SszIo {
    type Error = SszError;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        Ok(value.to_ssz()?)
    }

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(T::from_ssz_default(bytes)?)
    }
}
