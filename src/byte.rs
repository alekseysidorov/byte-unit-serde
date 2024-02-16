use byte_unit::Byte;
use serde::{Deserialize, Serialize, Serializer};

pub fn deserialize<'a, T, D>(der: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'a>,
    Scalar<T>: From<Byte>,
{
    let byte = Byte::deserialize(der)?;
    Ok(Scalar::<T>::from(byte).0)
}

pub fn serialize<T, S>(val: &T, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    Byte: for<'a> From<Scalar<&'a T>>,
{
    let byte = Byte::from(Scalar(val));
    byte.serialize(ser)
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Scalar<T>(T);

impl From<Byte> for Scalar<u64> {
    fn from(value: Byte) -> Self {
        Self(value.as_u64())
    }
}

impl From<Scalar<&u64>> for Byte {
    fn from(value: Scalar<&u64>) -> Self {
        Byte::from_u64(*value.0)
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
            (json!({ "size": "1024kB" }), 1024000),
            (json!({ "size": "1024MiB" }), 1073741824),
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
            (Values { size: 1024000 }, json!({ "size": "1.024 MB" })),
            (Values { size: 1073741824 }, json!({ "size": "1 GiB" })),
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
