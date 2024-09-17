// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// CreateTopicsResponse, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Results for each topic we tried to create.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<CreatableTopicResult>,
}

impl ApiMessage for CreateTopicsResponse {
    fn api_key(&self) -> i16 {
        19
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Response for CreateTopicsResponse { }

impl Default for CreateTopicsResponse {
    fn default() -> Self {
        CreateTopicsResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<CreatableTopicResult>::new(),
        }
    }
}

impl CreateTopicsResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<CreatableTopicResult>) -> Self {
        Self {
            throttle_time_ms,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_create_topics_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateTopicsResponse::new(
            0_i32,
            Vec::<CreatableTopicResult>::new(),
        );
        assert_eq!(d, CreateTopicsResponse::default());
    }
}

impl Readable for CreateTopicsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<CreatableTopicResult>(input, "topics", false)?;
        Ok(CreateTopicsResponse {
            throttle_time_ms, topics
        })
    }
}

impl Writable for CreateTopicsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// CreatableTopicResult, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatableTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
}

impl Default for CreatableTopicResult {
    fn default() -> Self {
        CreatableTopicResult {
            name: String::from(""),
            error_code: 0_i16,
            error_message: Some(String::from("")),
        }
    }
}

impl CreatableTopicResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, error_code: i16, error_message: Option<S2>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
        }
    }
}

#[cfg(test)]
mod tests_creatable_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatableTopicResult::new(
            String::from(""),
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, CreatableTopicResult::default());
    }
}

impl Readable for CreatableTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        Ok(CreatableTopicResult {
            name, error_code, error_message
        })
    }
}

impl Writable for CreatableTopicResult {
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
        crate::test_utils::test_java_default::<CreateTopicsResponse>("CreateTopicsResponse", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreateTopicsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreateTopicsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "CreateTopicsResponse", 4);
        }
    }
}
