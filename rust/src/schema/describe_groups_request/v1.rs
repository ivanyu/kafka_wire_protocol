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
pub struct DescribeGroupsRequest {
    /// The names of the groups to describe
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<String>,
}

impl ApiMessage for DescribeGroupsRequest {
    fn api_key(&self) -> i16 {
        15
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for DescribeGroupsRequest { }

impl Default for DescribeGroupsRequest {
    fn default() -> Self {
        DescribeGroupsRequest {
            groups: Vec::<String>::new(),
        }
    }
}

impl DescribeGroupsRequest {
    pub fn new(groups: Vec<String>) -> Self {
        Self {
            groups,
        }
    }
}

#[cfg(test)]
mod tests_describe_groups_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeGroupsRequest::new(
            Vec::<String>::new(),
        );
        assert_eq!(d, DescribeGroupsRequest::default());
    }
}

impl Readable for DescribeGroupsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let groups = read_array::<String>(input, "groups", false)?;
        Ok(DescribeGroupsRequest {
            groups
        })
    }
}

impl Writable for DescribeGroupsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.groups", &self.groups, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DescribeGroupsRequest>("DescribeGroupsRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeGroupsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeGroupsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeGroupsRequest", 1);
        }
    }
}
