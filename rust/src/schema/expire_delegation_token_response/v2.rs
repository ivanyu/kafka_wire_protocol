// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ExpireDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The timestamp in milliseconds at which this token expires.
    pub expiry_timestamp_ms: i64,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ExpireDelegationTokenResponse {
    fn api_key(&self) -> i16 {
        40
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for ExpireDelegationTokenResponse { }

impl Default for ExpireDelegationTokenResponse {
    fn default() -> Self {
        ExpireDelegationTokenResponse {
            error_code: 0_i16,
            expiry_timestamp_ms: 0_i64,
            throttle_time_ms: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ExpireDelegationTokenResponse {
    pub fn new(error_code: i16, expiry_timestamp_ms: i64, throttle_time_ms: i32) -> Self {
        Self {
            error_code,
            expiry_timestamp_ms,
            throttle_time_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_expire_delegation_token_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ExpireDelegationTokenResponse::new(
            0_i16,
            0_i64,
            0_i32,
        );
        assert_eq!(d, ExpireDelegationTokenResponse::default());
    }
}

impl Readable for ExpireDelegationTokenResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let expiry_timestamp_ms = i64::read(input)?;
        let throttle_time_ms = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ExpireDelegationTokenResponse {
            error_code, expiry_timestamp_ms, throttle_time_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for ExpireDelegationTokenResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.expiry_timestamp_ms.write(output)?;
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
        crate::test_utils::test_java_default::<ExpireDelegationTokenResponse>("ExpireDelegationTokenResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ExpireDelegationTokenResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ExpireDelegationTokenResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ExpireDelegationTokenResponse", 2);
        }
    }
}
