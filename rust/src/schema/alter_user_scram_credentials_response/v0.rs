// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AlterUserScramCredentialsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterUserScramCredentialsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for deletions and alterations, one per affected user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<AlterUserScramCredentialsResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterUserScramCredentialsResponse {
    fn api_key(&self) -> i16 {
        51
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for AlterUserScramCredentialsResponse { }

impl Default for AlterUserScramCredentialsResponse {
    fn default() -> Self {
        AlterUserScramCredentialsResponse {
            throttle_time_ms: 0_i32,
            results: Vec::<AlterUserScramCredentialsResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterUserScramCredentialsResponse {
    pub fn new(throttle_time_ms: i32, results: Vec<AlterUserScramCredentialsResult>) -> Self {
        Self {
            throttle_time_ms,
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_user_scram_credentials_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterUserScramCredentialsResponse::new(
            0_i32,
            Vec::<AlterUserScramCredentialsResult>::new(),
        );
        assert_eq!(d, AlterUserScramCredentialsResponse::default());
    }
}

impl Readable for AlterUserScramCredentialsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let results = read_array::<AlterUserScramCredentialsResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterUserScramCredentialsResponse {
            throttle_time_ms, results, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterUserScramCredentialsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AlterUserScramCredentialsResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterUserScramCredentialsResult {
    /// The user name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub user: String,
    /// The error code.
    pub error_code: i16,
    /// The error message, if any.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AlterUserScramCredentialsResult {
    fn default() -> Self {
        AlterUserScramCredentialsResult {
            user: String::from(""),
            error_code: 0_i16,
            error_message: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterUserScramCredentialsResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(user: S1, error_code: i16, error_message: Option<S2>) -> Self {
        Self {
            user: user.as_ref().to_string(),
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_user_scram_credentials_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterUserScramCredentialsResult::new(
            String::from(""),
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, AlterUserScramCredentialsResult::default());
    }
}

impl Readable for AlterUserScramCredentialsResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let user = String::read_ext(input, "user", true)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterUserScramCredentialsResult {
            user, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterUserScramCredentialsResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.user.write_ext(output, "self.user", true)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
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
        crate::test_utils::test_java_default::<AlterUserScramCredentialsResponse>("AlterUserScramCredentialsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterUserScramCredentialsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterUserScramCredentialsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AlterUserScramCredentialsResponse", 0);
        }
    }
}
