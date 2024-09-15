// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ExpireDelegationTokenRequest {
    /// The HMAC of the delegation token to be expired.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub hmac: Vec<u8>,
    /// The expiry time period in milliseconds.
    pub expiry_time_period_ms: i64,
}

impl ApiMessage for ExpireDelegationTokenRequest {
    fn api_key(&self) -> i16 {
        40
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ExpireDelegationTokenRequest { }

impl Default for ExpireDelegationTokenRequest {
    fn default() -> Self {
        ExpireDelegationTokenRequest {
            hmac: Vec::new(),
            expiry_time_period_ms: 0_i64,
        }
    }
}

impl ExpireDelegationTokenRequest {
    pub fn new(hmac: Vec<u8>, expiry_time_period_ms: i64) -> Self {
        Self {
            hmac,
            expiry_time_period_ms,
        }
    }
}

#[cfg(test)]
mod tests_expire_delegation_token_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ExpireDelegationTokenRequest::new(
            Vec::new(),
            0_i64,
        );
        assert_eq!(d, ExpireDelegationTokenRequest::default());
    }
}

impl Readable for ExpireDelegationTokenRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let hmac = read_bytes(input, "hmac", false)?;
        let expiry_time_period_ms = i64::read(input)?;
        Ok(ExpireDelegationTokenRequest {
            hmac, expiry_time_period_ms
        })
    }
}

impl Writable for ExpireDelegationTokenRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_bytes(output, "self.hmac", &self.hmac, false)?;
        self.expiry_time_period_ms.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ExpireDelegationTokenRequest>("ExpireDelegationTokenRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ExpireDelegationTokenRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ExpireDelegationTokenRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ExpireDelegationTokenRequest", 0);
        }
    }
}
