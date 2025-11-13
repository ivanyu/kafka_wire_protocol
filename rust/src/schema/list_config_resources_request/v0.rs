// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListConfigResourcesRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListConfigResourcesRequest {
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListConfigResourcesRequest {
    fn api_key(&self) -> i16 {
        74
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ListConfigResourcesRequest { }

impl Default for ListConfigResourcesRequest {
    fn default() -> Self {
        ListConfigResourcesRequest {
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListConfigResourcesRequest {
    pub fn new() -> Self {
        Self {
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_config_resources_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListConfigResourcesRequest::new(
        );
        assert_eq!(d, ListConfigResourcesRequest::default());
    }
}

impl Readable for ListConfigResourcesRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListConfigResourcesRequest {
            _unknown_tagged_fields
        })
    }
}

impl Writable for ListConfigResourcesRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
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
        crate::test_utils::test_java_default::<ListConfigResourcesRequest>("ListConfigResourcesRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListConfigResourcesRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListConfigResourcesRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ListConfigResourcesRequest", 0);
        }
    }
}
