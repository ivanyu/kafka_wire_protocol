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
pub struct EndQuorumEpochRequest {
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub cluster_id: Option<String>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
}

impl ApiMessage for EndQuorumEpochRequest {
    fn api_key(&self) -> i16 {
        54
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for EndQuorumEpochRequest { }

impl Default for EndQuorumEpochRequest {
    fn default() -> Self {
        EndQuorumEpochRequest {
            cluster_id: None,
            topics: Vec::<TopicData>::new(),
        }
    }
}

impl EndQuorumEpochRequest {
    pub fn new<S1: AsRef<str>>(cluster_id: Option<S1>, topics: Vec<TopicData>) -> Self {
        Self {
            cluster_id: cluster_id.map(|s| s.as_ref().to_string()),
            topics,
        }
    }
}

#[cfg(test)]
mod tests_end_quorum_epoch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EndQuorumEpochRequest::new(
            None::<String>,
            Vec::<TopicData>::new(),
        );
        assert_eq!(d, EndQuorumEpochRequest::default());
    }
}

impl Readable for EndQuorumEpochRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let cluster_id = Option::<String>::read_ext(input, "cluster_id", false)?;
        let topics = read_array::<TopicData>(input, "topics", false)?;
        Ok(EndQuorumEpochRequest {
            cluster_id, topics
        })
    }
}

impl Writable for EndQuorumEpochRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.cluster_id.write_ext(output, "self.cluster_id", false)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

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

impl ApiMessage for TopicData {
    fn api_key(&self) -> i16 {
        54
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for TopicData { }

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// The current leader ID that is resigning
    pub leader_id: i32,
    /// The current epoch
    pub leader_epoch: i32,
    /// A sorted list of preferred successors to start the election
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub preferred_successors: Vec<i32>,
}

impl ApiMessage for PartitionData {
    fn api_key(&self) -> i16 {
        54
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for PartitionData { }

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            leader_id: 0_i32,
            leader_epoch: 0_i32,
            preferred_successors: Vec::<i32>::new(),
        }
    }
}

impl PartitionData {
    pub fn new(partition_index: i32, leader_id: i32, leader_epoch: i32, preferred_successors: Vec<i32>) -> Self {
        Self {
            partition_index,
            leader_id,
            leader_epoch,
            preferred_successors,
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
            Vec::<i32>::new(),
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let preferred_successors = read_array::<i32>(input, "preferred_successors", false)?;
        Ok(PartitionData {
            partition_index, leader_id, leader_epoch, preferred_successors
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.preferred_successors", &self.preferred_successors, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<EndQuorumEpochRequest>("EndQuorumEpochRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: EndQuorumEpochRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: EndQuorumEpochRequest) {
            crate::test_utils::test_java_arbitrary(&data, "EndQuorumEpochRequest", 0);
        }
    }
}
