// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// BrokerHeartbeatResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BrokerHeartbeatResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// True if the broker has approximately caught up with the latest metadata.
    pub is_caught_up: bool,
    /// True if the broker is fenced.
    pub is_fenced: bool,
    /// True if the broker should proceed with its shutdown.
    pub should_shut_down: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for BrokerHeartbeatResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        63
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for BrokerHeartbeatResponse { }

impl Default for BrokerHeartbeatResponse {
    fn default() -> Self {
        BrokerHeartbeatResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            is_caught_up: false,
            is_fenced: true,
            should_shut_down: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl BrokerHeartbeatResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, is_caught_up: bool, is_fenced: bool, should_shut_down: bool) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            is_caught_up,
            is_fenced,
            should_shut_down,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_broker_heartbeat_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = BrokerHeartbeatResponse::new(
            0_i32,
            0_i16,
            false,
            true,
            false,
        );
        assert_eq!(d, BrokerHeartbeatResponse::default());
    }
}

impl Readable for BrokerHeartbeatResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let is_caught_up = bool::read(input)?;
        let is_fenced = bool::read(input)?;
        let should_shut_down = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(BrokerHeartbeatResponse {
            throttle_time_ms, error_code, is_caught_up, is_fenced, should_shut_down, _unknown_tagged_fields
        })
    }
}

impl Writable for BrokerHeartbeatResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.is_caught_up.write(output)?;
        self.is_fenced.write(output)?;
        self.should_shut_down.write(output)?;
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
        crate::test_utils::test_java_default::<BrokerHeartbeatResponse>("BrokerHeartbeatResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: BrokerHeartbeatResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: BrokerHeartbeatResponse) {
            crate::test_utils::test_java_arbitrary(&data, "BrokerHeartbeatResponse", 0);
        }
    }
}
