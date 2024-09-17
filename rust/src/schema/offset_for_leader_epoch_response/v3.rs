// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetForLeaderEpochResponse, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetForLeaderEpochResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each topic we fetched offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetForLeaderTopicResult>,
}

impl ApiMessage for OffsetForLeaderEpochResponse {
    fn api_key(&self) -> i16 {
        23
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Response for OffsetForLeaderEpochResponse { }

impl Default for OffsetForLeaderEpochResponse {
    fn default() -> Self {
        OffsetForLeaderEpochResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<OffsetForLeaderTopicResult>::new(),
        }
    }
}

impl OffsetForLeaderEpochResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<OffsetForLeaderTopicResult>) -> Self {
        Self {
            throttle_time_ms,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_offset_for_leader_epoch_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetForLeaderEpochResponse::new(
            0_i32,
            Vec::<OffsetForLeaderTopicResult>::new(),
        );
        assert_eq!(d, OffsetForLeaderEpochResponse::default());
    }
}

impl Readable for OffsetForLeaderEpochResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<OffsetForLeaderTopicResult>(input, "topics", false)?;
        Ok(OffsetForLeaderEpochResponse {
            throttle_time_ms, topics
        })
    }
}

impl Writable for OffsetForLeaderEpochResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// OffsetForLeaderTopicResult, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetForLeaderTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// Each partition in the topic we fetched offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<EpochEndOffset>,
}

impl Default for OffsetForLeaderTopicResult {
    fn default() -> Self {
        OffsetForLeaderTopicResult {
            topic: String::from(""),
            partitions: Vec::<EpochEndOffset>::new(),
        }
    }
}

impl OffsetForLeaderTopicResult {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<EpochEndOffset>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_offset_for_leader_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetForLeaderTopicResult::new(
            String::from(""),
            Vec::<EpochEndOffset>::new(),
        );
        assert_eq!(d, OffsetForLeaderTopicResult::default());
    }
}

impl Readable for OffsetForLeaderTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", false)?;
        let partitions = read_array::<EpochEndOffset>(input, "partitions", false)?;
        Ok(OffsetForLeaderTopicResult {
            topic, partitions
        })
    }
}

impl Writable for OffsetForLeaderTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// EpochEndOffset, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EpochEndOffset {
    /// The error code 0, or if there was no error.
    pub error_code: i16,
    /// The partition index.
    pub partition: i32,
    /// The leader epoch of the partition.
    pub leader_epoch: i32,
    /// The end offset of the epoch.
    pub end_offset: i64,
}

impl Default for EpochEndOffset {
    fn default() -> Self {
        EpochEndOffset {
            error_code: 0_i16,
            partition: 0_i32,
            leader_epoch: -1_i32,
            end_offset: -1_i64,
        }
    }
}

impl EpochEndOffset {
    pub fn new(error_code: i16, partition: i32, leader_epoch: i32, end_offset: i64) -> Self {
        Self {
            error_code,
            partition,
            leader_epoch,
            end_offset,
        }
    }
}

#[cfg(test)]
mod tests_epoch_end_offset_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EpochEndOffset::new(
            0_i16,
            0_i32,
            -1_i32,
            -1_i64,
        );
        assert_eq!(d, EpochEndOffset::default());
    }
}

impl Readable for EpochEndOffset {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let partition = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let end_offset = i64::read(input)?;
        Ok(EpochEndOffset {
            error_code, partition, leader_epoch, end_offset
        })
    }
}

impl Writable for EpochEndOffset {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.partition.write(output)?;
        self.leader_epoch.write(output)?;
        self.end_offset.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<OffsetForLeaderEpochResponse>("OffsetForLeaderEpochResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetForLeaderEpochResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetForLeaderEpochResponse) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetForLeaderEpochResponse", 3);
        }
    }
}
