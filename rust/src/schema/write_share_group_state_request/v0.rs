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

/// WriteShareGroupStateRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WriteShareGroupStateRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The data for the topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<WriteStateData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for WriteShareGroupStateRequest {
    fn api_key(&self) -> i16 {
        85
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for WriteShareGroupStateRequest { }

impl Default for WriteShareGroupStateRequest {
    fn default() -> Self {
        WriteShareGroupStateRequest {
            group_id: String::from(""),
            topics: Vec::<WriteStateData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl WriteShareGroupStateRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<WriteStateData>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_write_share_group_state_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WriteShareGroupStateRequest::new(
            String::from(""),
            Vec::<WriteStateData>::new(),
        );
        assert_eq!(d, WriteShareGroupStateRequest::default());
    }
}

impl Readable for WriteShareGroupStateRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_array::<WriteStateData>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(WriteShareGroupStateRequest {
            group_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for WriteShareGroupStateRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// WriteStateData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WriteStateData {
    /// The topic identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The data for the partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for WriteStateData {
    fn default() -> Self {
        WriteStateData {
            topic_id: Uuid::nil(),
            partitions: Vec::<PartitionData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl WriteStateData {
    pub fn new(topic_id: Uuid, partitions: Vec<PartitionData>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_write_state_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WriteStateData::new(
            Uuid::nil(),
            Vec::<PartitionData>::new(),
        );
        assert_eq!(d, WriteStateData::default());
    }
}

impl Readable for WriteStateData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<PartitionData>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(WriteStateData {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for WriteStateData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
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
    pub partition: i32,
    /// The state epoch for this share-partition.
    pub state_epoch: i32,
    /// The leader epoch of the share-partition.
    pub leader_epoch: i32,
    /// The share-partition start offset, or -1 if the start offset is not being written.
    pub start_offset: i64,
    /// The state batches for the share-partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub state_batches: Vec<StateBatch>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition: 0_i32,
            state_epoch: 0_i32,
            leader_epoch: 0_i32,
            start_offset: 0_i64,
            state_batches: Vec::<StateBatch>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new(partition: i32, state_epoch: i32, leader_epoch: i32, start_offset: i64, state_batches: Vec<StateBatch>) -> Self {
        Self {
            partition,
            state_epoch,
            leader_epoch,
            start_offset,
            state_batches,
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
            0_i32,
            0_i32,
            0_i64,
            Vec::<StateBatch>::new(),
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let state_epoch = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let start_offset = i64::read(input)?;
        let state_batches = read_array::<StateBatch>(input, "state_batches", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition, state_epoch, leader_epoch, start_offset, state_batches, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.state_epoch.write(output)?;
        self.leader_epoch.write(output)?;
        self.start_offset.write(output)?;
        write_array(output, "self.state_batches", &self.state_batches, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// StateBatch, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StateBatch {
    /// The base offset of this state batch.
    pub first_offset: i64,
    /// The last offset of this state batch.
    pub last_offset: i64,
    /// The state - 0:Available,2:Acked,4:Archived.
    pub delivery_state: i8,
    /// The delivery count.
    pub delivery_count: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for StateBatch {
    fn default() -> Self {
        StateBatch {
            first_offset: 0_i64,
            last_offset: 0_i64,
            delivery_state: 0_i8,
            delivery_count: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl StateBatch {
    pub fn new(first_offset: i64, last_offset: i64, delivery_state: i8, delivery_count: i16) -> Self {
        Self {
            first_offset,
            last_offset,
            delivery_state,
            delivery_count,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_state_batch_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StateBatch::new(
            0_i64,
            0_i64,
            0_i8,
            0_i16,
        );
        assert_eq!(d, StateBatch::default());
    }
}

impl Readable for StateBatch {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let first_offset = i64::read(input)?;
        let last_offset = i64::read(input)?;
        let delivery_state = i8::read(input)?;
        let delivery_count = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(StateBatch {
            first_offset, last_offset, delivery_state, delivery_count, _unknown_tagged_fields
        })
    }
}

impl Writable for StateBatch {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.first_offset.write(output)?;
        self.last_offset.write(output)?;
        self.delivery_state.write(output)?;
        self.delivery_count.write(output)?;
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
        crate::test_utils::test_java_default::<WriteShareGroupStateRequest>("WriteShareGroupStateRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: WriteShareGroupStateRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: WriteShareGroupStateRequest) {
            crate::test_utils::test_java_arbitrary(&data, "WriteShareGroupStateRequest", 0);
        }
    }
}
