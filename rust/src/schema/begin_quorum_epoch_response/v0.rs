// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// BeginQuorumEpochResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BeginQuorumEpochResponse {
    /// The top level error code.
    pub error_code: i16,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
}

impl ApiMessage for BeginQuorumEpochResponse {
    fn api_key(&self) -> i16 {
        53
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for BeginQuorumEpochResponse { }

impl Default for BeginQuorumEpochResponse {
    fn default() -> Self {
        BeginQuorumEpochResponse {
            error_code: 0_i16,
            topics: Vec::<TopicData>::new(),
        }
    }
}

impl BeginQuorumEpochResponse {
    pub fn new(error_code: i16, topics: Vec<TopicData>) -> Self {
        Self {
            error_code,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_begin_quorum_epoch_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = BeginQuorumEpochResponse::new(
            0_i16,
            Vec::<TopicData>::new(),
        );
        assert_eq!(d, BeginQuorumEpochResponse::default());
    }
}

impl Readable for BeginQuorumEpochResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let topics = read_array::<TopicData>(input, "topics", false)?;
        Ok(BeginQuorumEpochResponse {
            error_code, topics
        })
    }
}

impl Writable for BeginQuorumEpochResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// TopicData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicData {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionData>,
}

impl Default for TopicData {
    fn default() -> Self {
        TopicData {
            topic_name: String::from(""),
            partitions: Vec::<PartitionData>::new(),
        }
    }
}

impl TopicData {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partitions: Vec<PartitionData>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partitions,
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
        let topic_name = String::read_ext(input, "topic_name", false)?;
        let partitions = read_array::<PartitionData>(input, "partitions", false)?;
        Ok(TopicData {
            topic_name, partitions
        })
    }
}

impl Writable for TopicData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// PartitionData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// 
    pub error_code: i16,
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: i32,
    /// The latest known leader epoch
    pub leader_epoch: i32,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            error_code: 0_i16,
            leader_id: 0_i32,
            leader_epoch: 0_i32,
        }
    }
}

impl PartitionData {
    pub fn new(partition_index: i32, error_code: i16, leader_id: i32, leader_epoch: i32) -> Self {
        Self {
            partition_index,
            error_code,
            leader_id,
            leader_epoch,
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
            0_i16,
            0_i32,
            0_i32,
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        Ok(PartitionData {
            partition_index, error_code, leader_id, leader_epoch
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.leader_id.write(output)?;
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
        crate::test_utils::test_java_default::<BeginQuorumEpochResponse>("BeginQuorumEpochResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: BeginQuorumEpochResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: BeginQuorumEpochResponse) {
            crate::test_utils::test_java_arbitrary(&data, "BeginQuorumEpochResponse", 0);
        }
    }
}
