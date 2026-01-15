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

    /// Serializes a value into bincode-encoded bytes.
    ///
    /// # Returns
    ///
    /// `Vec<u8>` containing the bincode encoding of `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::Serialize;
    /// use univm_io::bincode::BincodeIo;
    ///
    /// #[derive(Serialize)]
    /// struct S(u8);
    ///
    /// let io = BincodeIo;
    /// let bytes = io.serialize(S(42)).unwrap();
    /// assert!(!bytes.is_empty());
    /// ```
    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        Ok(bincode::serialize(&value)?)
    }

    /// Deserializes a byte slice into a value of type `T` using bincode.
    ///
    /// # Returns
    ///
    /// `Ok(T)` containing the deserialized value, `Err(BincodeError)` if deserialization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// struct Item { x: i32 }
    ///
    /// let io = BincodeIo;
    /// let value = Item { x: 42 };
    /// let bytes = bincode::serialize(&value).unwrap();
    /// let decoded = io.deserialize(&bytes).unwrap();
    /// assert_eq!(decoded, value);
    /// ```
    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        Ok(bincode::deserialize(bytes)?)
    }
}