// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ShareGroupDescribeRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ShareGroupDescribeRequest {
    /// The ids of the groups to describe
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub group_ids: Vec<String>,
    /// Whether to include authorized operations.
    pub include_authorized_operations: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ShareGroupDescribeRequest {
    fn api_key(&self) -> i16 {
        77
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ShareGroupDescribeRequest { }

impl Default for ShareGroupDescribeRequest {
    fn default() -> Self {
        ShareGroupDescribeRequest {
            group_ids: Vec::<String>::new(),
            include_authorized_operations: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ShareGroupDescribeRequest {
    pub fn new(group_ids: Vec<String>, include_authorized_operations: bool) -> Self {
        Self {
            group_ids,
            include_authorized_operations,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_share_group_describe_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ShareGroupDescribeRequest::new(
            Vec::<String>::new(),
            false,
        );
        assert_eq!(d, ShareGroupDescribeRequest::default());
    }
}

impl Readable for ShareGroupDescribeRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_ids = read_array::<String>(input, "group_ids", true)?;
        let include_authorized_operations = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ShareGroupDescribeRequest {
            group_ids, include_authorized_operations, _unknown_tagged_fields
        })
    }
}

impl Writable for ShareGroupDescribeRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.group_ids", &self.group_ids, true)?;
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
        crate::test_utils::test_java_default::<ShareGroupDescribeRequest>("ShareGroupDescribeRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ShareGroupDescribeRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ShareGroupDescribeRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ShareGroupDescribeRequest", 0);
        }
    }
}
