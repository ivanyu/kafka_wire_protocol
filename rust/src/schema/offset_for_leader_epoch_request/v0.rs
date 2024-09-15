// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetForLeaderEpochRequest {
    /// Each topic to get offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetForLeaderTopic>,
}

impl ApiMessage for OffsetForLeaderEpochRequest {
    fn api_key(&self) -> i16 {
        23
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for OffsetForLeaderEpochRequest { }

impl Default for OffsetForLeaderEpochRequest {
    fn default() -> Self {
        OffsetForLeaderEpochRequest {
            topics: Vec::<OffsetForLeaderTopic>::new(),
        }
    }
}

impl OffsetForLeaderEpochRequest {
    pub fn new(topics: Vec<OffsetForLeaderTopic>) -> Self {
        Self {
            topics,
        }
    }
}

#[cfg(test)]
mod tests_offset_for_leader_epoch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetForLeaderEpochRequest::new(
            Vec::<OffsetForLeaderTopic>::new(),
        );
        assert_eq!(d, OffsetForLeaderEpochRequest::default());
    }
}

impl Readable for OffsetForLeaderEpochRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<OffsetForLeaderTopic>(input, "topics", false)?;
        Ok(OffsetForLeaderEpochRequest {
            topics
        })
    }
}

impl Writable for OffsetForLeaderEpochRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, false)?;
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
}

impl ApiMessage for OffsetForLeaderTopic {
    fn api_key(&self) -> i16 {
        23
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for OffsetForLeaderTopic { }

impl Default for OffsetForLeaderTopic {
    fn default() -> Self {
        OffsetForLeaderTopic {
            topic: String::from(""),
            partitions: Vec::<OffsetForLeaderPartition>::new(),
        }
    }
}

impl OffsetForLeaderTopic {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<OffsetForLeaderPartition>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
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
        let topic = String::read_ext(input, "topic", false)?;
        let partitions = read_array::<OffsetForLeaderPartition>(input, "partitions", false)?;
        Ok(OffsetForLeaderTopic {
            topic, partitions
        })
    }
}

impl Writable for OffsetForLeaderTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    pub partition: i32,
    /// The epoch to look up an offset for.
    pub leader_epoch: i32,
}

impl ApiMessage for OffsetForLeaderPartition {
    fn api_key(&self) -> i16 {
        23
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for OffsetForLeaderPartition { }

impl Default for OffsetForLeaderPartition {
    fn default() -> Self {
        OffsetForLeaderPartition {
            partition: 0_i32,
            leader_epoch: 0_i32,
        }
    }
}

impl OffsetForLeaderPartition {
    pub fn new(partition: i32, leader_epoch: i32) -> Self {
        Self {
            partition,
            leader_epoch,
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
            0_i32,
        );
        assert_eq!(d, OffsetForLeaderPartition::default());
    }
}

impl Readable for OffsetForLeaderPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        Ok(OffsetForLeaderPartition {
            partition, leader_epoch
        })
    }
}

impl Writable for OffsetForLeaderPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.leader_epoch.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<OffsetForLeaderEpochRequest>("OffsetForLeaderEpochRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "OffsetForLeaderEpochRequest", 0);
        }
    }
}
