use std::marker::PhantomData;

use ssz::{ReadError, SszReadDefault, SszWrite, WriteError};
use thiserror::Error;

use crate::Io;

pub struct SszIo<T>(PhantomData<T>);

#[derive(Debug, Error)]
pub enum SszError {
    #[error("failed to read ssz data: {0:?}")]
    Read(#[from] ReadError),

    #[error("failed to write ssz data: {0:?}")]
    Write(#[from] WriteError),
}

impl<T: SszReadDefault + SszWrite> Io for SszIo<T> {
    type Value = T;

    type Error = SszError;

    fn serialize(value: Self::Value) -> Result<Vec<u8>, Self::Error> {
        Ok(value.to_ssz()?)
    }

    fn deserialize(bytes: &[u8]) -> Result<Self::Value, Self::Error> {
        Ok(Self::Value::from_ssz_default(bytes)?)
    }
}
