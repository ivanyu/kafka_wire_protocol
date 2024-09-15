// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FindCoordinatorRequest {
    /// The coordinator key type. (Group, transaction, etc.)
    pub key_type: i8,
    /// The coordinator keys.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub coordinator_keys: Vec<String>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for FindCoordinatorRequest {
    fn api_key(&self) -> i16 {
        10
    }
    
    fn version(&self) -> i16 {
        5
    }
}

impl Request for FindCoordinatorRequest { }

impl Default for FindCoordinatorRequest {
    fn default() -> Self {
        FindCoordinatorRequest {
            key_type: 0_i8,
            coordinator_keys: Vec::<String>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FindCoordinatorRequest {
    pub fn new(key_type: i8, coordinator_keys: Vec<String>) -> Self {
        Self {
            key_type,
            coordinator_keys,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_find_coordinator_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FindCoordinatorRequest::new(
            0_i8,
            Vec::<String>::new(),
        );
        assert_eq!(d, FindCoordinatorRequest::default());
    }
}

impl Readable for FindCoordinatorRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let key_type = i8::read(input)?;
        let coordinator_keys = read_array::<String>(input, "coordinator_keys", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FindCoordinatorRequest {
            key_type, coordinator_keys, _unknown_tagged_fields
        })
    }
}

impl Writable for FindCoordinatorRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key_type.write(output)?;
        write_array(output, "self.coordinator_keys", &self.coordinator_keys, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<FindCoordinatorRequest>("FindCoordinatorRequest", 5);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: FindCoordinatorRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: FindCoordinatorRequest) {
            crate::test_utils::test_java_arbitrary(&data, "FindCoordinatorRequest", 5);
        }
    }
}
