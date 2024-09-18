// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// SyncGroupResponse, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SyncGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The member assignment.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub assignment: Vec<u8>,
}

impl ApiMessage for SyncGroupResponse {
    fn api_key(&self) -> i16 {
        14
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Response for SyncGroupResponse { }

impl Default for SyncGroupResponse {
    fn default() -> Self {
        SyncGroupResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            assignment: Vec::new(),
        }
    }
}

impl SyncGroupResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, assignment: Vec<u8>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            assignment,
        }
    }
}

#[cfg(test)]
mod tests_sync_group_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SyncGroupResponse::new(
            0_i32,
            0_i16,
            Vec::new(),
        );
        assert_eq!(d, SyncGroupResponse::default());
    }
}

impl Readable for SyncGroupResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let assignment = read_bytes(input, "assignment", false)?;
        Ok(SyncGroupResponse {
            throttle_time_ms, error_code, assignment
        })
    }
}

impl Writable for SyncGroupResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_bytes(output, "self.assignment", &self.assignment, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<SyncGroupResponse>("SyncGroupResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: SyncGroupResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: SyncGroupResponse) {
            crate::test_utils::test_java_arbitrary(&data, "SyncGroupResponse", 3);
        }
    }
}
