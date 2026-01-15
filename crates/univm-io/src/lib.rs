#[cfg(feature = "ssz")]
pub mod ssz;

#[cfg(feature = "ssz_grandine")]
pub mod ssz_grandine;

#[cfg(feature = "bincode")]
pub mod bincode;

#[cfg(feature = "cbor")]
pub mod cbor;

#[cfg(feature = "rkyv")]
pub mod rkyv;

pub trait Io<T> {
    type Error: std::error::Error;

    fn serialize(&self, value: T) -> Result<Vec<u8>, Self::Error>;

    fn deserialize(&self, bytes: &[u8]) -> Result<T, Self::Error>;
}
