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
    /// Convert a `DecodeError` into an `SszError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ethereum_ssz::DecodeError;
    /// // construct or obtain a `DecodeError` somehow
    /// let decode_err: DecodeError = DecodeError::InvalidLength;
    /// let ssz_err: SszError = SszError::from(decode_err);
    /// ```
    fn from(err: DecodeError) -> Self {
        SszError(err)
    }
}

impl<T: Encode + Decode> Io<T> for SszIo {
    type Error = SszError;

    /// Serializes a value into SSZ-encoded bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use univm_io::ssz::SszIo;
    /// let io = SszIo::default();
    /// let bytes = io.serialize(42u64).unwrap();
    /// assert!(!bytes.is_empty());
    /// ```
    ///
    /// # Returns
    ///
    /// `Vec<u8>` containing the SSZ encoding of `value`.
    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        Ok(value.as_ssz_bytes())
    }

    /// Deserializes a value of type `T` from SSZ-encoded bytes.
    ///
    /// # Parameters
    ///
    /// - `bytes`: SSZ-encoded bytes representing a value of type `T`.
    ///
    /// # Returns
    ///
    /// `Ok(T)` containing the decoded value on success, `Err(SszError)` if decoding fails.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assume `MyType` implements `ethereum_ssz::Encode + ethereum_ssz::Decode`.
    /// let io = SszIo::default();
    /// let bytes: Vec<u8> = /* SSZ bytes for MyType */ vec![];
    /// let value: MyType = io.deserialize(&bytes).unwrap();
    /// ```
    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(T::from_ssz_bytes(bytes)?)
    }
}