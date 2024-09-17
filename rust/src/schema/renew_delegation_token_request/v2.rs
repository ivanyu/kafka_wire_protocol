// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// RenewDelegationTokenRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RenewDelegationTokenRequest {
    /// The HMAC of the delegation token to be renewed.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub hmac: Vec<u8>,
    /// The renewal time period in milliseconds.
    pub renew_period_ms: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for RenewDelegationTokenRequest {
    fn api_key(&self) -> i16 {
        39
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for RenewDelegationTokenRequest { }

impl Default for RenewDelegationTokenRequest {
    fn default() -> Self {
        RenewDelegationTokenRequest {
            hmac: Vec::new(),
            renew_period_ms: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl RenewDelegationTokenRequest {
    pub fn new(hmac: Vec<u8>, renew_period_ms: i64) -> Self {
        Self {
            hmac,
            renew_period_ms,
            _unknown_tagged_fields: vec![],
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
        let hmac = read_bytes(input, "hmac", true)?;
        let renew_period_ms = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(RenewDelegationTokenRequest {
            hmac, renew_period_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for RenewDelegationTokenRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_bytes(output, "self.hmac", &self.hmac, true)?;
        self.renew_period_ms.write(output)?;
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
        crate::test_utils::test_java_default::<RenewDelegationTokenRequest>("RenewDelegationTokenRequest", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "RenewDelegationTokenRequest", 2);
        }
    }
}
