// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeClusterRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeClusterRequest {
    /// Whether to include cluster authorized operations.
    pub include_cluster_authorized_operations: bool,
    /// The endpoint type to describe. 1=brokers, 2=controllers.
    pub endpoint_type: i8,
    /// Whether to include fenced brokers when listing brokers.
    pub include_fenced_brokers: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeClusterRequest {
    fn api_key(&self) -> i16 {
        60
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for DescribeClusterRequest { }

impl Default for DescribeClusterRequest {
    fn default() -> Self {
        DescribeClusterRequest {
            include_cluster_authorized_operations: false,
            endpoint_type: 1_i8,
            include_fenced_brokers: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClusterRequest {
    pub fn new(include_cluster_authorized_operations: bool, endpoint_type: i8, include_fenced_brokers: bool) -> Self {
        Self {
            include_cluster_authorized_operations,
            endpoint_type,
            include_fenced_brokers,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_cluster_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeClusterRequest::new(
            false,
            1_i8,
            false,
        );
        assert_eq!(d, DescribeClusterRequest::default());
    }
}

impl Readable for DescribeClusterRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let include_cluster_authorized_operations = bool::read(input)?;
        let endpoint_type = i8::read(input)?;
        let include_fenced_brokers = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeClusterRequest {
            include_cluster_authorized_operations, endpoint_type, include_fenced_brokers, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeClusterRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.include_cluster_authorized_operations.write(output)?;
        self.endpoint_type.write(output)?;
        self.include_fenced_brokers.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeClusterRequest>("DescribeClusterRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeClusterRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeClusterRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeClusterRequest", 2);
        }
    }
}
