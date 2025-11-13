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

/// ShareAcknowledgeRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ShareAcknowledgeRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_id: Option<String>,
    /// The member ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub member_id: Option<String>,
    /// The current share session epoch: 0 to open a share session; -1 to close it; otherwise increments for consecutive requests.
    pub share_session_epoch: i32,
    /// The topics containing records to acknowledge.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<AcknowledgeTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ShareAcknowledgeRequest {
    fn api_key(&self) -> i16 {
        79
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for ShareAcknowledgeRequest { }

impl Default for ShareAcknowledgeRequest {
    fn default() -> Self {
        ShareAcknowledgeRequest {
            group_id: None,
            member_id: Some(String::from("")),
            share_session_epoch: 0_i32,
            topics: Vec::<AcknowledgeTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ShareAcknowledgeRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(group_id: Option<S1>, member_id: Option<S2>, share_session_epoch: i32, topics: Vec<AcknowledgeTopic>) -> Self {
        Self {
            group_id: group_id.map(|s| s.as_ref().to_string()),
            member_id: member_id.map(|s| s.as_ref().to_string()),
            share_session_epoch,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_share_acknowledge_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ShareAcknowledgeRequest::new(
            None::<String>,
            Some(String::from("")),
            0_i32,
            Vec::<AcknowledgeTopic>::new(),
        );
        assert_eq!(d, ShareAcknowledgeRequest::default());
    }
}

impl Readable for ShareAcknowledgeRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = Option::<String>::read_ext(input, "group_id", true)?;
        let member_id = Option::<String>::read_ext(input, "member_id", true)?;
        let share_session_epoch = i32::read(input)?;
        let topics = read_array::<AcknowledgeTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ShareAcknowledgeRequest {
            group_id, member_id, share_session_epoch, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for ShareAcknowledgeRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.share_session_epoch.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AcknowledgeTopic, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AcknowledgeTopic {
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The partitions containing records to acknowledge.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<AcknowledgePartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AcknowledgeTopic {
    fn default() -> Self {
        AcknowledgeTopic {
            topic_id: Uuid::nil(),
            partitions: Vec::<AcknowledgePartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AcknowledgeTopic {
    pub fn new(topic_id: Uuid, partitions: Vec<AcknowledgePartition>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_acknowledge_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AcknowledgeTopic::new(
            Uuid::nil(),
            Vec::<AcknowledgePartition>::new(),
        );
        assert_eq!(d, AcknowledgeTopic::default());
    }
}

impl Readable for AcknowledgeTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<AcknowledgePartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AcknowledgeTopic {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for AcknowledgeTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AcknowledgePartition, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AcknowledgePartition {
    /// The partition index.
    pub partition_index: i32,
    /// Record batches to acknowledge.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub acknowledgement_batches: Vec<AcknowledgementBatch>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AcknowledgePartition {
    fn default() -> Self {
        AcknowledgePartition {
            partition_index: 0_i32,
            acknowledgement_batches: Vec::<AcknowledgementBatch>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AcknowledgePartition {
    pub fn new(partition_index: i32, acknowledgement_batches: Vec<AcknowledgementBatch>) -> Self {
        Self {
            partition_index,
            acknowledgement_batches,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_acknowledge_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AcknowledgePartition::new(
            0_i32,
            Vec::<AcknowledgementBatch>::new(),
        );
        assert_eq!(d, AcknowledgePartition::default());
    }
}

impl Readable for AcknowledgePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let acknowledgement_batches = read_array::<AcknowledgementBatch>(input, "acknowledgement_batches", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AcknowledgePartition {
            partition_index, acknowledgement_batches, _unknown_tagged_fields
        })
    }
}

impl Writable for AcknowledgePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        write_array(output, "self.acknowledgement_batches", &self.acknowledgement_batches, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AcknowledgementBatch, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AcknowledgementBatch {
    /// First offset of batch of records to acknowledge.
    pub first_offset: i64,
    /// Last offset (inclusive) of batch of records to acknowledge.
    pub last_offset: i64,
    /// Array of acknowledge types - 0:Gap,1:Accept,2:Release,3:Reject.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub acknowledge_types: Vec<i8>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AcknowledgementBatch {
    fn default() -> Self {
        AcknowledgementBatch {
            first_offset: 0_i64,
            last_offset: 0_i64,
            acknowledge_types: Vec::<i8>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AcknowledgementBatch {
    pub fn new(first_offset: i64, last_offset: i64, acknowledge_types: Vec<i8>) -> Self {
        Self {
            first_offset,
            last_offset,
            acknowledge_types,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_acknowledgement_batch_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AcknowledgementBatch::new(
            0_i64,
            0_i64,
            Vec::<i8>::new(),
        );
        assert_eq!(d, AcknowledgementBatch::default());
    }
}

impl Readable for AcknowledgementBatch {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let first_offset = i64::read(input)?;
        let last_offset = i64::read(input)?;
        let acknowledge_types = read_array::<i8>(input, "acknowledge_types", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AcknowledgementBatch {
            first_offset, last_offset, acknowledge_types, _unknown_tagged_fields
        })
    }
}

impl Writable for AcknowledgementBatch {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.first_offset.write(output)?;
        self.last_offset.write(output)?;
        write_array(output, "self.acknowledge_types", &self.acknowledge_types, true)?;
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
        crate::test_utils::test_java_default::<ShareAcknowledgeRequest>("ShareAcknowledgeRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ShareAcknowledgeRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ShareAcknowledgeRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ShareAcknowledgeRequest", 1);
        }
    }
}
