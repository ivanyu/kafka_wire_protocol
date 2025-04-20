// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// TxnOffsetCommitRequest, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitRequest {
    /// The ID of the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transactional_id: String,
    /// The ID of the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,
    /// The current epoch associated with the producer ID.
    pub producer_epoch: i16,
    /// The generation of the consumer.
    pub generation_id: i32,
    /// The member ID assigned by the group coordinator.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The unique identifier of the consumer instance provided by end user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// Each topic that we want to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for TxnOffsetCommitRequest {
    fn api_key(&self) -> i16 {
        28
    }
    
    fn version(&self) -> i16 {
        5
    }
}

impl Request for TxnOffsetCommitRequest { }

impl Default for TxnOffsetCommitRequest {
    fn default() -> Self {
        TxnOffsetCommitRequest {
            transactional_id: String::from(""),
            group_id: String::from(""),
            producer_id: 0_i64,
            producer_epoch: 0_i16,
            generation_id: -1_i32,
            member_id: String::from(""),
            group_instance_id: None,
            topics: Vec::<TxnOffsetCommitRequestTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TxnOffsetCommitRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>>(transactional_id: S1, group_id: S2, producer_id: i64, producer_epoch: i16, generation_id: i32, member_id: S3, group_instance_id: Option<S4>, topics: Vec<TxnOffsetCommitRequestTopic>) -> Self {
        Self {
            transactional_id: transactional_id.as_ref().to_string(),
            group_id: group_id.as_ref().to_string(),
            producer_id,
            producer_epoch,
            generation_id,
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_txn_offset_commit_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TxnOffsetCommitRequest::new(
            String::from(""),
            String::from(""),
            0_i64,
            0_i16,
            -1_i32,
            String::from(""),
            None::<String>,
            Vec::<TxnOffsetCommitRequestTopic>::new(),
        );
        assert_eq!(d, TxnOffsetCommitRequest::default());
    }
}

impl Readable for TxnOffsetCommitRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = String::read_ext(input, "transactional_id", true)?;
        let group_id = String::read_ext(input, "group_id", true)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let generation_id = i32::read(input)?;
        let member_id = String::read_ext(input, "member_id", true)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", true)?;
        let topics = read_array::<TxnOffsetCommitRequestTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TxnOffsetCommitRequest {
            transactional_id, group_id, producer_id, producer_epoch, generation_id, member_id, group_instance_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for TxnOffsetCommitRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", true)?;
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        self.generation_id.write(output)?;
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TxnOffsetCommitRequestTopic, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partitions inside the topic that we want to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TxnOffsetCommitRequestTopic {
    fn default() -> Self {
        TxnOffsetCommitRequestTopic {
            name: String::from(""),
            partitions: Vec::<TxnOffsetCommitRequestPartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TxnOffsetCommitRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<TxnOffsetCommitRequestPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_txn_offset_commit_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TxnOffsetCommitRequestTopic::new(
            String::from(""),
            Vec::<TxnOffsetCommitRequestPartition>::new(),
        );
        assert_eq!(d, TxnOffsetCommitRequestTopic::default());
    }
}

impl Readable for TxnOffsetCommitRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<TxnOffsetCommitRequestPartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TxnOffsetCommitRequestTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TxnOffsetCommitRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TxnOffsetCommitRequestPartition, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitRequestPartition {
    /// The index of the partition within the topic.
    pub partition_index: i32,
    /// The message offset to be committed.
    pub committed_offset: i64,
    /// The leader epoch of the last consumed record.
    pub committed_leader_epoch: i32,
    /// Any associated metadata the client wants to keep.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub committed_metadata: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TxnOffsetCommitRequestPartition {
    fn default() -> Self {
        TxnOffsetCommitRequestPartition {
            partition_index: 0_i32,
            committed_offset: 0_i64,
            committed_leader_epoch: -1_i32,
            committed_metadata: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TxnOffsetCommitRequestPartition {
    pub fn new<S1: AsRef<str>>(partition_index: i32, committed_offset: i64, committed_leader_epoch: i32, committed_metadata: Option<S1>) -> Self {
        Self {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            committed_metadata: committed_metadata.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_txn_offset_commit_request_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TxnOffsetCommitRequestPartition::new(
            0_i32,
            0_i64,
            -1_i32,
            Some(String::from("")),
        );
        assert_eq!(d, TxnOffsetCommitRequestPartition::default());
    }
}

impl Readable for TxnOffsetCommitRequestPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let committed_offset = i64::read(input)?;
        let committed_leader_epoch = i32::read(input)?;
        let committed_metadata = Option::<String>::read_ext(input, "committed_metadata", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TxnOffsetCommitRequestPartition {
            partition_index, committed_offset, committed_leader_epoch, committed_metadata, _unknown_tagged_fields
        })
    }
}

impl Writable for TxnOffsetCommitRequestPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.committed_offset.write(output)?;
        self.committed_leader_epoch.write(output)?;
        self.committed_metadata.write_ext(output, "self.committed_metadata", true)?;
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
        crate::test_utils::test_java_default::<TxnOffsetCommitRequest>("TxnOffsetCommitRequest", 5);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: TxnOffsetCommitRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: TxnOffsetCommitRequest) {
            crate::test_utils::test_java_arbitrary(&data, "TxnOffsetCommitRequest", 5);
        }
    }
}
