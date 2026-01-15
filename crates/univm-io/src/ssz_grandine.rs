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

    /// Serializes a value into its SSZ byte representation.
    ///
    /// Returns the SSZ-encoded bytes of `value`.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let io = SszGrandineIo;
    /// let value = 42u64; // type must implement `SszReadDefault` + `SszWrite`
    /// let bytes = io.serialize(value).unwrap();
    /// assert!(!bytes.is_empty());
    /// ```
    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        Ok(value.to_ssz()?)
    }

    /// Deserializes SSZ-encoded bytes into the target type.
    ///
    /// On success, returns the deserialized value of type `T`. On failure, returns
    /// a `SszGrandineError::Read` wrapping the underlying read/deserialization error.
    ///
    /// # Examples
    ///
    /// ```
    /// use univm_io::ssz_grandine::SszGrandineIo;
    ///
    /// let io = SszGrandineIo::default();
    /// // `u64` is shown as an example type that implements `SszReadDefault` + `SszWrite`.
    /// let bytes = 42u64.to_ssz();
    /// let value: u64 = io.deserialize(&bytes).unwrap();
    /// assert_eq!(value, 42u64);
    /// ```
    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(T::from_ssz_default(bytes)?)
    }
}