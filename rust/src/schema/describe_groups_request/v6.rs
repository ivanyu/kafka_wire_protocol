// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeGroupsRequest, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeGroupsRequest {
    /// The names of the groups to describe.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<String>,
    /// Whether to include authorized operations.
    pub include_authorized_operations: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeGroupsRequest {
    fn api_key(&self) -> i16 {
        15
    }
    
    fn version(&self) -> i16 {
        6
    }
}

impl Request for DescribeGroupsRequest { }

impl Default for DescribeGroupsRequest {
    fn default() -> Self {
        DescribeGroupsRequest {
            groups: Vec::<String>::new(),
            include_authorized_operations: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeGroupsRequest {
    pub fn new(groups: Vec<String>, include_authorized_operations: bool) -> Self {
        Self {
            groups,
            include_authorized_operations,
            _unknown_tagged_fields: vec![],
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
            false,
        );
        assert_eq!(d, DescribeGroupsRequest::default());
    }
}

impl Readable for DescribeGroupsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let groups = read_array::<String>(input, "groups", true)?;
        let include_authorized_operations = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeGroupsRequest {
            groups, include_authorized_operations, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeGroupsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.groups", &self.groups, true)?;
        self.include_authorized_operations.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeGroupsRequest>("DescribeGroupsRequest", 6);
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
            crate::test_utils::test_java_arbitrary(&data, "DescribeGroupsRequest", 6);
        }
    }
}
