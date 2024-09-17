// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// RenewDelegationTokenRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RenewDelegationTokenRequest {
    /// The HMAC of the delegation token to be renewed.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub hmac: Vec<u8>,
    /// The renewal time period in milliseconds.
    pub renew_period_ms: i64,
}

impl ApiMessage for RenewDelegationTokenRequest {
    fn api_key(&self) -> i16 {
        39
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for RenewDelegationTokenRequest { }

impl Default for RenewDelegationTokenRequest {
    fn default() -> Self {
        RenewDelegationTokenRequest {
            hmac: Vec::new(),
            renew_period_ms: 0_i64,
        }
    }
}

impl RenewDelegationTokenRequest {
    pub fn new(hmac: Vec<u8>, renew_period_ms: i64) -> Self {
        Self {
            hmac,
            renew_period_ms,
        }
    }
}

#[cfg(test)]
mod tests_renew_delegation_token_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = RenewDelegationTokenRequest::new(
            Vec::new(),
            0_i64,
        );
        assert_eq!(d, RenewDelegationTokenRequest::default());
    }
}

impl Readable for RenewDelegationTokenRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let hmac = read_bytes(input, "hmac", false)?;
        let renew_period_ms = i64::read(input)?;
        Ok(RenewDelegationTokenRequest {
            hmac, renew_period_ms
        })
    }
}

impl Writable for RenewDelegationTokenRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_bytes(output, "self.hmac", &self.hmac, false)?;
        self.renew_period_ms.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<RenewDelegationTokenRequest>("RenewDelegationTokenRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: RenewDelegationTokenRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: RenewDelegationTokenRequest) {
            crate::test_utils::test_java_arbitrary(&data, "RenewDelegationTokenRequest", 1);
        }
    }
}
