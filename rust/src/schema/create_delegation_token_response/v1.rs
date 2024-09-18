// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// CreateDelegationTokenResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateDelegationTokenResponse {
    /// The top-level error, or zero if there was no error.
    pub error_code: i16,
    /// The principal type of the token owner.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_type: String,
    /// The name of the token owner.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_name: String,
    /// When this token was generated.
    pub issue_timestamp_ms: i64,
    /// When this token expires.
    pub expiry_timestamp_ms: i64,
    /// The maximum lifetime of this token.
    pub max_timestamp_ms: i64,
    /// The token UUID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub token_id: String,
    /// HMAC of the delegation token.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub hmac: Vec<u8>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

impl ApiMessage for CreateDelegationTokenResponse {
    fn api_key(&self) -> i16 {
        38
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for CreateDelegationTokenResponse { }

impl Default for CreateDelegationTokenResponse {
    fn default() -> Self {
        CreateDelegationTokenResponse {
            error_code: 0_i16,
            principal_type: String::from(""),
            principal_name: String::from(""),
            issue_timestamp_ms: 0_i64,
            expiry_timestamp_ms: 0_i64,
            max_timestamp_ms: 0_i64,
            token_id: String::from(""),
            hmac: Vec::new(),
            throttle_time_ms: 0_i32,
        }
    }
}

impl CreateDelegationTokenResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(error_code: i16, principal_type: S1, principal_name: S2, issue_timestamp_ms: i64, expiry_timestamp_ms: i64, max_timestamp_ms: i64, token_id: S3, hmac: Vec<u8>, throttle_time_ms: i32) -> Self {
        Self {
            error_code,
            principal_type: principal_type.as_ref().to_string(),
            principal_name: principal_name.as_ref().to_string(),
            issue_timestamp_ms,
            expiry_timestamp_ms,
            max_timestamp_ms,
            token_id: token_id.as_ref().to_string(),
            hmac,
            throttle_time_ms,
        }
    }
}

#[cfg(test)]
mod tests_create_delegation_token_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateDelegationTokenResponse::new(
            0_i16,
            String::from(""),
            String::from(""),
            0_i64,
            0_i64,
            0_i64,
            String::from(""),
            Vec::new(),
            0_i32,
        );
        assert_eq!(d, CreateDelegationTokenResponse::default());
    }
}

impl Readable for CreateDelegationTokenResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let principal_type = String::read_ext(input, "principal_type", false)?;
        let principal_name = String::read_ext(input, "principal_name", false)?;
        let issue_timestamp_ms = i64::read(input)?;
        let expiry_timestamp_ms = i64::read(input)?;
        let max_timestamp_ms = i64::read(input)?;
        let token_id = String::read_ext(input, "token_id", false)?;
        let hmac = read_bytes(input, "hmac", false)?;
        let throttle_time_ms = i32::read(input)?;
        Ok(CreateDelegationTokenResponse {
            error_code, principal_type, principal_name, issue_timestamp_ms, expiry_timestamp_ms, max_timestamp_ms, token_id, hmac, throttle_time_ms
        })
    }
}

impl Writable for CreateDelegationTokenResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.principal_type.write_ext(output, "self.principal_type", false)?;
        self.principal_name.write_ext(output, "self.principal_name", false)?;
        self.issue_timestamp_ms.write(output)?;
        self.expiry_timestamp_ms.write(output)?;
        self.max_timestamp_ms.write(output)?;
        self.token_id.write_ext(output, "self.token_id", false)?;
        write_bytes(output, "self.hmac", &self.hmac, false)?;
        self.throttle_time_ms.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<CreateDelegationTokenResponse>("CreateDelegationTokenResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreateDelegationTokenResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreateDelegationTokenResponse) {
            crate::test_utils::test_java_arbitrary(&data, "CreateDelegationTokenResponse", 1);
        }
    }
}
