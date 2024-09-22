// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// CreatePartitionsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatePartitionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The partition creation results for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<CreatePartitionsTopicResult>,
}

impl ApiMessage for CreatePartitionsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        37
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for CreatePartitionsResponse { }

impl Default for CreatePartitionsResponse {
    fn default() -> Self {
        CreatePartitionsResponse {
            throttle_time_ms: 0_i32,
            results: Vec::<CreatePartitionsTopicResult>::new(),
        }
    }
}

impl CreatePartitionsResponse {
    pub fn new(throttle_time_ms: i32, results: Vec<CreatePartitionsTopicResult>) -> Self {
        Self {
            throttle_time_ms,
            results,
        }
    }
}

#[cfg(test)]
mod tests_create_partitions_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatePartitionsResponse::new(
            0_i32,
            Vec::<CreatePartitionsTopicResult>::new(),
        );
        assert_eq!(d, CreatePartitionsResponse::default());
    }
}

impl Readable for CreatePartitionsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let results = read_array::<CreatePartitionsTopicResult>(input, "results", false)?;
        Ok(CreatePartitionsResponse {
            throttle_time_ms, results
        })
    }
}

impl Writable for CreatePartitionsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.results", &self.results, false)?;
        Ok(())
    }
}

/// CreatePartitionsTopicResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatePartitionsTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The result error, or zero if there was no error.
    pub error_code: i16,
    /// The result message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
}

impl Default for CreatePartitionsTopicResult {
    fn default() -> Self {
        CreatePartitionsTopicResult {
            name: String::from(""),
            error_code: 0_i16,
            error_message: None,
        }
    }
}

impl CreatePartitionsTopicResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, error_code: i16, error_message: Option<S2>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
        }
    }
}

#[cfg(test)]
mod tests_create_partitions_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatePartitionsTopicResult::new(
            String::from(""),
            0_i16,
            None::<String>,
        );
        assert_eq!(d, CreatePartitionsTopicResult::default());
    }
}

impl Readable for CreatePartitionsTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        Ok(CreatePartitionsTopicResult {
            name, error_code, error_message
        })
    }
}

impl Writable for CreatePartitionsTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<CreatePartitionsResponse>("CreatePartitionsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreatePartitionsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreatePartitionsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "CreatePartitionsResponse", 0);
        }
    }
}
