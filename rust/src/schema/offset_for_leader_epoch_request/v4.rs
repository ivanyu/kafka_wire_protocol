// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetForLeaderEpochRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    pub replica_id: i32,
    /// Each topic to get offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetForLeaderTopic>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OffsetForLeaderEpochRequest {
    fn api_key(&self) -> i16 {
        23
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for OffsetForLeaderEpochRequest { }

impl Default for OffsetForLeaderEpochRequest {
    fn default() -> Self {
        OffsetForLeaderEpochRequest {
            replica_id: -2_i32,
            topics: Vec::<OffsetForLeaderTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetForLeaderEpochRequest {
    pub fn new(replica_id: i32, topics: Vec<OffsetForLeaderTopic>) -> Self {
        Self {
            replica_id,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_for_leader_epoch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetForLeaderEpochRequest::new(
            -2_i32,
            Vec::<OffsetForLeaderTopic>::new(),
        );
        assert_eq!(d, OffsetForLeaderEpochRequest::default());
    }
}

impl Readable for OffsetForLeaderEpochRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let replica_id = i32::read(input)?;
        let topics = read_array::<OffsetForLeaderTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetForLeaderEpochRequest {
            replica_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetForLeaderEpochRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetForLeaderTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// Each partition to get offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetForLeaderPartition>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OffsetForLeaderTopic {
    fn api_key(&self) -> i16 {
        23
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for OffsetForLeaderTopic { }

impl Default for OffsetForLeaderTopic {
    fn default() -> Self {
        OffsetForLeaderTopic {
            topic: String::from(""),
            partitions: Vec::<OffsetForLeaderPartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetForLeaderTopic {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<OffsetForLeaderPartition>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_for_leader_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetForLeaderTopic::new(
            String::from(""),
            Vec::<OffsetForLeaderPartition>::new(),
        );
        assert_eq!(d, OffsetForLeaderTopic::default());
    }
}

impl Readable for OffsetForLeaderTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", true)?;
        let partitions = read_array::<OffsetForLeaderPartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetForLeaderTopic {
            topic, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetForLeaderTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    pub partition: i32,
    /// An epoch used to fence consumers/replicas with old metadata. If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
    pub current_leader_epoch: i32,
    /// The epoch to look up an offset for.
    pub leader_epoch: i32,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OffsetForLeaderPartition {
    fn api_key(&self) -> i16 {
        23
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for OffsetForLeaderPartition { }

impl Default for OffsetForLeaderPartition {
    fn default() -> Self {
        OffsetForLeaderPartition {
            partition: 0_i32,
            current_leader_epoch: -1_i32,
            leader_epoch: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetForLeaderPartition {
    pub fn new(partition: i32, current_leader_epoch: i32, leader_epoch: i32) -> Self {
        Self {
            partition,
            current_leader_epoch,
            leader_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_for_leader_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetForLeaderPartition::new(
            0_i32,
            -1_i32,
            0_i32,
        );
        assert_eq!(d, OffsetForLeaderPartition::default());
    }
}

impl Readable for OffsetForLeaderPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let current_leader_epoch = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetForLeaderPartition {
            partition, current_leader_epoch, leader_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetForLeaderPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.current_leader_epoch.write(output)?;
        self.leader_epoch.write(output)?;
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
        crate::test_utils::test_java_default::<OffsetForLeaderEpochRequest>("OffsetForLeaderEpochRequest", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetForLeaderEpochRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetForLeaderEpochRequest) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetForLeaderEpochRequest", 4);
        }
    }
}
