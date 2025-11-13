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

/// DescribeShareGroupOffsetsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeShareGroupOffsetsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<DescribeShareGroupOffsetsResponseGroup>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeShareGroupOffsetsResponse {
    fn api_key(&self) -> i16 {
        90
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DescribeShareGroupOffsetsResponse { }

impl Default for DescribeShareGroupOffsetsResponse {
    fn default() -> Self {
        DescribeShareGroupOffsetsResponse {
            throttle_time_ms: 0_i32,
            groups: Vec::<DescribeShareGroupOffsetsResponseGroup>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeShareGroupOffsetsResponse {
    pub fn new(throttle_time_ms: i32, groups: Vec<DescribeShareGroupOffsetsResponseGroup>) -> Self {
        Self {
            throttle_time_ms,
            groups,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_share_group_offsets_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeShareGroupOffsetsResponse::new(
            0_i32,
            Vec::<DescribeShareGroupOffsetsResponseGroup>::new(),
        );
        assert_eq!(d, DescribeShareGroupOffsetsResponse::default());
    }
}

impl Readable for DescribeShareGroupOffsetsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let groups = read_array::<DescribeShareGroupOffsetsResponseGroup>(input, "groups", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeShareGroupOffsetsResponse {
            throttle_time_ms, groups, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeShareGroupOffsetsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.groups", &self.groups, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeShareGroupOffsetsResponseGroup, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeShareGroupOffsetsResponseGroup {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The results for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<DescribeShareGroupOffsetsResponseTopic>,
    /// The group-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The group-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeShareGroupOffsetsResponseGroup {
    fn default() -> Self {
        DescribeShareGroupOffsetsResponseGroup {
            group_id: String::from(""),
            topics: Vec::<DescribeShareGroupOffsetsResponseTopic>::new(),
            error_code: 0_i16,
            error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeShareGroupOffsetsResponseGroup {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(group_id: S1, topics: Vec<DescribeShareGroupOffsetsResponseTopic>, error_code: i16, error_message: Option<S2>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_share_group_offsets_response_group_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeShareGroupOffsetsResponseGroup::new(
            String::from(""),
            Vec::<DescribeShareGroupOffsetsResponseTopic>::new(),
            0_i16,
            None::<String>,
        );
        assert_eq!(d, DescribeShareGroupOffsetsResponseGroup::default());
    }
}

impl Readable for DescribeShareGroupOffsetsResponseGroup {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_array::<DescribeShareGroupOffsetsResponseTopic>(input, "topics", true)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeShareGroupOffsetsResponseGroup {
            group_id, topics, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeShareGroupOffsetsResponseGroup {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeShareGroupOffsetsResponseTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeShareGroupOffsetsResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<DescribeShareGroupOffsetsResponsePartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeShareGroupOffsetsResponseTopic {
    fn default() -> Self {
        DescribeShareGroupOffsetsResponseTopic {
            topic_name: String::from(""),
            topic_id: Uuid::nil(),
            partitions: Vec::<DescribeShareGroupOffsetsResponsePartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeShareGroupOffsetsResponseTopic {
    pub fn new<S1: AsRef<str>>(topic_name: S1, topic_id: Uuid, partitions: Vec<DescribeShareGroupOffsetsResponsePartition>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_share_group_offsets_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeShareGroupOffsetsResponseTopic::new(
            String::from(""),
            Uuid::nil(),
            Vec::<DescribeShareGroupOffsetsResponsePartition>::new(),
        );
        assert_eq!(d, DescribeShareGroupOffsetsResponseTopic::default());
    }
}

impl Readable for DescribeShareGroupOffsetsResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<DescribeShareGroupOffsetsResponsePartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeShareGroupOffsetsResponseTopic {
            topic_name, topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeShareGroupOffsetsResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeShareGroupOffsetsResponsePartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeShareGroupOffsetsResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The share-partition start offset.
    pub start_offset: i64,
    /// The leader epoch of the partition.
    pub leader_epoch: i32,
    /// The partition-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The partition-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeShareGroupOffsetsResponsePartition {
    fn default() -> Self {
        DescribeShareGroupOffsetsResponsePartition {
            partition_index: 0_i32,
            start_offset: 0_i64,
            leader_epoch: 0_i32,
            error_code: 0_i16,
            error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeShareGroupOffsetsResponsePartition {
    pub fn new<S1: AsRef<str>>(partition_index: i32, start_offset: i64, leader_epoch: i32, error_code: i16, error_message: Option<S1>) -> Self {
        Self {
            partition_index,
            start_offset,
            leader_epoch,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_share_group_offsets_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeShareGroupOffsetsResponsePartition::new(
            0_i32,
            0_i64,
            0_i32,
            0_i16,
            None::<String>,
        );
        assert_eq!(d, DescribeShareGroupOffsetsResponsePartition::default());
    }
}

impl Readable for DescribeShareGroupOffsetsResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let start_offset = i64::read(input)?;
        let leader_epoch = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeShareGroupOffsetsResponsePartition {
            partition_index, start_offset, leader_epoch, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeShareGroupOffsetsResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.start_offset.write(output)?;
        self.leader_epoch.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeShareGroupOffsetsResponse>("DescribeShareGroupOffsetsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeShareGroupOffsetsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeShareGroupOffsetsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeShareGroupOffsetsResponse", 0);
        }
    }
}
