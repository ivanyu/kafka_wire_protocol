// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeQuorumResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeQuorumResponse {
    /// The top level error code.
    pub error_code: i16,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeQuorumResponse {
    fn api_key(&self) -> i16 {
        55
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DescribeQuorumResponse { }

impl Default for DescribeQuorumResponse {
    fn default() -> Self {
        DescribeQuorumResponse {
            error_code: 0_i16,
            topics: Vec::<TopicData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeQuorumResponse {
    pub fn new(error_code: i16, topics: Vec<TopicData>) -> Self {
        Self {
            error_code,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_quorum_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeQuorumResponse::new(
            0_i16,
            Vec::<TopicData>::new(),
        );
        assert_eq!(d, DescribeQuorumResponse::default());
    }
}

impl Readable for DescribeQuorumResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let topics = read_array::<TopicData>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeQuorumResponse {
            error_code, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeQuorumResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicData {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicData {
    fn default() -> Self {
        TopicData {
            topic_name: String::from(""),
            partitions: Vec::<PartitionData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicData {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partitions: Vec<PartitionData>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicData::new(
            String::from(""),
            Vec::<PartitionData>::new(),
        );
        assert_eq!(d, TopicData::default());
    }
}

impl Readable for TopicData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partitions = read_array::<PartitionData>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicData {
            topic_name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// 
    pub error_code: i16,
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: i32,
    /// The latest known leader epoch
    pub leader_epoch: i32,
    /// 
    pub high_watermark: i64,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub current_voters: Vec<ReplicaState>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub observers: Vec<ReplicaState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            error_code: 0_i16,
            leader_id: 0_i32,
            leader_epoch: 0_i32,
            high_watermark: 0_i64,
            current_voters: Vec::<ReplicaState>::new(),
            observers: Vec::<ReplicaState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new(partition_index: i32, error_code: i16, leader_id: i32, leader_epoch: i32, high_watermark: i64, current_voters: Vec<ReplicaState>, observers: Vec<ReplicaState>) -> Self {
        Self {
            partition_index,
            error_code,
            leader_id,
            leader_epoch,
            high_watermark,
            current_voters,
            observers,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_partition_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionData::new(
            0_i32,
            0_i16,
            0_i32,
            0_i32,
            0_i64,
            Vec::<ReplicaState>::new(),
            Vec::<ReplicaState>::new(),
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let high_watermark = i64::read(input)?;
        let current_voters = read_array::<ReplicaState>(input, "current_voters", true)?;
        let observers = read_array::<ReplicaState>(input, "observers", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition_index, error_code, leader_id, leader_epoch, high_watermark, current_voters, observers, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        self.high_watermark.write(output)?;
        write_array(output, "self.current_voters", &self.current_voters, true)?;
        write_array(output, "self.observers", &self.observers, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ReplicaState, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReplicaState {
    /// 
    pub replica_id: i32,
    /// The last known log end offset of the follower or -1 if it is unknown
    pub log_end_offset: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ReplicaState {
    fn default() -> Self {
        ReplicaState {
            replica_id: 0_i32,
            log_end_offset: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReplicaState {
    pub fn new(replica_id: i32, log_end_offset: i64) -> Self {
        Self {
            replica_id,
            log_end_offset,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_replica_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReplicaState::new(
            0_i32,
            0_i64,
        );
        assert_eq!(d, ReplicaState::default());
    }
}

impl Readable for ReplicaState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let replica_id = i32::read(input)?;
        let log_end_offset = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReplicaState {
            replica_id, log_end_offset, _unknown_tagged_fields
        })
    }
}

impl Writable for ReplicaState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        self.log_end_offset.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeQuorumResponse>("DescribeQuorumResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeQuorumResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeQuorumResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeQuorumResponse", 0);
        }
    }
}
