// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// TxnOffsetCommitRequest, version 0.
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
    /// Each topic that we want to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TxnOffsetCommitRequestTopic>,
}

impl ApiMessage for TxnOffsetCommitRequest {
    fn api_key(&self) -> i16 {
        28
    }
    
    fn version(&self) -> i16 {
        0
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
            topics: Vec::<TxnOffsetCommitRequestTopic>::new(),
        }
    }
}

impl TxnOffsetCommitRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(transactional_id: S1, group_id: S2, producer_id: i64, producer_epoch: i16, topics: Vec<TxnOffsetCommitRequestTopic>) -> Self {
        Self {
            transactional_id: transactional_id.as_ref().to_string(),
            group_id: group_id.as_ref().to_string(),
            producer_id,
            producer_epoch,
            topics,
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
            Vec::<TxnOffsetCommitRequestTopic>::new(),
        );
        assert_eq!(d, TxnOffsetCommitRequest::default());
    }
}

impl Readable for TxnOffsetCommitRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = String::read_ext(input, "transactional_id", false)?;
        let group_id = String::read_ext(input, "group_id", false)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let topics = read_array::<TxnOffsetCommitRequestTopic>(input, "topics", false)?;
        Ok(TxnOffsetCommitRequest {
            transactional_id, group_id, producer_id, producer_epoch, topics
        })
    }
}

impl Writable for TxnOffsetCommitRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", false)?;
        self.group_id.write_ext(output, "self.group_id", false)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// TxnOffsetCommitRequestTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partitions inside the topic that we want to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<TxnOffsetCommitRequestPartition>,
}

impl Default for TxnOffsetCommitRequestTopic {
    fn default() -> Self {
        TxnOffsetCommitRequestTopic {
            name: String::from(""),
            partitions: Vec::<TxnOffsetCommitRequestPartition>::new(),
        }
    }
}

impl TxnOffsetCommitRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<TxnOffsetCommitRequestPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
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
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<TxnOffsetCommitRequestPartition>(input, "partitions", false)?;
        Ok(TxnOffsetCommitRequestTopic {
            name, partitions
        })
    }
}

impl Writable for TxnOffsetCommitRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// TxnOffsetCommitRequestPartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitRequestPartition {
    /// The index of the partition within the topic.
    pub partition_index: i32,
    /// The message offset to be committed.
    pub committed_offset: i64,
    /// Any associated metadata the client wants to keep.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub committed_metadata: Option<String>,
}

impl Default for TxnOffsetCommitRequestPartition {
    fn default() -> Self {
        TxnOffsetCommitRequestPartition {
            partition_index: 0_i32,
            committed_offset: 0_i64,
            committed_metadata: Some(String::from("")),
        }
    }
}

impl TxnOffsetCommitRequestPartition {
    pub fn new<S1: AsRef<str>>(partition_index: i32, committed_offset: i64, committed_metadata: Option<S1>) -> Self {
        Self {
            partition_index,
            committed_offset,
            committed_metadata: committed_metadata.map(|s| s.as_ref().to_string()),
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
            Some(String::from("")),
        );
        assert_eq!(d, TxnOffsetCommitRequestPartition::default());
    }
}

impl Readable for TxnOffsetCommitRequestPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let committed_offset = i64::read(input)?;
        let committed_metadata = Option::<String>::read_ext(input, "committed_metadata", false)?;
        Ok(TxnOffsetCommitRequestPartition {
            partition_index, committed_offset, committed_metadata
        })
    }
}

impl Writable for TxnOffsetCommitRequestPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.committed_offset.write(output)?;
        self.committed_metadata.write_ext(output, "self.committed_metadata", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<TxnOffsetCommitRequest>("TxnOffsetCommitRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "TxnOffsetCommitRequest", 0);
        }
    }
}
