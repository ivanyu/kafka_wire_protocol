// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaveGroupResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for LeaveGroupResponse {
    fn api_key(&self) -> i16 {
        13
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for LeaveGroupResponse { }

impl Default for LeaveGroupResponse {
    fn default() -> Self {
        LeaveGroupResponse {
            error_code: 0_i16,
        }
    }
}

impl LeaveGroupResponse {
    pub fn new(error_code: i16) -> Self {
        Self {
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_leave_group_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaveGroupResponse::new(
            0_i16,
        );
        assert_eq!(d, LeaveGroupResponse::default());
    }
}

impl Readable for LeaveGroupResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        Ok(LeaveGroupResponse {
            error_code
        })
    }
}

impl Writable for LeaveGroupResponse {
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
        crate::test_utils::test_java_default::<LeaveGroupResponse>("LeaveGroupResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: LeaveGroupResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: LeaveGroupResponse) {
            crate::test_utils::test_java_arbitrary(&data, "LeaveGroupResponse", 0);
        }
    }
}
