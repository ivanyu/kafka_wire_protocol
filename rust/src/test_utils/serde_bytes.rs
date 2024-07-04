use base64::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S: Serializer>(v: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
    let base64 = BASE64_STANDARD.encode(v);
    String::serialize(&base64, serializer)
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let base64 = String::deserialize(deserializer)?;
    BASE64_STANDARD.decode(base64.as_bytes())
        .map_err(|e| serde::de::Error::custom(e))
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
        #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
        data: Vec<u8>,
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