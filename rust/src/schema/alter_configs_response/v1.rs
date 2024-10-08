// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AlterConfigsResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterConfigsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses for each resource.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<AlterConfigsResourceResponse>,
}

impl ApiMessage for AlterConfigsResponse {
    fn api_key(&self) -> i16 {
        33
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for AlterConfigsResponse { }

impl Default for AlterConfigsResponse {
    fn default() -> Self {
        AlterConfigsResponse {
            throttle_time_ms: 0_i32,
            responses: Vec::<AlterConfigsResourceResponse>::new(),
        }
    }
}

impl AlterConfigsResponse {
    pub fn new(throttle_time_ms: i32, responses: Vec<AlterConfigsResourceResponse>) -> Self {
        Self {
            throttle_time_ms,
            responses,
        }
    }
}

#[cfg(test)]
mod tests_alter_configs_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterConfigsResponse::new(
            0_i32,
            Vec::<AlterConfigsResourceResponse>::new(),
        );
        assert_eq!(d, AlterConfigsResponse::default());
    }
}

impl Readable for AlterConfigsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let responses = read_array::<AlterConfigsResourceResponse>(input, "responses", false)?;
        Ok(AlterConfigsResponse {
            throttle_time_ms, responses
        })
    }
}

impl Writable for AlterConfigsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.responses", &self.responses, false)?;
        Ok(())
    }
}

/// AlterConfigsResourceResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterConfigsResourceResponse {
    /// The resource error code.
    pub error_code: i16,
    /// The resource error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The resource type.
    pub resource_type: i8,
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
}

impl Default for AlterConfigsResourceResponse {
    fn default() -> Self {
        AlterConfigsResourceResponse {
            error_code: 0_i16,
            error_message: Some(String::from("")),
            resource_type: 0_i8,
            resource_name: String::from(""),
        }
    }
}

impl AlterConfigsResourceResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(error_code: i16, error_message: Option<S1>, resource_type: i8, resource_name: S2) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
        }
    }
}

#[cfg(test)]
mod tests_alter_configs_resource_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterConfigsResourceResponse::new(
            0_i16,
            Some(String::from("")),
            0_i8,
            String::from(""),
        );
        assert_eq!(d, AlterConfigsResourceResponse::default());
    }
}

impl Readable for AlterConfigsResourceResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", false)?;
        Ok(AlterConfigsResourceResponse {
            error_code, error_message, resource_type, resource_name
        })
    }
}

impl Writable for AlterConfigsResourceResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<AlterConfigsResponse>("AlterConfigsResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterConfigsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterConfigsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AlterConfigsResponse", 1);
        }
    }
}
