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

/// AlterShareGroupOffsetsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterShareGroupOffsetsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The results for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<AlterShareGroupOffsetsResponseTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterShareGroupOffsetsResponse {
    fn api_key(&self) -> i16 {
        91
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for AlterShareGroupOffsetsResponse { }

impl Default for AlterShareGroupOffsetsResponse {
    fn default() -> Self {
        AlterShareGroupOffsetsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: None,
            responses: Vec::<AlterShareGroupOffsetsResponseTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterShareGroupOffsetsResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, responses: Vec<AlterShareGroupOffsetsResponseTopic>) -> Self {
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
mod tests_alter_share_group_offsets_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterShareGroupOffsetsResponse::new(
            0_i32,
            0_i16,
            None::<String>,
            Vec::<AlterShareGroupOffsetsResponseTopic>::new(),
        );
        assert_eq!(d, AlterShareGroupOffsetsResponse::default());
    }
}

impl Readable for AlterShareGroupOffsetsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let responses = read_array::<AlterShareGroupOffsetsResponseTopic>(input, "responses", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterShareGroupOffsetsResponse {
            throttle_time_ms, error_code, error_message, responses, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterShareGroupOffsetsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.responses", &self.responses, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AlterShareGroupOffsetsResponseTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterShareGroupOffsetsResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<AlterShareGroupOffsetsResponsePartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AlterShareGroupOffsetsResponseTopic {
    fn default() -> Self {
        AlterShareGroupOffsetsResponseTopic {
            topic_name: String::from(""),
            topic_id: Uuid::nil(),
            partitions: Vec::<AlterShareGroupOffsetsResponsePartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterShareGroupOffsetsResponseTopic {
    pub fn new<S1: AsRef<str>>(topic_name: S1, topic_id: Uuid, partitions: Vec<AlterShareGroupOffsetsResponsePartition>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_share_group_offsets_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterShareGroupOffsetsResponseTopic::new(
            String::from(""),
            Uuid::nil(),
            Vec::<AlterShareGroupOffsetsResponsePartition>::new(),
        );
        assert_eq!(d, AlterShareGroupOffsetsResponseTopic::default());
    }
}

impl Readable for AlterShareGroupOffsetsResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<AlterShareGroupOffsetsResponsePartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterShareGroupOffsetsResponseTopic {
            topic_name, topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterShareGroupOffsetsResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AlterShareGroupOffsetsResponsePartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterShareGroupOffsetsResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AlterShareGroupOffsetsResponsePartition {
    fn default() -> Self {
        AlterShareGroupOffsetsResponsePartition {
            partition_index: 0_i32,
            error_code: 0_i16,
            error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterShareGroupOffsetsResponsePartition {
    pub fn new<S1: AsRef<str>>(partition_index: i32, error_code: i16, error_message: Option<S1>) -> Self {
        Self {
            partition_index,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_share_group_offsets_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterShareGroupOffsetsResponsePartition::new(
            0_i32,
            0_i16,
            None::<String>,
        );
        assert_eq!(d, AlterShareGroupOffsetsResponsePartition::default());
    }
}

impl Readable for AlterShareGroupOffsetsResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterShareGroupOffsetsResponsePartition {
            partition_index, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterShareGroupOffsetsResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
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
        crate::test_utils::test_java_default::<AlterShareGroupOffsetsResponse>("AlterShareGroupOffsetsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterShareGroupOffsetsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterShareGroupOffsetsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AlterShareGroupOffsetsResponse", 0);
        }
    }
}
