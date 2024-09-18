// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeUserScramCredentialsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeUserScramCredentialsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The message-level error code, 0 except for user authorization or infrastructure issues.
    pub error_code: i16,
    /// The message-level error message, if any.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The results for descriptions, one per user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<DescribeUserScramCredentialsResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeUserScramCredentialsResponse {
    fn api_key(&self) -> i16 {
        50
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DescribeUserScramCredentialsResponse { }

impl Default for DescribeUserScramCredentialsResponse {
    fn default() -> Self {
        DescribeUserScramCredentialsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            results: Vec::<DescribeUserScramCredentialsResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeUserScramCredentialsResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, results: Vec<DescribeUserScramCredentialsResult>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_user_scram_credentials_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeUserScramCredentialsResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
            Vec::<DescribeUserScramCredentialsResult>::new(),
        );
        assert_eq!(d, DescribeUserScramCredentialsResponse::default());
    }
}

impl Readable for DescribeUserScramCredentialsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let results = read_array::<DescribeUserScramCredentialsResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeUserScramCredentialsResponse {
            throttle_time_ms, error_code, error_message, results, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeUserScramCredentialsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeUserScramCredentialsResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeUserScramCredentialsResult {
    /// The user name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub user: String,
    /// The user-level error code.
    pub error_code: i16,
    /// The user-level error message, if any.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The mechanism and related information associated with the user's SCRAM credentials.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub credential_infos: Vec<CredentialInfo>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeUserScramCredentialsResult {
    fn default() -> Self {
        DescribeUserScramCredentialsResult {
            user: String::from(""),
            error_code: 0_i16,
            error_message: Some(String::from("")),
            credential_infos: Vec::<CredentialInfo>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeUserScramCredentialsResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(user: S1, error_code: i16, error_message: Option<S2>, credential_infos: Vec<CredentialInfo>) -> Self {
        Self {
            user: user.as_ref().to_string(),
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            credential_infos,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_user_scram_credentials_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeUserScramCredentialsResult::new(
            String::from(""),
            0_i16,
            Some(String::from("")),
            Vec::<CredentialInfo>::new(),
        );
        assert_eq!(d, DescribeUserScramCredentialsResult::default());
    }
}

impl Readable for DescribeUserScramCredentialsResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let user = String::read_ext(input, "user", true)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let credential_infos = read_array::<CredentialInfo>(input, "credential_infos", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeUserScramCredentialsResult {
            user, error_code, error_message, credential_infos, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeUserScramCredentialsResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.user.write_ext(output, "self.user", true)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.credential_infos", &self.credential_infos, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// CredentialInfo, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CredentialInfo {
    /// The SCRAM mechanism.
    pub mechanism: i8,
    /// The number of iterations used in the SCRAM credential.
    pub iterations: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for CredentialInfo {
    fn default() -> Self {
        CredentialInfo {
            mechanism: 0_i8,
            iterations: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CredentialInfo {
    pub fn new(mechanism: i8, iterations: i32) -> Self {
        Self {
            mechanism,
            iterations,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_credential_info_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CredentialInfo::new(
            0_i8,
            0_i32,
        );
        assert_eq!(d, CredentialInfo::default());
    }
}

impl Readable for CredentialInfo {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let mechanism = i8::read(input)?;
        let iterations = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CredentialInfo {
            mechanism, iterations, _unknown_tagged_fields
        })
    }
}

impl Writable for CredentialInfo {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.mechanism.write(output)?;
        self.iterations.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeUserScramCredentialsResponse>("DescribeUserScramCredentialsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeUserScramCredentialsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeUserScramCredentialsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeUserScramCredentialsResponse", 0);
        }
    }
}
