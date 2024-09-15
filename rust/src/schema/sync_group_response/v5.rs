// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SyncGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The group protocol type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub protocol_type: Option<String>,
    /// The group protocol name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub protocol_name: Option<String>,
    /// The member assignment.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub assignment: Vec<u8>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for SyncGroupResponse {
    fn api_key(&self) -> i16 {
        14
    }
    
    fn version(&self) -> i16 {
        5
    }
}

impl Response for SyncGroupResponse { }

impl Default for SyncGroupResponse {
    fn default() -> Self {
        SyncGroupResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            protocol_type: None,
            protocol_name: None,
            assignment: Vec::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl SyncGroupResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(throttle_time_ms: i32, error_code: i16, protocol_type: Option<S1>, protocol_name: Option<S2>, assignment: Vec<u8>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            protocol_type: protocol_type.map(|s| s.as_ref().to_string()),
            protocol_name: protocol_name.map(|s| s.as_ref().to_string()),
            assignment,
            _unknown_tagged_fields: vec![],
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
            None::<String>,
            None::<String>,
            Vec::new(),
        );
        assert_eq!(d, SyncGroupResponse::default());
    }
}

impl Readable for SyncGroupResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let protocol_type = Option::<String>::read_ext(input, "protocol_type", true)?;
        let protocol_name = Option::<String>::read_ext(input, "protocol_name", true)?;
        let assignment = read_bytes(input, "assignment", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(SyncGroupResponse {
            throttle_time_ms, error_code, protocol_type, protocol_name, assignment, _unknown_tagged_fields
        })
    }
}

impl Writable for SyncGroupResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.protocol_type.write_ext(output, "self.protocol_type", true)?;
        self.protocol_name.write_ext(output, "self.protocol_name", true)?;
        write_bytes(output, "self.assignment", &self.assignment, true)?;
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
        crate::test_utils::test_java_default::<SyncGroupResponse>("SyncGroupResponse", 5);
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
            crate::test_utils::test_java_arbitrary(&data, "SyncGroupResponse", 5);
        }
    }
}
