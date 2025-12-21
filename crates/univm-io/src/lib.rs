#[cfg(feature = "ssz")]
pub mod ssz;

pub trait Io<T> {
    type Error: std::error::Error;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error>;

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error>;
}
