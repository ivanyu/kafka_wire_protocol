// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListGroupsRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListGroupsRequest {
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListGroupsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        16
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Request for ListGroupsRequest { }

impl Default for ListGroupsRequest {
    fn default() -> Self {
        ListGroupsRequest {
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListGroupsRequest {
    pub fn new() -> Self {
        Self {
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_groups_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListGroupsRequest::new(
        );
        assert_eq!(d, ListGroupsRequest::default());
    }
}

impl Readable for ListGroupsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListGroupsRequest {
            _unknown_tagged_fields
        })
    }
}

impl Writable for ListGroupsRequest {
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
        crate::test_utils::test_java_default::<ListGroupsRequest>("ListGroupsRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListGroupsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListGroupsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ListGroupsRequest", 3);
        }
    }
}
