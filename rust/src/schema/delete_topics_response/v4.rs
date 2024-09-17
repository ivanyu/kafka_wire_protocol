// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteTopicsResponse, version 4.
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
        4
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

/// DeletableTopicResult, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeletableTopicResult {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeletableTopicResult {
    fn default() -> Self {
        DeletableTopicResult {
            name: String::from(""),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeletableTopicResult {
    pub fn new<S1: AsRef<str>>(name: S1, error_code: i16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            error_code,
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
            String::from(""),
            0_i16,
        );
        assert_eq!(d, DeletableTopicResult::default());
    }
}

impl Readable for DeletableTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeletableTopicResult {
            name, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for DeletableTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.error_code.write(output)?;
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
        crate::test_utils::test_java_default::<DeleteTopicsResponse>("DeleteTopicsResponse", 4);
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
            crate::test_utils::test_java_arbitrary(&data, "DeleteTopicsResponse", 4);
        }
    }
}
