// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListGroupsRequest, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListGroupsRequest {
    /// The states of the groups we want to list. If empty, all groups are returned with their state.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub states_filter: Vec<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListGroupsRequest {
    fn api_key(&self) -> i16 {
        16
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for ListGroupsRequest { }

impl Default for ListGroupsRequest {
    fn default() -> Self {
        ListGroupsRequest {
            states_filter: Vec::<String>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListGroupsRequest {
    pub fn new(states_filter: Vec<String>) -> Self {
        Self {
            states_filter,
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
            Vec::<String>::new(),
        );
        assert_eq!(d, ListGroupsRequest::default());
    }
}

impl Readable for ListGroupsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let states_filter = read_array::<String>(input, "states_filter", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListGroupsRequest {
            states_filter, _unknown_tagged_fields
        })
    }
}

impl Writable for ListGroupsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.states_filter", &self.states_filter, true)?;
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
        crate::test_utils::test_java_default::<ListGroupsRequest>("ListGroupsRequest", 4);
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
            crate::test_utils::test_java_arbitrary(&data, "ListGroupsRequest", 4);
        }
    }
}
