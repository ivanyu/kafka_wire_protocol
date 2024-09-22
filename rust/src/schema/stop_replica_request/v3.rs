// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// StopReplicaRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaRequest {
    /// The controller id.
    pub controller_id: i32,
    /// The controller epoch.
    pub controller_epoch: i32,
    /// The broker epoch.
    pub broker_epoch: i64,
    /// Each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topic_states: Vec<StopReplicaTopicState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for StopReplicaRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        5
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Request for StopReplicaRequest { }

impl Default for StopReplicaRequest {
    fn default() -> Self {
        StopReplicaRequest {
            controller_id: 0_i32,
            controller_epoch: 0_i32,
            broker_epoch: -1_i64,
            topic_states: Vec::<StopReplicaTopicState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl StopReplicaRequest {
    pub fn new(controller_id: i32, controller_epoch: i32, broker_epoch: i64, topic_states: Vec<StopReplicaTopicState>) -> Self {
        Self {
            controller_id,
            controller_epoch,
            broker_epoch,
            topic_states,
            _unknown_tagged_fields: vec![],
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
            Vec::<StopReplicaTopicState>::new(),
        );
        assert_eq!(d, StopReplicaRequest::default());
    }
}

impl Readable for StopReplicaRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let controller_id = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let topic_states = read_array::<StopReplicaTopicState>(input, "topic_states", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(StopReplicaRequest {
            controller_id, controller_epoch, broker_epoch, topic_states, _unknown_tagged_fields
        })
    }
}

impl Writable for StopReplicaRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.controller_id.write(output)?;
        self.controller_epoch.write(output)?;
        self.broker_epoch.write(output)?;
        write_array(output, "self.topic_states", &self.topic_states, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// StopReplicaTopicState, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaTopicState {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The state of each partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_states: Vec<StopReplicaPartitionState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for StopReplicaTopicState {
    fn default() -> Self {
        StopReplicaTopicState {
            topic_name: String::from(""),
            partition_states: Vec::<StopReplicaPartitionState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl StopReplicaTopicState {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_states: Vec<StopReplicaPartitionState>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_states,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_topic_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaTopicState::new(
            String::from(""),
            Vec::<StopReplicaPartitionState>::new(),
        );
        assert_eq!(d, StopReplicaTopicState::default());
    }
}

impl Readable for StopReplicaTopicState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partition_states = read_array::<StopReplicaPartitionState>(input, "partition_states", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(StopReplicaTopicState {
            topic_name, partition_states, _unknown_tagged_fields
        })
    }
}

impl Writable for StopReplicaTopicState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        write_array(output, "self.partition_states", &self.partition_states, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// StopReplicaPartitionState, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaPartitionState {
    /// The partition index.
    pub partition_index: i32,
    /// The leader epoch.
    pub leader_epoch: i32,
    /// Whether this partition should be deleted.
    pub delete_partition: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for StopReplicaPartitionState {
    fn default() -> Self {
        StopReplicaPartitionState {
            partition_index: 0_i32,
            leader_epoch: -1_i32,
            delete_partition: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl StopReplicaPartitionState {
    pub fn new(partition_index: i32, leader_epoch: i32, delete_partition: bool) -> Self {
        Self {
            partition_index,
            leader_epoch,
            delete_partition,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_partition_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaPartitionState::new(
            0_i32,
            -1_i32,
            false,
        );
        assert_eq!(d, StopReplicaPartitionState::default());
    }
}

impl Readable for StopReplicaPartitionState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let delete_partition = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(StopReplicaPartitionState {
            partition_index, leader_epoch, delete_partition, _unknown_tagged_fields
        })
    }
}

impl Writable for StopReplicaPartitionState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.leader_epoch.write(output)?;
        self.delete_partition.write(output)?;
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
        crate::test_utils::test_java_default::<StopReplicaRequest>("StopReplicaRequest", 3);
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
            crate::test_utils::test_java_arbitrary(&data, "StopReplicaRequest", 3);
        }
    }
}
