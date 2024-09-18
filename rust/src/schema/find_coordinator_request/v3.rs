// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// FindCoordinatorRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FindCoordinatorRequest {
    /// The coordinator key.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub key: String,
    /// The coordinator key type. (Group, transaction, etc.)
    pub key_type: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for FindCoordinatorRequest {
    fn api_key(&self) -> i16 {
        10
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for FindCoordinatorRequest { }

impl Default for FindCoordinatorRequest {
    fn default() -> Self {
        FindCoordinatorRequest {
            key: String::from(""),
            key_type: 0_i8,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FindCoordinatorRequest {
    pub fn new<S1: AsRef<str>>(key: S1, key_type: i8) -> Self {
        Self {
            key: key.as_ref().to_string(),
            key_type,
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
            String::from(""),
            0_i8,
        );
        assert_eq!(d, FindCoordinatorRequest::default());
    }
}

impl Readable for FindCoordinatorRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let key = String::read_ext(input, "key", true)?;
        let key_type = i8::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FindCoordinatorRequest {
            key, key_type, _unknown_tagged_fields
        })
    }
}

impl Writable for FindCoordinatorRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key.write_ext(output, "self.key", true)?;
        self.key_type.write(output)?;
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
        crate::test_utils::test_java_default::<FindCoordinatorRequest>("FindCoordinatorRequest", 3);
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
            crate::test_utils::test_java_arbitrary(&data, "FindCoordinatorRequest", 3);
        }
    }
}
