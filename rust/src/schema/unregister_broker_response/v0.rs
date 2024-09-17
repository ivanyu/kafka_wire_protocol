// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// UnregisterBrokerResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UnregisterBrokerResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The top-level error message, or `null` if there was no top-level error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for UnregisterBrokerResponse {
    fn api_key(&self) -> i16 {
        64
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for UnregisterBrokerResponse { }

impl Default for UnregisterBrokerResponse {
    fn default() -> Self {
        UnregisterBrokerResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UnregisterBrokerResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_unregister_broker_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UnregisterBrokerResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, UnregisterBrokerResponse::default());
    }
}

impl Readable for UnregisterBrokerResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UnregisterBrokerResponse {
            throttle_time_ms, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for UnregisterBrokerResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
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
        crate::test_utils::test_java_default::<UnregisterBrokerResponse>("UnregisterBrokerResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UnregisterBrokerResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UnregisterBrokerResponse) {
            crate::test_utils::test_java_arbitrary(&data, "UnregisterBrokerResponse", 0);
        }
    }
}
