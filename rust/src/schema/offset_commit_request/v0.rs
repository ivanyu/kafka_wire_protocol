// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetCommitRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitRequest {
    /// The unique group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The topics to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetCommitRequestTopic>,
}

impl ApiMessage for OffsetCommitRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        8
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for OffsetCommitRequest { }

impl Default for OffsetCommitRequest {
    fn default() -> Self {
        OffsetCommitRequest {
            group_id: String::from(""),
            topics: Vec::<OffsetCommitRequestTopic>::new(),
        }
    }
}

impl OffsetCommitRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<OffsetCommitRequestTopic>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
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
            Vec::<OffsetCommitRequestTopic>::new(),
        );
        assert_eq!(d, OffsetCommitRequest::default());
    }
}

impl Readable for OffsetCommitRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", false)?;
        let topics = read_array::<OffsetCommitRequestTopic>(input, "topics", false)?;
        Ok(OffsetCommitRequest {
            group_id, topics
        })
    }
}

impl Writable for OffsetCommitRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", false)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// OffsetCommitRequestTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition to commit offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetCommitRequestPartition>,
}

impl Default for OffsetCommitRequestTopic {
    fn default() -> Self {
        OffsetCommitRequestTopic {
            name: String::from(""),
            partitions: Vec::<OffsetCommitRequestPartition>::new(),
        }
    }
}

impl OffsetCommitRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OffsetCommitRequestPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
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
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<OffsetCommitRequestPartition>(input, "partitions", false)?;
        Ok(OffsetCommitRequestTopic {
            name, partitions
        })
    }
}

impl Writable for OffsetCommitRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// OffsetCommitRequestPartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitRequestPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The message offset to be committed.
    pub committed_offset: i64,
    /// Any associated metadata the client wants to keep.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub committed_metadata: Option<String>,
}

impl Default for OffsetCommitRequestPartition {
    fn default() -> Self {
        OffsetCommitRequestPartition {
            partition_index: 0_i32,
            committed_offset: 0_i64,
            committed_metadata: Some(String::from("")),
        }
    }
}

impl OffsetCommitRequestPartition {
    pub fn new<S1: AsRef<str>>(partition_index: i32, committed_offset: i64, committed_metadata: Option<S1>) -> Self {
        Self {
            partition_index,
            committed_offset,
            committed_metadata: committed_metadata.map(|s| s.as_ref().to_string()),
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
            Some(String::from("")),
        );
        assert_eq!(d, OffsetCommitRequestPartition::default());
    }
}

impl Readable for OffsetCommitRequestPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let committed_offset = i64::read(input)?;
        let committed_metadata = Option::<String>::read_ext(input, "committed_metadata", false)?;
        Ok(OffsetCommitRequestPartition {
            partition_index, committed_offset, committed_metadata
        })
    }
}

impl Writable for OffsetCommitRequestPartition {
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
        crate::test_utils::test_java_default::<OffsetCommitRequest>("OffsetCommitRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "OffsetCommitRequest", 0);
        }
    }
}
