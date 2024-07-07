use base64::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error> {
    match value {
        Some(v) => {
            let base64 = BASE64_STANDARD.encode(v);
            String::serialize(&base64, serializer)
        }

        None => serializer.serialize_none()
    }
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error> {
    match Option::<String>::deserialize(deserializer)? {
        Some(s) =>
            BASE64_STANDARD.decode(s.as_bytes())
                .map(Some)
                .map_err(|e| serde::de::Error::custom(e)),

        None => Ok(None)
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;
    use serde::{Deserialize, Serialize};

    use crate::test_utils::proptest_strategies;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    #[cfg_attr(test, derive(Arbitrary))]
    struct TestStruct {
        #[serde(with = "super")]
        #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
        data: Option<Vec<u8>>,
    }

    proptest! {
        #[test]
        fn test_serde(original_data: TestStruct) {
            let string = serde_json::to_string(&original_data).unwrap();
            let read_data = serde_json::from_str::<TestStruct>(&string).unwrap();
            prop_assert_eq!(read_data, original_data);
        }
    }
}