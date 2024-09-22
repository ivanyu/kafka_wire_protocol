// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteTopicsRequest, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteTopicsRequest {
    /// The name or topic ID of the topic
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<DeleteTopicState>,
    /// The length of time in milliseconds to wait for the deletions to complete.
    pub timeout_ms: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteTopicsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        20
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        6
    }
}

impl Request for DeleteTopicsRequest { }

impl Default for DeleteTopicsRequest {
    fn default() -> Self {
        DeleteTopicsRequest {
            topics: Vec::<DeleteTopicState>::new(),
            timeout_ms: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteTopicsRequest {
    pub fn new(topics: Vec<DeleteTopicState>, timeout_ms: i32) -> Self {
        Self {
            topics,
            timeout_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_topics_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteTopicsRequest::new(
            Vec::<DeleteTopicState>::new(),
            0_i32,
        );
        assert_eq!(d, DeleteTopicsRequest::default());
    }
}

impl Readable for DeleteTopicsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<DeleteTopicState>(input, "topics", true)?;
        let timeout_ms = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteTopicsRequest {
            topics, timeout_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteTopicsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, true)?;
        self.timeout_ms.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteTopicState, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteTopicState {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub name: Option<String>,
    /// The unique topic ID
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteTopicState {
    fn default() -> Self {
        DeleteTopicState {
            name: None,
            topic_id: Uuid::nil(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteTopicState {
    pub fn new<S1: AsRef<str>>(name: Option<S1>, topic_id: Uuid) -> Self {
        Self {
            name: name.map(|s| s.as_ref().to_string()),
            topic_id,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_topic_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteTopicState::new(
            None::<String>,
            Uuid::nil(),
        );
        assert_eq!(d, DeleteTopicState::default());
    }
}

impl Readable for DeleteTopicState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = Option::<String>::read_ext(input, "name", true)?;
        let topic_id = Uuid::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteTopicState {
            name, topic_id, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteTopicState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.topic_id.write(output)?;
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
        crate::test_utils::test_java_default::<DeleteTopicsRequest>("DeleteTopicsRequest", 6);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteTopicsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteTopicsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteTopicsRequest", 6);
        }
    }
}
