// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddOffsetsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The response error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for AddOffsetsToTxnResponse {
    fn api_key(&self) -> i16 {
        25
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for AddOffsetsToTxnResponse { }

impl Default for AddOffsetsToTxnResponse {
    fn default() -> Self {
        AddOffsetsToTxnResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
        }
    }
}

impl AddOffsetsToTxnResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16) -> Self {
        Self {
            throttle_time_ms,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_add_offsets_to_txn_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddOffsetsToTxnResponse::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, AddOffsetsToTxnResponse::default());
    }
}

impl Readable for AddOffsetsToTxnResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        Ok(AddOffsetsToTxnResponse {
            throttle_time_ms, error_code
        })
    }
}

impl Writable for AddOffsetsToTxnResponse {
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
        crate::test_utils::test_java_default::<AddOffsetsToTxnResponse>("AddOffsetsToTxnResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AddOffsetsToTxnResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AddOffsetsToTxnResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AddOffsetsToTxnResponse", 2);
        }
    }
}
