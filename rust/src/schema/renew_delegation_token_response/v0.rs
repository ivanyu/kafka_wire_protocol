// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RenewDelegationTokenResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The timestamp in milliseconds at which this token expires.
    pub expiry_timestamp_ms: i64,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

impl ApiMessage for RenewDelegationTokenResponse {
    fn api_key(&self) -> i16 {
        39
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for RenewDelegationTokenResponse { }

impl Default for RenewDelegationTokenResponse {
    fn default() -> Self {
        RenewDelegationTokenResponse {
            error_code: 0_i16,
            expiry_timestamp_ms: 0_i64,
            throttle_time_ms: 0_i32,
        }
    }
}

impl RenewDelegationTokenResponse {
    pub fn new(error_code: i16, expiry_timestamp_ms: i64, throttle_time_ms: i32) -> Self {
        Self {
            error_code,
            expiry_timestamp_ms,
            throttle_time_ms,
        }
    }
}

#[cfg(test)]
mod tests_renew_delegation_token_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = RenewDelegationTokenResponse::new(
            0_i16,
            0_i64,
            0_i32,
        );
        assert_eq!(d, RenewDelegationTokenResponse::default());
    }
}

impl Readable for RenewDelegationTokenResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let expiry_timestamp_ms = i64::read(input)?;
        let throttle_time_ms = i32::read(input)?;
        Ok(RenewDelegationTokenResponse {
            error_code, expiry_timestamp_ms, throttle_time_ms
        })
    }
}

impl Writable for RenewDelegationTokenResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.expiry_timestamp_ms.write(output)?;
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
        crate::test_utils::test_java_default::<RenewDelegationTokenResponse>("RenewDelegationTokenResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: RenewDelegationTokenResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: RenewDelegationTokenResponse) {
            crate::test_utils::test_java_arbitrary(&data, "RenewDelegationTokenResponse", 0);
        }
    }
}
