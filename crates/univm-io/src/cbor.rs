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

    /// Serializes a value to CBOR and returns the resulting byte vector.
    ///
    /// On success returns the CBOR-encoded bytes representing `value`.
    ///
    /// # Errors
    ///
    /// Returns a `CborError::Serialize` if the value cannot be encoded as CBOR.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::Serialize;
    ///
    /// let io = CborIo;
    /// let bytes = io.serialize(42u32).unwrap();
    /// assert!(!bytes.is_empty());
    /// ```
    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        let mut buf = Vec::new();
        ciborium::into_writer(&value, &mut buf)?;
        Ok(buf)
    }

    /// Deserializes a value of type `T` from CBOR-encoded bytes.
    ///
    /// # Returns
    ///
    /// `Ok(T)` with the decoded value, `Err(CborError)` if CBOR deserialization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// struct S(u8);
    ///
    /// let io = CborIo;
    /// let bytes = io.serialize(S(7)).unwrap();
    /// let decoded: S = io.deserialize(&bytes).unwrap();
    /// assert_eq!(decoded, S(7));
    /// ```
    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(ciborium::from_reader(bytes)?)
    }
}