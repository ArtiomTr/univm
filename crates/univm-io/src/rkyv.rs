use rkyv::{
    api::high::{HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    de::Pool,
    rancor::{BoxedError, Strategy},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    Archive, Deserialize, Serialize,
};
use thiserror::Error;

use crate::Io;

/// Zero-copy serialization IO using rkyv.
///
/// rkyv provides extremely fast serialization and zero-copy deserialization,
/// making it ideal for performance-critical applications.
///
/// # Example
///
/// ```ignore
/// use univm_io::rkyv::RkyvIo;
/// use rkyv::{Archive, Serialize, Deserialize};
///
/// #[derive(Archive, Serialize, Deserialize)]
/// struct MyData {
///     field: u32,
/// }
///
/// let io = RkyvIo;
/// let bytes = io.serialize(my_data)?;
/// let value: MyData = io.deserialize(&bytes)?;
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct RkyvIo;

#[derive(Debug, Error)]
pub enum RkyvError {
    #[error("failed to serialize data: {0}")]
    Serialize(BoxedError),

    #[error("failed to deserialize data: {0}")]
    Deserialize(BoxedError),
}

impl<T> Io<T> for RkyvIo
where
    T: Archive + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, BoxedError>>,
    T::Archived: Deserialize<T, Strategy<Pool, BoxedError>>
        + for<'a> CheckBytes<HighValidator<'a, BoxedError>>,
{
    type Error = RkyvError;

    /// Serialize a value into a rkyv-encoded byte vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::rkyv::RkyvIo;
    ///
    /// let io = RkyvIo;
    /// let bytes = io.serialize(42u32).expect("serialization failed");
    /// assert!(!bytes.is_empty());
    /// ```
    ///
    /// # Returns
    ///
    /// Serialized bytes as a `Vec<u8>`.
    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error> {
        rkyv::to_bytes::<BoxedError>(&value)
            .map(|aligned| aligned.to_vec())
            .map_err(RkyvError::Serialize)
    }

    /// Deserializes a value of type `T` from a Rkyv-encoded byte slice.
    ///
    /// Attempts to validate and reconstruct the original value from `bytes`.
    ///
    /// # Returns
    ///
    /// The deserialized value of type `T`.
    ///
    /// # Errors
    ///
    /// Returns `RkyvError::Deserialize` if validation or reconstruction fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use univm_io::RkyvIo;
    ///
    /// // `MyType` must implement the rkyv archive/serialize/deserialize traits required by RkyvIo.
    /// let io = RkyvIo::default();
    /// let original = MyType::example();
    /// let bytes = io.serialize(original).unwrap();
    /// let recovered: MyType = io.deserialize(&bytes).unwrap();
    /// ```
    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error> {
        rkyv::from_bytes::<T, BoxedError>(bytes).map_err(RkyvError::Deserialize)
    }
}