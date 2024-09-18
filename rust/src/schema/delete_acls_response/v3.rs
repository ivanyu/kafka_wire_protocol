// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteAclsResponse, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each filter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub filter_results: Vec<DeleteAclsFilterResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteAclsResponse {
    fn api_key(&self) -> i16 {
        31
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Response for DeleteAclsResponse { }

impl Default for DeleteAclsResponse {
    fn default() -> Self {
        DeleteAclsResponse {
            throttle_time_ms: 0_i32,
            filter_results: Vec::<DeleteAclsFilterResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteAclsResponse {
    pub fn new(throttle_time_ms: i32, filter_results: Vec<DeleteAclsFilterResult>) -> Self {
        Self {
            throttle_time_ms,
            filter_results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_acls_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteAclsResponse::new(
            0_i32,
            Vec::<DeleteAclsFilterResult>::new(),
        );
        assert_eq!(d, DeleteAclsResponse::default());
    }
}

impl Readable for DeleteAclsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let filter_results = read_array::<DeleteAclsFilterResult>(input, "filter_results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteAclsResponse {
            throttle_time_ms, filter_results, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteAclsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.filter_results", &self.filter_results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteAclsFilterResult, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteAclsFilterResult {
    /// The error code, or 0 if the filter succeeded.
    pub error_code: i16,
    /// The error message, or null if the filter succeeded.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The ACLs which matched this filter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub matching_acls: Vec<DeleteAclsMatchingAcl>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteAclsFilterResult {
    fn default() -> Self {
        DeleteAclsFilterResult {
            error_code: 0_i16,
            error_message: Some(String::from("")),
            matching_acls: Vec::<DeleteAclsMatchingAcl>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteAclsFilterResult {
    pub fn new<S1: AsRef<str>>(error_code: i16, error_message: Option<S1>, matching_acls: Vec<DeleteAclsMatchingAcl>) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            matching_acls,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_acls_filter_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteAclsFilterResult::new(
            0_i16,
            Some(String::from("")),
            Vec::<DeleteAclsMatchingAcl>::new(),
        );
        assert_eq!(d, DeleteAclsFilterResult::default());
    }
}

impl Readable for DeleteAclsFilterResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let matching_acls = read_array::<DeleteAclsMatchingAcl>(input, "matching_acls", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteAclsFilterResult {
            error_code, error_message, matching_acls, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteAclsFilterResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.matching_acls", &self.matching_acls, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteAclsMatchingAcl, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteAclsMatchingAcl {
    /// The deletion error code, or 0 if the deletion succeeded.
    pub error_code: i16,
    /// The deletion error message, or null if the deletion succeeded.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The ACL resource type.
    pub resource_type: i8,
    /// The ACL resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
    /// The ACL resource pattern type.
    pub pattern_type: i8,
    /// The ACL principal.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal: String,
    /// The ACL host.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The ACL operation.
    pub operation: i8,
    /// The ACL permission type.
    pub permission_type: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteAclsMatchingAcl {
    fn default() -> Self {
        DeleteAclsMatchingAcl {
            error_code: 0_i16,
            error_message: Some(String::from("")),
            resource_type: 0_i8,
            resource_name: String::from(""),
            pattern_type: 3_i8,
            principal: String::from(""),
            host: String::from(""),
            operation: 0_i8,
            permission_type: 0_i8,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteAclsMatchingAcl {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>>(error_code: i16, error_message: Option<S1>, resource_type: i8, resource_name: S2, pattern_type: i8, principal: S3, host: S4, operation: i8, permission_type: i8) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
            pattern_type,
            principal: principal.as_ref().to_string(),
            host: host.as_ref().to_string(),
            operation,
            permission_type,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_acls_matching_acl_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteAclsMatchingAcl::new(
            0_i16,
            Some(String::from("")),
            0_i8,
            String::from(""),
            3_i8,
            String::from(""),
            String::from(""),
            0_i8,
            0_i8,
        );
        assert_eq!(d, DeleteAclsMatchingAcl::default());
    }
}

impl Readable for DeleteAclsMatchingAcl {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", true)?;
        let pattern_type = i8::read(input)?;
        let principal = String::read_ext(input, "principal", true)?;
        let host = String::read_ext(input, "host", true)?;
        let operation = i8::read(input)?;
        let permission_type = i8::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteAclsMatchingAcl {
            error_code, error_message, resource_type, resource_name, pattern_type, principal, host, operation, permission_type, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteAclsMatchingAcl {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", true)?;
        self.pattern_type.write(output)?;
        self.principal.write_ext(output, "self.principal", true)?;
        self.host.write_ext(output, "self.host", true)?;
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
        crate::test_utils::test_java_default::<DeleteAclsResponse>("DeleteAclsResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteAclsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteAclsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteAclsResponse", 3);
        }
    }
}
