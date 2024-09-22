// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// CreateDelegationTokenResponse, version 2.
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
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreateDelegationTokenResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        38
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        2
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
            _unknown_tagged_fields: Vec::new(),
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
            _unknown_tagged_fields: vec![],
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
        let principal_type = String::read_ext(input, "principal_type", true)?;
        let principal_name = String::read_ext(input, "principal_name", true)?;
        let issue_timestamp_ms = i64::read(input)?;
        let expiry_timestamp_ms = i64::read(input)?;
        let max_timestamp_ms = i64::read(input)?;
        let token_id = String::read_ext(input, "token_id", true)?;
        let hmac = read_bytes(input, "hmac", true)?;
        let throttle_time_ms = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreateDelegationTokenResponse {
            error_code, principal_type, principal_name, issue_timestamp_ms, expiry_timestamp_ms, max_timestamp_ms, token_id, hmac, throttle_time_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for CreateDelegationTokenResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.principal_type.write_ext(output, "self.principal_type", true)?;
        self.principal_name.write_ext(output, "self.principal_name", true)?;
        self.issue_timestamp_ms.write(output)?;
        self.expiry_timestamp_ms.write(output)?;
        self.max_timestamp_ms.write(output)?;
        self.token_id.write_ext(output, "self.token_id", true)?;
        write_bytes(output, "self.hmac", &self.hmac, true)?;
        self.throttle_time_ms.write(output)?;
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
        crate::test_utils::test_java_default::<CreateDelegationTokenResponse>("CreateDelegationTokenResponse", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "CreateDelegationTokenResponse", 2);
        }
    }
}
