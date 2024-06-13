use proptest::num::u128::Any;
use proptest::prelude::{any, Arbitrary, Strategy};
use proptest::strategy::Map;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid as UuidNormal;

#[derive(Deserialize, Debug)]
pub struct Uuid {
    normal_uuid: UuidNormal,
}

impl Uuid {
    pub fn from_u128(v: u128) -> Self {
        Self {
            normal_uuid: UuidNormal::from_u128(v)
        }
    }
}

impl Serialize for Uuid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.normal_uuid.serialize(serializer)
    }
}

impl Arbitrary for Uuid {
    type Parameters = ();

    fn arbitrary_with(#[allow(unused)] args: Self::Parameters) -> Self::Strategy {
        any::<u128>().prop_map(Uuid::from_u128)
    }

    type Strategy = Map<Any, fn(u128) -> Uuid>;
}

#[cfg(test)]
mod tests {
    use proptest::prelude::ProptestConfig;
    use proptest::proptest;
    use super::*;

    #[test]
    fn test_json_serialization() {
        let uuid = Uuid::from_u128(0);
        let str = serde_json::to_string(&uuid).expect("should work");
        assert_eq!(str, "\"00000000-0000-0000-0000-000000000000\"");

        let uuid = Uuid::from_u128(u128::MAX);
        let str = serde_json::to_string(&uuid).expect("should work");
        assert_eq!(str, "\"ffffffff-ffff-ffff-ffff-ffffffffffff\"");
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn test_generation(uuid in any::<Uuid>()) {
            serde_json::to_string(&uuid).expect("should work");
        }
    }
}
