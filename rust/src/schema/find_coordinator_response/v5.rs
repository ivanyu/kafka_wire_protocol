// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// FindCoordinatorResponse, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FindCoordinatorResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each coordinator result in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub coordinators: Vec<Coordinator>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for FindCoordinatorResponse {
    fn api_key(&self) -> i16 {
        10
    }
    
    fn version(&self) -> i16 {
        5
    }
}

impl Response for FindCoordinatorResponse { }

impl Default for FindCoordinatorResponse {
    fn default() -> Self {
        FindCoordinatorResponse {
            throttle_time_ms: 0_i32,
            coordinators: Vec::<Coordinator>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FindCoordinatorResponse {
    pub fn new(throttle_time_ms: i32, coordinators: Vec<Coordinator>) -> Self {
        Self {
            throttle_time_ms,
            coordinators,
            _unknown_tagged_fields: vec![],
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
            Vec::<Coordinator>::new(),
        );
        assert_eq!(d, FindCoordinatorResponse::default());
    }
}

impl Readable for FindCoordinatorResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let coordinators = read_array::<Coordinator>(input, "coordinators", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FindCoordinatorResponse {
            throttle_time_ms, coordinators, _unknown_tagged_fields
        })
    }
}

impl Writable for FindCoordinatorResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.coordinators", &self.coordinators, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Coordinator, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Coordinator {
    /// The coordinator key.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub key: String,
    /// The node id.
    pub node_id: i32,
    /// The host name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The port.
    pub port: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Coordinator {
    fn default() -> Self {
        Coordinator {
            key: String::from(""),
            node_id: 0_i32,
            host: String::from(""),
            port: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Coordinator {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(key: S1, node_id: i32, host: S2, port: i32, error_code: i16, error_message: Option<S3>) -> Self {
        Self {
            key: key.as_ref().to_string(),
            node_id,
            host: host.as_ref().to_string(),
            port,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_coordinator_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Coordinator::new(
            String::from(""),
            0_i32,
            String::from(""),
            0_i32,
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, Coordinator::default());
    }
}

impl Readable for Coordinator {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let key = String::read_ext(input, "key", true)?;
        let node_id = i32::read(input)?;
        let host = String::read_ext(input, "host", true)?;
        let port = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Coordinator {
            key, node_id, host, port, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for Coordinator {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key.write_ext(output, "self.key", true)?;
        self.node_id.write(output)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
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
        crate::test_utils::test_java_default::<FindCoordinatorResponse>("FindCoordinatorResponse", 5);
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
            crate::test_utils::test_java_arbitrary(&data, "FindCoordinatorResponse", 5);
        }
    }
}
