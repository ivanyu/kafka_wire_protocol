// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FindCoordinatorResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The node id.
    pub node_id: i32,
    /// The host name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The port.
    pub port: i32,
}

impl ApiMessage for FindCoordinatorResponse {
    fn api_key(&self) -> i16 {
        10
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for FindCoordinatorResponse { }

impl Default for FindCoordinatorResponse {
    fn default() -> Self {
        FindCoordinatorResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            node_id: 0_i32,
            host: String::from(""),
            port: 0_i32,
        }
    }
}

impl FindCoordinatorResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, node_id: i32, host: S2, port: i32) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            node_id,
            host: host.as_ref().to_string(),
            port,
        }
    }
}

#[cfg(test)]
mod tests_find_coordinator_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FindCoordinatorResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
            0_i32,
            String::from(""),
            0_i32,
        );
        assert_eq!(d, FindCoordinatorResponse::default());
    }
}

impl Readable for FindCoordinatorResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        let node_id = i32::read(input)?;
        let host = String::read_ext(input, "host", false)?;
        let port = i32::read(input)?;
        Ok(FindCoordinatorResponse {
            throttle_time_ms, error_code, error_message, node_id, host, port
        })
    }
}

impl Writable for FindCoordinatorResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        self.node_id.write(output)?;
        self.host.write_ext(output, "self.host", false)?;
        self.port.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<FindCoordinatorResponse>("FindCoordinatorResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: FindCoordinatorResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: FindCoordinatorResponse) {
            crate::test_utils::test_java_arbitrary(&data, "FindCoordinatorResponse", 1);
        }
    }
}
