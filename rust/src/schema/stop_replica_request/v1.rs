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
pub struct StopReplicaRequest {
    /// The controller id.
    pub controller_id: i32,
    /// The controller epoch.
    pub controller_epoch: i32,
    /// The broker epoch.
    pub broker_epoch: i64,
    /// Whether these partitions should be deleted.
    pub delete_partitions: bool,
    /// The topics to stop.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<StopReplicaTopicV1>,
}

impl ApiMessage for StopReplicaRequest {
    fn api_key(&self) -> i16 {
        5
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for StopReplicaRequest { }

impl Default for StopReplicaRequest {
    fn default() -> Self {
        StopReplicaRequest {
            controller_id: 0_i32,
            controller_epoch: 0_i32,
            broker_epoch: -1_i64,
            delete_partitions: false,
            topics: Vec::<StopReplicaTopicV1>::new(),
        }
    }
}

impl StopReplicaRequest {
    pub fn new(controller_id: i32, controller_epoch: i32, broker_epoch: i64, delete_partitions: bool, topics: Vec<StopReplicaTopicV1>) -> Self {
        Self {
            controller_id,
            controller_epoch,
            broker_epoch,
            delete_partitions,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaRequest::new(
            0_i32,
            0_i32,
            -1_i64,
            false,
            Vec::<StopReplicaTopicV1>::new(),
        );
        assert_eq!(d, StopReplicaRequest::default());
    }
}

impl Readable for StopReplicaRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let controller_id = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let delete_partitions = bool::read(input)?;
        let topics = read_array::<StopReplicaTopicV1>(input, "topics", false)?;
        Ok(StopReplicaRequest {
            controller_id, controller_epoch, broker_epoch, delete_partitions, topics
        })
    }
}

impl Writable for StopReplicaRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.controller_id.write(output)?;
        self.controller_epoch.write(output)?;
        self.broker_epoch.write(output)?;
        self.delete_partitions.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaTopicV1 {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partition indexes.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_indexes: Vec<i32>,
}

impl ApiMessage for StopReplicaTopicV1 {
    fn api_key(&self) -> i16 {
        5
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for StopReplicaTopicV1 { }

impl Default for StopReplicaTopicV1 {
    fn default() -> Self {
        StopReplicaTopicV1 {
            name: String::from(""),
            partition_indexes: Vec::<i32>::new(),
        }
    }
}

impl StopReplicaTopicV1 {
    pub fn new<S1: AsRef<str>>(name: S1, partition_indexes: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_indexes,
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_topic_v1_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaTopicV1::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, StopReplicaTopicV1::default());
    }
}

impl Readable for StopReplicaTopicV1 {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partition_indexes = read_array::<i32>(input, "partition_indexes", false)?;
        Ok(StopReplicaTopicV1 {
            name, partition_indexes
        })
    }
}

impl Writable for StopReplicaTopicV1 {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partition_indexes", &self.partition_indexes, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<StopReplicaRequest>("StopReplicaRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: StopReplicaRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: StopReplicaRequest) {
            crate::test_utils::test_java_arbitrary(&data, "StopReplicaRequest", 1);
        }
    }
}
