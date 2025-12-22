#[cfg(feature = "ssz")]
pub mod ssz;

pub trait Io<T> {
    type Error: std::error::Error;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error>;

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error>;
}

/// A simple raw byte I/O implementation that works with types that can be
/// converted to/from byte slices.
pub struct RawIo;

#[derive(Debug, thiserror::Error)]
pub enum RawIoError {
    #[error("Invalid data length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
}

impl Io<()> for RawIo {
    type Error = RawIoError;

    fn serialize(&self, _value: ()) -> Result<Vec<u8>, Self::Error> {
        Ok(Vec::new())
    }

    fn deserialize(&self, _bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}
