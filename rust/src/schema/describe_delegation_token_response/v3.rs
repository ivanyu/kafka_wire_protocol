// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// DescribeDelegationTokenResponse, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The tokens.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub tokens: Vec<DescribedDelegationToken>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeDelegationTokenResponse {
    fn api_key(&self) -> i16 {
        41
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Response for DescribeDelegationTokenResponse { }

impl Default for DescribeDelegationTokenResponse {
    fn default() -> Self {
        DescribeDelegationTokenResponse {
            error_code: 0_i16,
            tokens: Vec::<DescribedDelegationToken>::new(),
            throttle_time_ms: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeDelegationTokenResponse {
    pub fn new(error_code: i16, tokens: Vec<DescribedDelegationToken>, throttle_time_ms: i32) -> Self {
        Self {
            error_code,
            tokens,
            throttle_time_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_delegation_token_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeDelegationTokenResponse::new(
            0_i16,
            Vec::<DescribedDelegationToken>::new(),
            0_i32,
        );
        assert_eq!(d, DescribeDelegationTokenResponse::default());
    }
}

impl Readable for DescribeDelegationTokenResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let tokens = read_array::<DescribedDelegationToken>(input, "tokens", true)?;
        let throttle_time_ms = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeDelegationTokenResponse {
            error_code, tokens, throttle_time_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeDelegationTokenResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.tokens", &self.tokens, true)?;
        self.throttle_time_ms.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribedDelegationToken, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribedDelegationToken {
    /// The token principal type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_type: String,
    /// The token principal name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_name: String,
    /// The principal type of the requester of the token.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub token_requester_principal_type: String,
    /// The principal type of the requester of the token.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub token_requester_principal_name: String,
    /// The token issue timestamp in milliseconds.
    pub issue_timestamp: i64,
    /// The token expiry timestamp in milliseconds.
    pub expiry_timestamp: i64,
    /// The token maximum timestamp length in milliseconds.
    pub max_timestamp: i64,
    /// The token ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub token_id: String,
    /// The token HMAC.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub hmac: Vec<u8>,
    /// Those who are able to renew this token before it expires.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub renewers: Vec<DescribedDelegationTokenRenewer>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribedDelegationToken {
    fn default() -> Self {
        DescribedDelegationToken {
            principal_type: String::from(""),
            principal_name: String::from(""),
            token_requester_principal_type: String::from(""),
            token_requester_principal_name: String::from(""),
            issue_timestamp: 0_i64,
            expiry_timestamp: 0_i64,
            max_timestamp: 0_i64,
            token_id: String::from(""),
            hmac: Vec::new(),
            renewers: Vec::<DescribedDelegationTokenRenewer>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribedDelegationToken {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>, S5: AsRef<str>>(principal_type: S1, principal_name: S2, token_requester_principal_type: S3, token_requester_principal_name: S4, issue_timestamp: i64, expiry_timestamp: i64, max_timestamp: i64, token_id: S5, hmac: Vec<u8>, renewers: Vec<DescribedDelegationTokenRenewer>) -> Self {
        Self {
            principal_type: principal_type.as_ref().to_string(),
            principal_name: principal_name.as_ref().to_string(),
            token_requester_principal_type: token_requester_principal_type.as_ref().to_string(),
            token_requester_principal_name: token_requester_principal_name.as_ref().to_string(),
            issue_timestamp,
            expiry_timestamp,
            max_timestamp,
            token_id: token_id.as_ref().to_string(),
            hmac,
            renewers,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_described_delegation_token_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribedDelegationToken::new(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            0_i64,
            0_i64,
            0_i64,
            String::from(""),
            Vec::new(),
            Vec::<DescribedDelegationTokenRenewer>::new(),
        );
        assert_eq!(d, DescribedDelegationToken::default());
    }
}

impl Readable for DescribedDelegationToken {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let principal_type = String::read_ext(input, "principal_type", true)?;
        let principal_name = String::read_ext(input, "principal_name", true)?;
        let token_requester_principal_type = String::read_ext(input, "token_requester_principal_type", true)?;
        let token_requester_principal_name = String::read_ext(input, "token_requester_principal_name", true)?;
        let issue_timestamp = i64::read(input)?;
        let expiry_timestamp = i64::read(input)?;
        let max_timestamp = i64::read(input)?;
        let token_id = String::read_ext(input, "token_id", true)?;
        let hmac = read_bytes(input, "hmac", true)?;
        let renewers = read_array::<DescribedDelegationTokenRenewer>(input, "renewers", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribedDelegationToken {
            principal_type, principal_name, token_requester_principal_type, token_requester_principal_name, issue_timestamp, expiry_timestamp, max_timestamp, token_id, hmac, renewers, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribedDelegationToken {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.principal_type.write_ext(output, "self.principal_type", true)?;
        self.principal_name.write_ext(output, "self.principal_name", true)?;
        self.token_requester_principal_type.write_ext(output, "self.token_requester_principal_type", true)?;
        self.token_requester_principal_name.write_ext(output, "self.token_requester_principal_name", true)?;
        self.issue_timestamp.write(output)?;
        self.expiry_timestamp.write(output)?;
        self.max_timestamp.write(output)?;
        self.token_id.write_ext(output, "self.token_id", true)?;
        write_bytes(output, "self.hmac", &self.hmac, true)?;
        write_array(output, "self.renewers", &self.renewers, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribedDelegationTokenRenewer, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribedDelegationTokenRenewer {
    /// The renewer principal type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_type: String,
    /// The renewer principal name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_name: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribedDelegationTokenRenewer {
    fn default() -> Self {
        DescribedDelegationTokenRenewer {
            principal_type: String::from(""),
            principal_name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribedDelegationTokenRenewer {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(principal_type: S1, principal_name: S2) -> Self {
        Self {
            principal_type: principal_type.as_ref().to_string(),
            principal_name: principal_name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_described_delegation_token_renewer_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribedDelegationTokenRenewer::new(
            String::from(""),
            String::from(""),
        );
        assert_eq!(d, DescribedDelegationTokenRenewer::default());
    }
}

impl Readable for DescribedDelegationTokenRenewer {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let principal_type = String::read_ext(input, "principal_type", true)?;
        let principal_name = String::read_ext(input, "principal_name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribedDelegationTokenRenewer {
            principal_type, principal_name, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribedDelegationTokenRenewer {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.principal_type.write_ext(output, "self.principal_type", true)?;
        self.principal_name.write_ext(output, "self.principal_name", true)?;
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
        crate::test_utils::test_java_default::<DescribeDelegationTokenResponse>("DescribeDelegationTokenResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeDelegationTokenResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeDelegationTokenResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeDelegationTokenResponse", 3);
        }
    }
}
