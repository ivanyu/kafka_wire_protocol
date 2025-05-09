// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteTopicsResponse, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each topic we tried to delete.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<DeletableTopicResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteTopicsResponse {
    fn api_key(&self) -> i16 {
        20
    }
    
    fn version(&self) -> i16 {
        6
    }
}

impl Response for DeleteTopicsResponse { }

impl Default for DeleteTopicsResponse {
    fn default() -> Self {
        DeleteTopicsResponse {
            throttle_time_ms: 0_i32,
            responses: Vec::<DeletableTopicResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteTopicsResponse {
    pub fn new(throttle_time_ms: i32, responses: Vec<DeletableTopicResult>) -> Self {
        Self {
            throttle_time_ms,
            responses,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_topics_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteTopicsResponse::new(
            0_i32,
            Vec::<DeletableTopicResult>::new(),
        );
        assert_eq!(d, DeleteTopicsResponse::default());
    }
}

impl Readable for DeleteTopicsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let responses = read_array::<DeletableTopicResult>(input, "responses", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteTopicsResponse {
            throttle_time_ms, responses, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteTopicsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.responses", &self.responses, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeletableTopicResult, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeletableTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub name: Option<String>,
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeletableTopicResult {
    fn default() -> Self {
        DeletableTopicResult {
            name: Some(String::from("")),
            topic_id: Uuid::nil(),
            error_code: 0_i16,
            error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeletableTopicResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: Option<S1>, topic_id: Uuid, error_code: i16, error_message: Option<S2>) -> Self {
        Self {
            name: name.map(|s| s.as_ref().to_string()),
            topic_id,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_deletable_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeletableTopicResult::new(
            Some(String::from("")),
            Uuid::nil(),
            0_i16,
            None::<String>,
        );
        assert_eq!(d, DeletableTopicResult::default());
    }
}

impl Readable for DeletableTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = Option::<String>::read_ext(input, "name", true)?;
        let topic_id = Uuid::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeletableTopicResult {
            name, topic_id, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for DeletableTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.topic_id.write(output)?;
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
        crate::test_utils::test_java_default::<DeleteTopicsResponse>("DeleteTopicsResponse", 6);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteTopicsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteTopicsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteTopicsResponse", 6);
        }
    }
}
