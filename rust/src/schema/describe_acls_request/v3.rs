// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeAclsRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeAclsRequest {
    /// The resource type.
    pub resource_type_filter: i8,
    /// The resource name, or null to match any resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub resource_name_filter: Option<String>,
    /// The resource pattern to match.
    pub pattern_type_filter: i8,
    /// The principal to match, or null to match any principal.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub principal_filter: Option<String>,
    /// The host to match, or null to match any host.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub host_filter: Option<String>,
    /// The operation to match.
    pub operation: i8,
    /// The permission type to match.
    pub permission_type: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeAclsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        29
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Request for DescribeAclsRequest { }

impl Default for DescribeAclsRequest {
    fn default() -> Self {
        DescribeAclsRequest {
            resource_type_filter: 0_i8,
            resource_name_filter: Some(String::from("")),
            pattern_type_filter: 3_i8,
            principal_filter: Some(String::from("")),
            host_filter: Some(String::from("")),
            operation: 0_i8,
            permission_type: 0_i8,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeAclsRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(resource_type_filter: i8, resource_name_filter: Option<S1>, pattern_type_filter: i8, principal_filter: Option<S2>, host_filter: Option<S3>, operation: i8, permission_type: i8) -> Self {
        Self {
            resource_type_filter,
            resource_name_filter: resource_name_filter.map(|s| s.as_ref().to_string()),
            pattern_type_filter,
            principal_filter: principal_filter.map(|s| s.as_ref().to_string()),
            host_filter: host_filter.map(|s| s.as_ref().to_string()),
            operation,
            permission_type,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_acls_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeAclsRequest::new(
            0_i8,
            Some(String::from("")),
            3_i8,
            Some(String::from("")),
            Some(String::from("")),
            0_i8,
            0_i8,
        );
        assert_eq!(d, DescribeAclsRequest::default());
    }
}

impl Readable for DescribeAclsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resource_type_filter = i8::read(input)?;
        let resource_name_filter = Option::<String>::read_ext(input, "resource_name_filter", true)?;
        let pattern_type_filter = i8::read(input)?;
        let principal_filter = Option::<String>::read_ext(input, "principal_filter", true)?;
        let host_filter = Option::<String>::read_ext(input, "host_filter", true)?;
        let operation = i8::read(input)?;
        let permission_type = i8::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeAclsRequest {
            resource_type_filter, resource_name_filter, pattern_type_filter, principal_filter, host_filter, operation, permission_type, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeAclsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.resource_type_filter.write(output)?;
        self.resource_name_filter.write_ext(output, "self.resource_name_filter", true)?;
        self.pattern_type_filter.write(output)?;
        self.principal_filter.write_ext(output, "self.principal_filter", true)?;
        self.host_filter.write_ext(output, "self.host_filter", true)?;
        self.operation.write(output)?;
        self.permission_type.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeAclsRequest>("DescribeAclsRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeAclsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeAclsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeAclsRequest", 3);
        }
    }
}
