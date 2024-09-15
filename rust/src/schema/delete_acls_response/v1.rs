// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each filter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub filter_results: Vec<DeleteAclsFilterResult>,
}

impl ApiMessage for DeleteAclsResponse {
    fn api_key(&self) -> i16 {
        31
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for DeleteAclsResponse { }

impl Default for DeleteAclsResponse {
    fn default() -> Self {
        DeleteAclsResponse {
            throttle_time_ms: 0_i32,
            filter_results: Vec::<DeleteAclsFilterResult>::new(),
        }
    }
}

impl DeleteAclsResponse {
    pub fn new(throttle_time_ms: i32, filter_results: Vec<DeleteAclsFilterResult>) -> Self {
        Self {
            throttle_time_ms,
            filter_results,
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
        let filter_results = read_array::<DeleteAclsFilterResult>(input, "filter_results", false)?;
        Ok(DeleteAclsResponse {
            throttle_time_ms, filter_results
        })
    }
}

impl Writable for DeleteAclsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.filter_results", &self.filter_results, false)?;
        Ok(())
    }
}

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
}

impl ApiMessage for DeleteAclsFilterResult {
    fn api_key(&self) -> i16 {
        31
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for DeleteAclsFilterResult { }

impl Default for DeleteAclsFilterResult {
    fn default() -> Self {
        DeleteAclsFilterResult {
            error_code: 0_i16,
            error_message: Some(String::from("")),
            matching_acls: Vec::<DeleteAclsMatchingAcl>::new(),
        }
    }
}

impl DeleteAclsFilterResult {
    pub fn new<S1: AsRef<str>>(error_code: i16, error_message: Option<S1>, matching_acls: Vec<DeleteAclsMatchingAcl>) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            matching_acls,
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
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        let matching_acls = read_array::<DeleteAclsMatchingAcl>(input, "matching_acls", false)?;
        Ok(DeleteAclsFilterResult {
            error_code, error_message, matching_acls
        })
    }
}

impl Writable for DeleteAclsFilterResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        write_array(output, "self.matching_acls", &self.matching_acls, false)?;
        Ok(())
    }
}

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
}

impl ApiMessage for DeleteAclsMatchingAcl {
    fn api_key(&self) -> i16 {
        31
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for DeleteAclsMatchingAcl { }

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
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", false)?;
        let pattern_type = i8::read(input)?;
        let principal = String::read_ext(input, "principal", false)?;
        let host = String::read_ext(input, "host", false)?;
        let operation = i8::read(input)?;
        let permission_type = i8::read(input)?;
        Ok(DeleteAclsMatchingAcl {
            error_code, error_message, resource_type, resource_name, pattern_type, principal, host, operation, permission_type
        })
    }
}

impl Writable for DeleteAclsMatchingAcl {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", false)?;
        self.pattern_type.write(output)?;
        self.principal.write_ext(output, "self.principal", false)?;
        self.host.write_ext(output, "self.host", false)?;
        self.operation.write(output)?;
        self.permission_type.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DeleteAclsResponse>("DeleteAclsResponse", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "DeleteAclsResponse", 1);
        }
    }
}
