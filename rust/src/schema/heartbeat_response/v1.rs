// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};

/// HeartbeatResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct HeartbeatResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for HeartbeatResponse {
    fn api_key(&self) -> i16 {
        12
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for HeartbeatResponse { }

impl Default for HeartbeatResponse {
    fn default() -> Self {
        HeartbeatResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
        }
    }
}

impl HeartbeatResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16) -> Self {
        Self {
            throttle_time_ms,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_heartbeat_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = HeartbeatResponse::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, HeartbeatResponse::default());
    }
}

impl Readable for HeartbeatResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        Ok(HeartbeatResponse {
            throttle_time_ms, error_code
        })
    }
}

impl Writable for HeartbeatResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<HeartbeatResponse>("HeartbeatResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: HeartbeatResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: HeartbeatResponse) {
            crate::test_utils::test_java_arbitrary(&data, "HeartbeatResponse", 1);
        }
    }
}
