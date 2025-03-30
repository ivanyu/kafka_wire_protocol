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

/// VoteRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct VoteRequest {
    /// The cluster id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub cluster_id: Option<String>,
    /// The replica id of the voter receiving the request.
    pub voter_id: i32,
    /// The topic data.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for VoteRequest {
    fn api_key(&self) -> i16 {
        52
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for VoteRequest { }

impl Default for VoteRequest {
    fn default() -> Self {
        VoteRequest {
            cluster_id: None,
            voter_id: -1_i32,
            topics: Vec::<TopicData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl VoteRequest {
    pub fn new<S1: AsRef<str>>(cluster_id: Option<S1>, voter_id: i32, topics: Vec<TopicData>) -> Self {
        Self {
            cluster_id: cluster_id.map(|s| s.as_ref().to_string()),
            voter_id,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_vote_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = VoteRequest::new(
            None::<String>,
            -1_i32,
            Vec::<TopicData>::new(),
        );
        assert_eq!(d, VoteRequest::default());
    }
}

impl Readable for VoteRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let cluster_id = Option::<String>::read_ext(input, "cluster_id", true)?;
        let voter_id = i32::read(input)?;
        let topics = read_array::<TopicData>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(VoteRequest {
            cluster_id, voter_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for VoteRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
        self.voter_id.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicData, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicData {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partition data.
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

/// PartitionData, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// The epoch of the voter sending the request
    pub replica_epoch: i32,
    /// The replica id of the voter sending the request
    pub replica_id: i32,
    /// The directory id of the voter sending the request
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub replica_directory_id: Uuid,
    /// The directory id of the voter receiving the request
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub voter_directory_id: Uuid,
    /// The epoch of the last record written to the metadata log.
    pub last_offset_epoch: i32,
    /// The log end offset of the metadata log of the voter sending the request.
    pub last_offset: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            replica_epoch: 0_i32,
            replica_id: 0_i32,
            replica_directory_id: Uuid::nil(),
            voter_directory_id: Uuid::nil(),
            last_offset_epoch: 0_i32,
            last_offset: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new(partition_index: i32, replica_epoch: i32, replica_id: i32, replica_directory_id: Uuid, voter_directory_id: Uuid, last_offset_epoch: i32, last_offset: i64) -> Self {
        Self {
            partition_index,
            replica_epoch,
            replica_id,
            replica_directory_id,
            voter_directory_id,
            last_offset_epoch,
            last_offset,
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
            Uuid::nil(),
            Uuid::nil(),
            0_i32,
            0_i64,
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let replica_epoch = i32::read(input)?;
        let replica_id = i32::read(input)?;
        let replica_directory_id = Uuid::read(input)?;
        let voter_directory_id = Uuid::read(input)?;
        let last_offset_epoch = i32::read(input)?;
        let last_offset = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition_index, replica_epoch, replica_id, replica_directory_id, voter_directory_id, last_offset_epoch, last_offset, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.replica_epoch.write(output)?;
        self.replica_id.write(output)?;
        self.replica_directory_id.write(output)?;
        self.voter_directory_id.write(output)?;
        self.last_offset_epoch.write(output)?;
        self.last_offset.write(output)?;
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
        crate::test_utils::test_java_default::<VoteRequest>("VoteRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: VoteRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: VoteRequest) {
            crate::test_utils::test_java_arbitrary(&data, "VoteRequest", 1);
        }
    }
}
