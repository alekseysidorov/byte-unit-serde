//! A [`serde`] wrapper, that can be used to serialize and deserialize [`u64`]
//! types using [`Byte`] representation.

use std::marker::PhantomData;

use byte_unit::Byte;
use serde::{Deserialize, Serialize, Serializer};

/// Serializes a [`u64`] via [`Byte`] type.
///
/// This function is desined to use with the `serde_derive`'s
/// `with` and `serialize_with` annotations.
pub fn deserialize<'a, T, D>(der: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'a>,
    T: TryFrom<Deser<'a, D>, Error = D::Error>,
{
    T::try_from(Deser(der, PhantomData))
}

/// Deserializes a [`u64`] via [`byte_unit::Byte`] type.
///
/// This function is desined to use with the `serde_derive`'s
/// `with` and `serialize_with` annotations.
pub fn serialize<T, S>(val: &T, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    for<'a> Ser<&'a T>: Serialize,
{
    Ser::from(val).serialize(ser)
}

/// A wrapper type which provides [`TryFrom`] implementation for types which
/// can be deserialized by using the [`Byte`] type's [`Deserialize`] implementation.
#[derive(Debug)]
pub struct Deser<'a, D>(D, PhantomData<&'a ()>);

impl<'a, D> TryFrom<Deser<'a, D>> for u64
where
    D: serde::Deserializer<'a>,
{
    type Error = D::Error;

    fn try_from(value: Deser<'a, D>) -> Result<Self, Self::Error> {
        let byte = Byte::deserialize(value.0)?;
        Ok(byte.as_u64())
    }
}

impl<'a, D> TryFrom<Deser<'a, D>> for Option<u64>
where
    D: serde::Deserializer<'a>,
{
    type Error = D::Error;

    fn try_from(value: Deser<'a, D>) -> Result<Self, Self::Error> {
        let byte = Option::<Byte>::deserialize(value.0)?;
        Ok(byte.map(Byte::as_u64))
    }
}

/// A wrapper which provides [`Serialize`] implementation for types which
/// can be serialized by using the [`Byte`] type's [`Serialize`] implementation.
#[derive(Debug)]
pub struct Ser<T>(T);

impl Serialize for Ser<&u64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Byte::from_u64(*self.0).serialize(serializer)
    }
}

impl Serialize for Ser<&Option<u64>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.map(Byte::from_u64).serialize(serializer)
    }
}

impl<T> From<T> for Ser<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    struct Values {
        #[serde(with = "super")]
        size: u64,
    }

    #[test]
    fn test_deserialize_byte() {
        let test_cases = [
            (json!({ "size": "1024kB" }), 1_024_000),
            (json!({ "size": "1024MiB" }), 1_073_741_824),
            (json!({ "size": "1kB" }), 1000),
            (json!({ "size": "1kiB" }), 1024),
            (json!({ "size": "1" }), 1),
        ];

        for (json, expected) in test_cases {
            let values: Values = serde_json::from_value(json).unwrap();
            assert_eq!(values.size, expected);
        }
    }

    #[test]
    fn test_serialize_byte() {
        let test_cases = [
            (Values { size: 1_024_000 }, json!({ "size": "1.024 MB" })),
            (
                Values {
                    size: 1_073_741_824,
                },
                json!({ "size": "1 GiB" }),
            ),
            (Values { size: 1000 }, json!({ "size": "1 KB" })),
            (Values { size: 1024 }, json!({ "size": "1 KiB" })),
            (Values { size: 1 }, json!({ "size": "1 B" })),
        ];

        for (value, expected) in test_cases {
            let json = json!(value);
            assert_eq!(json, expected);
        }
    }
}
