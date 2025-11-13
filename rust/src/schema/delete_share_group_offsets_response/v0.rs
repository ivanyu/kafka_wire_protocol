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

/// DeleteShareGroupOffsetsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteShareGroupOffsetsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The results for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<DeleteShareGroupOffsetsResponseTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteShareGroupOffsetsResponse {
    fn api_key(&self) -> i16 {
        92
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DeleteShareGroupOffsetsResponse { }

impl Default for DeleteShareGroupOffsetsResponse {
    fn default() -> Self {
        DeleteShareGroupOffsetsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: None,
            responses: Vec::<DeleteShareGroupOffsetsResponseTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteShareGroupOffsetsResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, responses: Vec<DeleteShareGroupOffsetsResponseTopic>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            responses,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_share_group_offsets_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteShareGroupOffsetsResponse::new(
            0_i32,
            0_i16,
            None::<String>,
            Vec::<DeleteShareGroupOffsetsResponseTopic>::new(),
        );
        assert_eq!(d, DeleteShareGroupOffsetsResponse::default());
    }
}

impl Readable for DeleteShareGroupOffsetsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let responses = read_array::<DeleteShareGroupOffsetsResponseTopic>(input, "responses", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteShareGroupOffsetsResponse {
            throttle_time_ms, error_code, error_message, responses, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteShareGroupOffsetsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.responses", &self.responses, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteShareGroupOffsetsResponseTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteShareGroupOffsetsResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The topic-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The topic-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteShareGroupOffsetsResponseTopic {
    fn default() -> Self {
        DeleteShareGroupOffsetsResponseTopic {
            topic_name: String::from(""),
            topic_id: Uuid::nil(),
            error_code: 0_i16,
            error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteShareGroupOffsetsResponseTopic {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(topic_name: S1, topic_id: Uuid, error_code: i16, error_message: Option<S2>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            topic_id,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_share_group_offsets_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteShareGroupOffsetsResponseTopic::new(
            String::from(""),
            Uuid::nil(),
            0_i16,
            None::<String>,
        );
        assert_eq!(d, DeleteShareGroupOffsetsResponseTopic::default());
    }
}

impl Readable for DeleteShareGroupOffsetsResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let topic_id = Uuid::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteShareGroupOffsetsResponseTopic {
            topic_name, topic_id, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteShareGroupOffsetsResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
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
        crate::test_utils::test_java_default::<DeleteShareGroupOffsetsResponse>("DeleteShareGroupOffsetsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteShareGroupOffsetsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteShareGroupOffsetsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteShareGroupOffsetsResponse", 0);
        }
    }
}
