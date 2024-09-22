// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeConfigsResponse, version 3.
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
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        32
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
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

/// DescribeConfigsResult, version 3.
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

/// DescribeConfigsResourceResult, version 3.
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
    /// The configuration source.
    pub config_source: i8,
    /// True if this configuration is sensitive.
    pub is_sensitive: bool,
    /// The synonyms for this configuration key.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub synonyms: Vec<DescribeConfigsSynonym>,
    /// The configuration data type. Type can be one of the following values - BOOLEAN, STRING, INT, SHORT, LONG, DOUBLE, LIST, CLASS, PASSWORD
    pub config_type: i8,
    /// The configuration documentation.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub documentation: Option<String>,
}

impl Default for DescribeConfigsResourceResult {
    fn default() -> Self {
        DescribeConfigsResourceResult {
            name: String::from(""),
            value: Some(String::from("")),
            read_only: false,
            config_source: -1_i8,
            is_sensitive: false,
            synonyms: Vec::<DescribeConfigsSynonym>::new(),
            config_type: 0_i8,
            documentation: Some(String::from("")),
        }
    }
}

impl DescribeConfigsResourceResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(name: S1, value: Option<S2>, read_only: bool, config_source: i8, is_sensitive: bool, synonyms: Vec<DescribeConfigsSynonym>, config_type: i8, documentation: Option<S3>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value: value.map(|s| s.as_ref().to_string()),
            read_only,
            config_source,
            is_sensitive,
            synonyms,
            config_type,
            documentation: documentation.map(|s| s.as_ref().to_string()),
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
            -1_i8,
            false,
            Vec::<DescribeConfigsSynonym>::new(),
            0_i8,
            Some(String::from("")),
        );
        assert_eq!(d, DescribeConfigsResourceResult::default());
    }
}

impl Readable for DescribeConfigsResourceResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let value = Option::<String>::read_ext(input, "value", false)?;
        let read_only = bool::read(input)?;
        let config_source = i8::read(input)?;
        let is_sensitive = bool::read(input)?;
        let synonyms = read_array::<DescribeConfigsSynonym>(input, "synonyms", false)?;
        let config_type = i8::read(input)?;
        let documentation = Option::<String>::read_ext(input, "documentation", false)?;
        Ok(DescribeConfigsResourceResult {
            name, value, read_only, config_source, is_sensitive, synonyms, config_type, documentation
        })
    }
}

impl Writable for DescribeConfigsResourceResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        self.value.write_ext(output, "self.value", false)?;
        self.read_only.write(output)?;
        self.config_source.write(output)?;
        self.is_sensitive.write(output)?;
        write_array(output, "self.synonyms", &self.synonyms, false)?;
        self.config_type.write(output)?;
        self.documentation.write_ext(output, "self.documentation", false)?;
        Ok(())
    }
}

/// DescribeConfigsSynonym, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeConfigsSynonym {
    /// The synonym name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The synonym value.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub value: Option<String>,
    /// The synonym source.
    pub source: i8,
}

impl Default for DescribeConfigsSynonym {
    fn default() -> Self {
        DescribeConfigsSynonym {
            name: String::from(""),
            value: Some(String::from("")),
            source: 0_i8,
        }
    }
}

impl DescribeConfigsSynonym {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, value: Option<S2>, source: i8) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value: value.map(|s| s.as_ref().to_string()),
            source,
        }
    }
}

#[cfg(test)]
mod tests_describe_configs_synonym_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeConfigsSynonym::new(
            String::from(""),
            Some(String::from("")),
            0_i8,
        );
        assert_eq!(d, DescribeConfigsSynonym::default());
    }
}

impl Readable for DescribeConfigsSynonym {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let value = Option::<String>::read_ext(input, "value", false)?;
        let source = i8::read(input)?;
        Ok(DescribeConfigsSynonym {
            name, value, source
        })
    }
}

impl Writable for DescribeConfigsSynonym {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        self.value.write_ext(output, "self.value", false)?;
        self.source.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DescribeConfigsResponse>("DescribeConfigsResponse", 3);
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
            crate::test_utils::test_java_arbitrary(&data, "DescribeConfigsResponse", 3);
        }
    }
}
