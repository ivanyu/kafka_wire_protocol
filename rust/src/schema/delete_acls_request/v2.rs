// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteAclsRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteAclsRequest {
    /// The filters to use when deleting ACLs.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub filters: Vec<DeleteAclsFilter>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteAclsRequest {
    fn api_key(&self) -> i16 {
        31
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for DeleteAclsRequest { }

impl Default for DeleteAclsRequest {
    fn default() -> Self {
        DeleteAclsRequest {
            filters: Vec::<DeleteAclsFilter>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteAclsRequest {
    pub fn new(filters: Vec<DeleteAclsFilter>) -> Self {
        Self {
            filters,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_acls_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteAclsRequest::new(
            Vec::<DeleteAclsFilter>::new(),
        );
        assert_eq!(d, DeleteAclsRequest::default());
    }
}

impl Readable for DeleteAclsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let filters = read_array::<DeleteAclsFilter>(input, "filters", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteAclsRequest {
            filters, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteAclsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.filters", &self.filters, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteAclsFilter, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteAclsFilter {
    /// The resource type.
    pub resource_type_filter: i8,
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub resource_name_filter: Option<String>,
    /// The pattern type.
    pub pattern_type_filter: i8,
    /// The principal filter, or null to accept all principals.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub principal_filter: Option<String>,
    /// The host filter, or null to accept all hosts.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub host_filter: Option<String>,
    /// The ACL operation.
    pub operation: i8,
    /// The permission type.
    pub permission_type: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteAclsFilter {
    fn default() -> Self {
        DeleteAclsFilter {
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

impl DeleteAclsFilter {
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
mod tests_delete_acls_filter_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteAclsFilter::new(
            0_i8,
            Some(String::from("")),
            3_i8,
            Some(String::from("")),
            Some(String::from("")),
            0_i8,
            0_i8,
        );
        assert_eq!(d, DeleteAclsFilter::default());
    }
}

impl Readable for DeleteAclsFilter {
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
        Ok(DeleteAclsFilter {
            resource_type_filter, resource_name_filter, pattern_type_filter, principal_filter, host_filter, operation, permission_type, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteAclsFilter {
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
        crate::test_utils::test_java_default::<DeleteAclsRequest>("DeleteAclsRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteAclsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteAclsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteAclsRequest", 2);
        }
    }
}
