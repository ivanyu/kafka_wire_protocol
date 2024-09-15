// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateTopicsResponse {
    /// Results for each topic we tried to create.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<CreatableTopicResult>,
}

impl ApiMessage for CreateTopicsResponse {
    fn api_key(&self) -> i16 {
        19
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for CreateTopicsResponse { }

impl Default for CreateTopicsResponse {
    fn default() -> Self {
        CreateTopicsResponse {
            topics: Vec::<CreatableTopicResult>::new(),
        }
    }
}

impl CreateTopicsResponse {
    pub fn new(topics: Vec<CreatableTopicResult>) -> Self {
        Self {
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
            Vec::<CreatableTopicResult>::new(),
        );
        assert_eq!(d, CreateTopicsResponse::default());
    }
}

impl Readable for CreateTopicsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<CreatableTopicResult>(input, "topics", false)?;
        Ok(CreateTopicsResponse {
            topics
        })
    }
}

impl Writable for CreateTopicsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatableTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for CreatableTopicResult {
    fn api_key(&self) -> i16 {
        19
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for CreatableTopicResult { }

impl Default for CreatableTopicResult {
    fn default() -> Self {
        CreatableTopicResult {
            name: String::from(""),
            error_code: 0_i16,
        }
    }
}

impl CreatableTopicResult {
    pub fn new<S1: AsRef<str>>(name: S1, error_code: i16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            error_code,
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
        );
        assert_eq!(d, CreatableTopicResult::default());
    }
}

impl Readable for CreatableTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let error_code = i16::read(input)?;
        Ok(CreatableTopicResult {
            name, error_code
        })
    }
}

impl Writable for CreatableTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
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
        crate::test_utils::test_java_default::<CreateTopicsResponse>("CreateTopicsResponse", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "CreateTopicsResponse", 0);
        }
    }
}
