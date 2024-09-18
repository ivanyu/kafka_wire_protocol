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

/// LeaderAndIsrResponse, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Each topic
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<LeaderAndIsrTopicError>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for LeaderAndIsrResponse {
    fn api_key(&self) -> i16 {
        4
    }
    
    fn version(&self) -> i16 {
        6
    }
}

impl Response for LeaderAndIsrResponse { }

impl Default for LeaderAndIsrResponse {
    fn default() -> Self {
        LeaderAndIsrResponse {
            error_code: 0_i16,
            topics: Vec::<LeaderAndIsrTopicError>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderAndIsrResponse {
    pub fn new(error_code: i16, topics: Vec<LeaderAndIsrTopicError>) -> Self {
        Self {
            error_code,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrResponse::new(
            0_i16,
            Vec::<LeaderAndIsrTopicError>::new(),
        );
        assert_eq!(d, LeaderAndIsrResponse::default());
    }
}

impl Readable for LeaderAndIsrResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let topics = read_array::<LeaderAndIsrTopicError>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderAndIsrResponse {
            error_code, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderAndIsrResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// LeaderAndIsrTopicError, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrTopicError {
    /// The unique topic ID
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// Each partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_errors: Vec<LeaderAndIsrPartitionError>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for LeaderAndIsrTopicError {
    fn default() -> Self {
        LeaderAndIsrTopicError {
            topic_id: Uuid::nil(),
            partition_errors: Vec::<LeaderAndIsrPartitionError>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderAndIsrTopicError {
    pub fn new(topic_id: Uuid, partition_errors: Vec<LeaderAndIsrPartitionError>) -> Self {
        Self {
            topic_id,
            partition_errors,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_topic_error_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrTopicError::new(
            Uuid::nil(),
            Vec::<LeaderAndIsrPartitionError>::new(),
        );
        assert_eq!(d, LeaderAndIsrTopicError::default());
    }
}

impl Readable for LeaderAndIsrTopicError {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partition_errors = read_array::<LeaderAndIsrPartitionError>(input, "partition_errors", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderAndIsrTopicError {
            topic_id, partition_errors, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderAndIsrTopicError {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partition_errors", &self.partition_errors, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// LeaderAndIsrPartitionError, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrPartitionError {
    /// The partition index.
    pub partition_index: i32,
    /// The partition error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for LeaderAndIsrPartitionError {
    fn default() -> Self {
        LeaderAndIsrPartitionError {
            partition_index: 0_i32,
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderAndIsrPartitionError {
    pub fn new(partition_index: i32, error_code: i16) -> Self {
        Self {
            partition_index,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_partition_error_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrPartitionError::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, LeaderAndIsrPartitionError::default());
    }
}

impl Readable for LeaderAndIsrPartitionError {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderAndIsrPartitionError {
            partition_index, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderAndIsrPartitionError {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
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
        crate::test_utils::test_java_default::<LeaderAndIsrResponse>("LeaderAndIsrResponse", 6);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: LeaderAndIsrResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: LeaderAndIsrResponse) {
            crate::test_utils::test_java_arbitrary(&data, "LeaderAndIsrResponse", 6);
        }
    }
}
