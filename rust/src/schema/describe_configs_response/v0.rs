// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeConfigsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeConfigsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each resource.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<DescribeConfigsResult>,
}

impl ApiMessage for DescribeConfigsResponse {
    fn api_key(&self) -> i16 {
        32
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DescribeConfigsResponse { }

impl Default for DescribeConfigsResponse {
    fn default() -> Self {
        DescribeConfigsResponse {
            throttle_time_ms: 0_i32,
            results: Vec::<DescribeConfigsResult>::new(),
        }
    }
}

impl DescribeConfigsResponse {
    pub fn new(throttle_time_ms: i32, results: Vec<DescribeConfigsResult>) -> Self {
        Self {
            throttle_time_ms,
            results,
        }
    }
}

#[cfg(test)]
mod tests_describe_configs_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeConfigsResponse::new(
            0_i32,
            Vec::<DescribeConfigsResult>::new(),
        );
        assert_eq!(d, DescribeConfigsResponse::default());
    }
}

impl Readable for DescribeConfigsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let results = read_array::<DescribeConfigsResult>(input, "results", false)?;
        Ok(DescribeConfigsResponse {
            throttle_time_ms, results
        })
    }
}

impl Writable for DescribeConfigsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.results", &self.results, false)?;
        Ok(())
    }
}

/// DescribeConfigsResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeConfigsResult {
    /// The error code, or 0 if we were able to successfully describe the configurations.
    pub error_code: i16,
    /// The error message, or null if we were able to successfully describe the configurations.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The resource type.
    pub resource_type: i8,
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
    /// Each listed configuration.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub configs: Vec<DescribeConfigsResourceResult>,
}

impl Default for DescribeConfigsResult {
    fn default() -> Self {
        DescribeConfigsResult {
            error_code: 0_i16,
            error_message: Some(String::from("")),
            resource_type: 0_i8,
            resource_name: String::from(""),
            configs: Vec::<DescribeConfigsResourceResult>::new(),
        }
    }
}

impl DescribeConfigsResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(error_code: i16, error_message: Option<S1>, resource_type: i8, resource_name: S2, configs: Vec<DescribeConfigsResourceResult>) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
            configs,
        }
    }
}

#[cfg(test)]
mod tests_describe_configs_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeConfigsResult::new(
            0_i16,
            Some(String::from("")),
            0_i8,
            String::from(""),
            Vec::<DescribeConfigsResourceResult>::new(),
        );
        assert_eq!(d, DescribeConfigsResult::default());
    }
}

impl Readable for DescribeConfigsResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", false)?;
        let configs = read_array::<DescribeConfigsResourceResult>(input, "configs", false)?;
        Ok(DescribeConfigsResult {
            error_code, error_message, resource_type, resource_name, configs
        })
    }
}

impl Writable for DescribeConfigsResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", false)?;
        write_array(output, "self.configs", &self.configs, false)?;
        Ok(())
    }
}

/// DescribeConfigsResourceResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeConfigsResourceResult {
    /// The configuration name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The configuration value.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub value: Option<String>,
    /// True if the configuration is read-only.
    pub read_only: bool,
    /// True if the configuration is not set.
    pub is_default: bool,
    /// True if this configuration is sensitive.
    pub is_sensitive: bool,
}

impl Default for DescribeConfigsResourceResult {
    fn default() -> Self {
        DescribeConfigsResourceResult {
            name: String::from(""),
            value: Some(String::from("")),
            read_only: false,
            is_default: false,
            is_sensitive: false,
        }
    }
}

impl DescribeConfigsResourceResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, value: Option<S2>, read_only: bool, is_default: bool, is_sensitive: bool) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value: value.map(|s| s.as_ref().to_string()),
            read_only,
            is_default,
            is_sensitive,
        }
    }
}

#[cfg(test)]
mod tests_describe_configs_resource_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeConfigsResourceResult::new(
            String::from(""),
            Some(String::from("")),
            false,
            false,
            false,
        );
        assert_eq!(d, DescribeConfigsResourceResult::default());
    }
}

impl Readable for DescribeConfigsResourceResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let value = Option::<String>::read_ext(input, "value", false)?;
        let read_only = bool::read(input)?;
        let is_default = bool::read(input)?;
        let is_sensitive = bool::read(input)?;
        Ok(DescribeConfigsResourceResult {
            name, value, read_only, is_default, is_sensitive
        })
    }
}

impl Writable for DescribeConfigsResourceResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        self.value.write_ext(output, "self.value", false)?;
        self.read_only.write(output)?;
        self.is_default.write(output)?;
        self.is_sensitive.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DescribeConfigsResponse>("DescribeConfigsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeConfigsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeConfigsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeConfigsResponse", 0);
        }
    }
}
