// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};

/// HeartbeatResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct HeartbeatResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for HeartbeatResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        12
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for HeartbeatResponse { }

impl Default for HeartbeatResponse {
    fn default() -> Self {
        HeartbeatResponse {
            error_code: 0_i16,
        }
    }
}

impl HeartbeatResponse {
    pub fn new(error_code: i16) -> Self {
        Self {
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
            0_i16,
        );
        assert_eq!(d, HeartbeatResponse::default());
    }
}

impl Readable for HeartbeatResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        Ok(HeartbeatResponse {
            error_code
        })
    }
}

impl Writable for HeartbeatResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
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
        crate::test_utils::test_java_default::<HeartbeatResponse>("HeartbeatResponse", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "HeartbeatResponse", 0);
        }
    }
}
