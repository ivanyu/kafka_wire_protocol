// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// CreateAclsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each ACL creation.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<AclCreationResult>,
}

impl ApiMessage for CreateAclsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        30
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for CreateAclsResponse { }

impl Default for CreateAclsResponse {
    fn default() -> Self {
        CreateAclsResponse {
            throttle_time_ms: 0_i32,
            results: Vec::<AclCreationResult>::new(),
        }
    }
}

impl CreateAclsResponse {
    pub fn new(throttle_time_ms: i32, results: Vec<AclCreationResult>) -> Self {
        Self {
            throttle_time_ms,
            results,
        }
    }
}

#[cfg(test)]
mod tests_create_acls_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateAclsResponse::new(
            0_i32,
            Vec::<AclCreationResult>::new(),
        );
        assert_eq!(d, CreateAclsResponse::default());
    }
}

impl Readable for CreateAclsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let results = read_array::<AclCreationResult>(input, "results", false)?;
        Ok(CreateAclsResponse {
            throttle_time_ms, results
        })
    }
}

impl Writable for CreateAclsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.results", &self.results, false)?;
        Ok(())
    }
}

/// AclCreationResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AclCreationResult {
    /// The result error, or zero if there was no error.
    pub error_code: i16,
    /// The result message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
}

impl Default for AclCreationResult {
    fn default() -> Self {
        AclCreationResult {
            error_code: 0_i16,
            error_message: Some(String::from("")),
        }
    }
}

impl AclCreationResult {
    pub fn new<S1: AsRef<str>>(error_code: i16, error_message: Option<S1>) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
        }
    }
}

#[cfg(test)]
mod tests_acl_creation_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AclCreationResult::new(
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, AclCreationResult::default());
    }
}

impl Readable for AclCreationResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        Ok(AclCreationResult {
            error_code, error_message
        })
    }
}

impl Writable for AclCreationResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<CreateAclsResponse>("CreateAclsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreateAclsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreateAclsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "CreateAclsResponse", 0);
        }
    }
}
