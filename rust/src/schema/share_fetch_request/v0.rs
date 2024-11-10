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

/// ShareFetchRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ShareFetchRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_id: Option<String>,
    /// The member ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub member_id: Option<String>,
    /// The current share session epoch: 0 to open a share session; -1 to close it; otherwise increments for consecutive requests.
    pub share_session_epoch: i32,
    /// The maximum time in milliseconds to wait for the response.
    pub max_wait_ms: i32,
    /// The minimum bytes to accumulate in the response.
    pub min_bytes: i32,
    /// The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,
    /// The topics to fetch.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<FetchTopic>,
    /// The partitions to remove from this share session.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub forgotten_topics_data: Vec<ForgottenTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ShareFetchRequest {
    fn api_key(&self) -> i16 {
        78
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ShareFetchRequest { }

impl Default for ShareFetchRequest {
    fn default() -> Self {
        ShareFetchRequest {
            group_id: None,
            member_id: Some(String::from("")),
            share_session_epoch: 0_i32,
            max_wait_ms: 0_i32,
            min_bytes: 0_i32,
            max_bytes: 0x7fffffff_i32,
            topics: Vec::<FetchTopic>::new(),
            forgotten_topics_data: Vec::<ForgottenTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ShareFetchRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(group_id: Option<S1>, member_id: Option<S2>, share_session_epoch: i32, max_wait_ms: i32, min_bytes: i32, max_bytes: i32, topics: Vec<FetchTopic>, forgotten_topics_data: Vec<ForgottenTopic>) -> Self {
        Self {
            group_id: group_id.map(|s| s.as_ref().to_string()),
            member_id: member_id.map(|s| s.as_ref().to_string()),
            share_session_epoch,
            max_wait_ms,
            min_bytes,
            max_bytes,
            topics,
            forgotten_topics_data,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_share_fetch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ShareFetchRequest::new(
            None::<String>,
            Some(String::from("")),
            0_i32,
            0_i32,
            0_i32,
            0x7fffffff_i32,
            Vec::<FetchTopic>::new(),
            Vec::<ForgottenTopic>::new(),
        );
        assert_eq!(d, ShareFetchRequest::default());
    }
}

impl Readable for ShareFetchRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = Option::<String>::read_ext(input, "group_id", true)?;
        let member_id = Option::<String>::read_ext(input, "member_id", true)?;
        let share_session_epoch = i32::read(input)?;
        let max_wait_ms = i32::read(input)?;
        let min_bytes = i32::read(input)?;
        let max_bytes = i32::read(input)?;
        let topics = read_array::<FetchTopic>(input, "topics", true)?;
        let forgotten_topics_data = read_array::<ForgottenTopic>(input, "forgotten_topics_data", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ShareFetchRequest {
            group_id, member_id, share_session_epoch, max_wait_ms, min_bytes, max_bytes, topics, forgotten_topics_data, _unknown_tagged_fields
        })
    }
}

impl Writable for ShareFetchRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.share_session_epoch.write(output)?;
        self.max_wait_ms.write(output)?;
        self.min_bytes.write(output)?;
        self.max_bytes.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_array(output, "self.forgotten_topics_data", &self.forgotten_topics_data, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// FetchTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchTopic {
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The partitions to fetch.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<FetchPartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for FetchTopic {
    fn default() -> Self {
        FetchTopic {
            topic_id: Uuid::nil(),
            partitions: Vec::<FetchPartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FetchTopic {
    pub fn new(topic_id: Uuid, partitions: Vec<FetchPartition>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_fetch_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FetchTopic::new(
            Uuid::nil(),
            Vec::<FetchPartition>::new(),
        );
        assert_eq!(d, FetchTopic::default());
    }
}

impl Readable for FetchTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<FetchPartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FetchTopic {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for FetchTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// FetchPartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The maximum bytes to fetch from this partition. 0 when only acknowledgement with no fetching is required. See KIP-74 for cases where this limit may not be honored.
    pub partition_max_bytes: i32,
    /// Record batches to acknowledge.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub acknowledgement_batches: Vec<AcknowledgementBatch>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for FetchPartition {
    fn default() -> Self {
        FetchPartition {
            partition_index: 0_i32,
            partition_max_bytes: 0_i32,
            acknowledgement_batches: Vec::<AcknowledgementBatch>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FetchPartition {
    pub fn new(partition_index: i32, partition_max_bytes: i32, acknowledgement_batches: Vec<AcknowledgementBatch>) -> Self {
        Self {
            partition_index,
            partition_max_bytes,
            acknowledgement_batches,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_fetch_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FetchPartition::new(
            0_i32,
            0_i32,
            Vec::<AcknowledgementBatch>::new(),
        );
        assert_eq!(d, FetchPartition::default());
    }
}

impl Readable for FetchPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let partition_max_bytes = i32::read(input)?;
        let acknowledgement_batches = read_array::<AcknowledgementBatch>(input, "acknowledgement_batches", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FetchPartition {
            partition_index, partition_max_bytes, acknowledgement_batches, _unknown_tagged_fields
        })
    }
}

impl Writable for FetchPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.partition_max_bytes.write(output)?;
        write_array(output, "self.acknowledgement_batches", &self.acknowledgement_batches, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AcknowledgementBatch, version 0.
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

/// ForgottenTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ForgottenTopic {
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The partitions indexes to forget.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ForgottenTopic {
    fn default() -> Self {
        ForgottenTopic {
            topic_id: Uuid::nil(),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ForgottenTopic {
    pub fn new(topic_id: Uuid, partitions: Vec<i32>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_forgotten_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ForgottenTopic::new(
            Uuid::nil(),
            Vec::<i32>::new(),
        );
        assert_eq!(d, ForgottenTopic::default());
    }
}

impl Readable for ForgottenTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ForgottenTopic {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for ForgottenTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
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
        crate::test_utils::test_java_default::<ShareFetchRequest>("ShareFetchRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ShareFetchRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ShareFetchRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ShareFetchRequest", 0);
        }
    }
}
