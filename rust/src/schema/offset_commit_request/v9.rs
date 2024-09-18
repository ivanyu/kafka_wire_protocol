// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetCommitRequest, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitRequest {
    /// The unique group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The generation of the group if using the classic group protocol or the member epoch if using the consumer protocol.
    pub generation_id_or_member_epoch: i32,
    /// The member ID assigned by the group coordinator.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The unique identifier of the consumer instance provided by end user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// The topics to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetCommitRequestTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OffsetCommitRequest {
    fn api_key(&self) -> i16 {
        8
    }
    
    fn version(&self) -> i16 {
        9
    }
}

impl Request for OffsetCommitRequest { }

impl Default for OffsetCommitRequest {
    fn default() -> Self {
        OffsetCommitRequest {
            group_id: String::from(""),
            generation_id_or_member_epoch: -1_i32,
            member_id: String::from(""),
            group_instance_id: None,
            topics: Vec::<OffsetCommitRequestTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetCommitRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(group_id: S1, generation_id_or_member_epoch: i32, member_id: S2, group_instance_id: Option<S3>, topics: Vec<OffsetCommitRequestTopic>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            generation_id_or_member_epoch,
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_commit_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetCommitRequest::new(
            String::from(""),
            -1_i32,
            String::from(""),
            None::<String>,
            Vec::<OffsetCommitRequestTopic>::new(),
        );
        assert_eq!(d, OffsetCommitRequest::default());
    }
}

impl Readable for OffsetCommitRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let generation_id_or_member_epoch = i32::read(input)?;
        let member_id = String::read_ext(input, "member_id", true)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", true)?;
        let topics = read_array::<OffsetCommitRequestTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetCommitRequest {
            group_id, generation_id_or_member_epoch, member_id, group_instance_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetCommitRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.generation_id_or_member_epoch.write(output)?;
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetCommitRequestTopic, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetCommitRequestPartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetCommitRequestTopic {
    fn default() -> Self {
        OffsetCommitRequestTopic {
            name: String::from(""),
            partitions: Vec::<OffsetCommitRequestPartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetCommitRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OffsetCommitRequestPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_commit_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetCommitRequestTopic::new(
            String::from(""),
            Vec::<OffsetCommitRequestPartition>::new(),
        );
        assert_eq!(d, OffsetCommitRequestTopic::default());
    }
}

impl Readable for OffsetCommitRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<OffsetCommitRequestPartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetCommitRequestTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetCommitRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetCommitRequestPartition, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitRequestPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The message offset to be committed.
    pub committed_offset: i64,
    /// The leader epoch of this partition.
    pub committed_leader_epoch: i32,
    /// Any associated metadata the client wants to keep.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub committed_metadata: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetCommitRequestPartition {
    fn default() -> Self {
        OffsetCommitRequestPartition {
            partition_index: 0_i32,
            committed_offset: 0_i64,
            committed_leader_epoch: -1_i32,
            committed_metadata: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetCommitRequestPartition {
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
mod tests_offset_commit_request_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetCommitRequestPartition::new(
            0_i32,
            0_i64,
            -1_i32,
            Some(String::from("")),
        );
        assert_eq!(d, OffsetCommitRequestPartition::default());
    }
}

impl Readable for OffsetCommitRequestPartition {
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
        Ok(OffsetCommitRequestPartition {
            partition_index, committed_offset, committed_leader_epoch, committed_metadata, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetCommitRequestPartition {
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
        crate::test_utils::test_java_default::<OffsetCommitRequest>("OffsetCommitRequest", 9);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetCommitRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetCommitRequest) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetCommitRequest", 9);
        }
    }
}
