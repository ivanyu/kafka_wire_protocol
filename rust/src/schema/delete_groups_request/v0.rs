// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteGroupsRequest {
    /// The group names to delete.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups_names: Vec<String>,
}

impl ApiMessage for DeleteGroupsRequest {
    fn api_key(&self) -> i16 {
        42
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DeleteGroupsRequest { }

impl Default for DeleteGroupsRequest {
    fn default() -> Self {
        DeleteGroupsRequest {
            groups_names: Vec::<String>::new(),
        }
    }
}

impl DeleteGroupsRequest {
    pub fn new(groups_names: Vec<String>) -> Self {
        Self {
            groups_names,
        }
    }
}

#[cfg(test)]
mod tests_delete_groups_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteGroupsRequest::new(
            Vec::<String>::new(),
        );
        assert_eq!(d, DeleteGroupsRequest::default());
    }
}

impl Readable for DeleteGroupsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let groups_names = read_array::<String>(input, "groups_names", false)?;
        Ok(DeleteGroupsRequest {
            groups_names
        })
    }
}

impl Writable for DeleteGroupsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.groups_names", &self.groups_names, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DeleteGroupsRequest>("DeleteGroupsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteGroupsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteGroupsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteGroupsRequest", 0);
        }
    }
}
