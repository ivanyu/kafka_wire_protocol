// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};

/// ListGroupsRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListGroupsRequest {
}

impl ApiMessage for ListGroupsRequest {
    fn api_key(&self) -> i16 {
        16
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for ListGroupsRequest { }

impl Default for ListGroupsRequest {
    fn default() -> Self {
        ListGroupsRequest {
        }
    }
}

impl ListGroupsRequest {
    pub fn new() -> Self {
        Self {
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
        Ok(ListGroupsRequest {
            
        })
    }
}

impl Writable for ListGroupsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ListGroupsRequest>("ListGroupsRequest", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "ListGroupsRequest", 2);
        }
    }
}
