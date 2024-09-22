// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// BrokerRegistrationResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BrokerRegistrationResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The broker's assigned epoch, or -1 if none was assigned.
    pub broker_epoch: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for BrokerRegistrationResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        62
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        1
    }
}

impl Response for BrokerRegistrationResponse { }

impl Default for BrokerRegistrationResponse {
    fn default() -> Self {
        BrokerRegistrationResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            broker_epoch: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl BrokerRegistrationResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, broker_epoch: i64) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            broker_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_broker_registration_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = BrokerRegistrationResponse::new(
            0_i32,
            0_i16,
            -1_i64,
        );
        assert_eq!(d, BrokerRegistrationResponse::default());
    }
}

impl Readable for BrokerRegistrationResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let broker_epoch = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(BrokerRegistrationResponse {
            throttle_time_ms, error_code, broker_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for BrokerRegistrationResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.broker_epoch.write(output)?;
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
        crate::test_utils::test_java_default::<BrokerRegistrationResponse>("BrokerRegistrationResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: BrokerRegistrationResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: BrokerRegistrationResponse) {
            crate::test_utils::test_java_arbitrary(&data, "BrokerRegistrationResponse", 1);
        }
    }
}
