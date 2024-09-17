// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// PushTelemetryResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PushTelemetryResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for PushTelemetryResponse {
    fn api_key(&self) -> i16 {
        72
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for PushTelemetryResponse { }

impl Default for PushTelemetryResponse {
    fn default() -> Self {
        PushTelemetryResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PushTelemetryResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_push_telemetry_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PushTelemetryResponse::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, PushTelemetryResponse::default());
    }
}

impl Readable for PushTelemetryResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PushTelemetryResponse {
            throttle_time_ms, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for PushTelemetryResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
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
        crate::test_utils::test_java_default::<PushTelemetryResponse>("PushTelemetryResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: PushTelemetryResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: PushTelemetryResponse) {
            crate::test_utils::test_java_arbitrary(&data, "PushTelemetryResponse", 0);
        }
    }
}
