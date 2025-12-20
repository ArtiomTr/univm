#[cfg(feature = "ssz")]
pub mod ssz;

pub trait Io {
    type Value;
    type Error;

    fn serialize(value: Self::Value) -> Result<Vec<u8>, Self::Error>;

    fn deserialize(bytes: &[u8]) -> Result<Self::Value, Self::Error>;
}
