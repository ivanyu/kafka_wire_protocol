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

/// LeaderAndIsrRequest, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrRequest {
    /// The current controller ID.
    pub controller_id: i32,
    /// The current controller epoch.
    pub controller_epoch: i32,
    /// The current broker epoch.
    pub broker_epoch: i64,
    /// The type that indicates whether all topics are included in the request
    pub type_: i8,
    /// Each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topic_states: Vec<LeaderAndIsrTopicState>,
    /// The current live leaders.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub live_leaders: Vec<LeaderAndIsrLiveLeader>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for LeaderAndIsrRequest {
    fn api_key(&self) -> i16 {
        4
    }
    
    fn version(&self) -> i16 {
        6
    }
}

impl Request for LeaderAndIsrRequest { }

impl Default for LeaderAndIsrRequest {
    fn default() -> Self {
        LeaderAndIsrRequest {
            controller_id: 0_i32,
            controller_epoch: 0_i32,
            broker_epoch: -1_i64,
            type_: 0_i8,
            topic_states: Vec::<LeaderAndIsrTopicState>::new(),
            live_leaders: Vec::<LeaderAndIsrLiveLeader>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderAndIsrRequest {
    pub fn new(controller_id: i32, controller_epoch: i32, broker_epoch: i64, type_: i8, topic_states: Vec<LeaderAndIsrTopicState>, live_leaders: Vec<LeaderAndIsrLiveLeader>) -> Self {
        Self {
            controller_id,
            controller_epoch,
            broker_epoch,
            type_,
            topic_states,
            live_leaders,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrRequest::new(
            0_i32,
            0_i32,
            -1_i64,
            0_i8,
            Vec::<LeaderAndIsrTopicState>::new(),
            Vec::<LeaderAndIsrLiveLeader>::new(),
        );
        assert_eq!(d, LeaderAndIsrRequest::default());
    }
}

impl Readable for LeaderAndIsrRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let controller_id = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let type_ = i8::read(input)?;
        let topic_states = read_array::<LeaderAndIsrTopicState>(input, "topic_states", true)?;
        let live_leaders = read_array::<LeaderAndIsrLiveLeader>(input, "live_leaders", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderAndIsrRequest {
            controller_id, controller_epoch, broker_epoch, type_, topic_states, live_leaders, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderAndIsrRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.controller_id.write(output)?;
        self.controller_epoch.write(output)?;
        self.broker_epoch.write(output)?;
        self.type_.write(output)?;
        write_array(output, "self.topic_states", &self.topic_states, true)?;
        write_array(output, "self.live_leaders", &self.live_leaders, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// LeaderAndIsrTopicState, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrTopicState {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The state of each partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_states: Vec<LeaderAndIsrPartitionState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for LeaderAndIsrTopicState {
    fn default() -> Self {
        LeaderAndIsrTopicState {
            topic_name: String::from(""),
            topic_id: Uuid::nil(),
            partition_states: Vec::<LeaderAndIsrPartitionState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderAndIsrTopicState {
    pub fn new<S1: AsRef<str>>(topic_name: S1, topic_id: Uuid, partition_states: Vec<LeaderAndIsrPartitionState>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            topic_id,
            partition_states,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_topic_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrTopicState::new(
            String::from(""),
            Uuid::nil(),
            Vec::<LeaderAndIsrPartitionState>::new(),
        );
        assert_eq!(d, LeaderAndIsrTopicState::default());
    }
}

impl Readable for LeaderAndIsrTopicState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let topic_id = Uuid::read(input)?;
        let partition_states = read_array::<LeaderAndIsrPartitionState>(input, "partition_states", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderAndIsrTopicState {
            topic_name, topic_id, partition_states, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderAndIsrTopicState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        self.topic_id.write(output)?;
        write_array(output, "self.partition_states", &self.partition_states, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// LeaderAndIsrLiveLeader, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrLiveLeader {
    /// The leader's broker ID.
    pub broker_id: i32,
    /// The leader's hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host_name: String,
    /// The leader's port.
    pub port: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for LeaderAndIsrLiveLeader {
    fn default() -> Self {
        LeaderAndIsrLiveLeader {
            broker_id: 0_i32,
            host_name: String::from(""),
            port: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderAndIsrLiveLeader {
    pub fn new<S1: AsRef<str>>(broker_id: i32, host_name: S1, port: i32) -> Self {
        Self {
            broker_id,
            host_name: host_name.as_ref().to_string(),
            port,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_live_leader_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrLiveLeader::new(
            0_i32,
            String::from(""),
            0_i32,
        );
        assert_eq!(d, LeaderAndIsrLiveLeader::default());
    }
}

impl Readable for LeaderAndIsrLiveLeader {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let host_name = String::read_ext(input, "host_name", true)?;
        let port = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderAndIsrLiveLeader {
            broker_id, host_name, port, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderAndIsrLiveLeader {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.host_name.write_ext(output, "self.host_name", true)?;
        self.port.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// LeaderAndIsrPartitionState, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrPartitionState {
    /// The partition index.
    pub partition_index: i32,
    /// The controller epoch.
    pub controller_epoch: i32,
    /// The broker ID of the leader.
    pub leader: i32,
    /// The leader epoch.
    pub leader_epoch: i32,
    /// The in-sync replica IDs.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub isr: Vec<i32>,
    /// The current epoch for the partition. The epoch is a monotonically increasing value which is incremented after every partition change. (Since the LeaderAndIsr request is only used by the legacy controller, this corresponds to the zkVersion)
    pub partition_epoch: i32,
    /// The replica IDs.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub replicas: Vec<i32>,
    /// The replica IDs that we are adding this partition to, or null if no replicas are being added.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub adding_replicas: Vec<i32>,
    /// The replica IDs that we are removing this partition from, or null if no replicas are being removed.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub removing_replicas: Vec<i32>,
    /// Whether the replica should have existed on the broker or not.
    pub is_new: bool,
    /// 1 if the partition is recovering from an unclean leader election; 0 otherwise.
    pub leader_recovery_state: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for LeaderAndIsrPartitionState {
    fn default() -> Self {
        LeaderAndIsrPartitionState {
            partition_index: 0_i32,
            controller_epoch: 0_i32,
            leader: 0_i32,
            leader_epoch: 0_i32,
            isr: Vec::<i32>::new(),
            partition_epoch: 0_i32,
            replicas: Vec::<i32>::new(),
            adding_replicas: Vec::<i32>::new(),
            removing_replicas: Vec::<i32>::new(),
            is_new: false,
            leader_recovery_state: 0_i8,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderAndIsrPartitionState {
    pub fn new(partition_index: i32, controller_epoch: i32, leader: i32, leader_epoch: i32, isr: Vec<i32>, partition_epoch: i32, replicas: Vec<i32>, adding_replicas: Vec<i32>, removing_replicas: Vec<i32>, is_new: bool, leader_recovery_state: i8) -> Self {
        Self {
            partition_index,
            controller_epoch,
            leader,
            leader_epoch,
            isr,
            partition_epoch,
            replicas,
            adding_replicas,
            removing_replicas,
            is_new,
            leader_recovery_state,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_partition_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrPartitionState::new(
            0_i32,
            0_i32,
            0_i32,
            0_i32,
            Vec::<i32>::new(),
            0_i32,
            Vec::<i32>::new(),
            Vec::<i32>::new(),
            Vec::<i32>::new(),
            false,
            0_i8,
        );
        assert_eq!(d, LeaderAndIsrPartitionState::default());
    }
}

impl Readable for LeaderAndIsrPartitionState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let leader = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let isr = read_array::<i32>(input, "isr", true)?;
        let partition_epoch = i32::read(input)?;
        let replicas = read_array::<i32>(input, "replicas", true)?;
        let adding_replicas = read_array::<i32>(input, "adding_replicas", true)?;
        let removing_replicas = read_array::<i32>(input, "removing_replicas", true)?;
        let is_new = bool::read(input)?;
        let leader_recovery_state = i8::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderAndIsrPartitionState {
            partition_index, controller_epoch, leader, leader_epoch, isr, partition_epoch, replicas, adding_replicas, removing_replicas, is_new, leader_recovery_state, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderAndIsrPartitionState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.controller_epoch.write(output)?;
        self.leader.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.isr", &self.isr, true)?;
        self.partition_epoch.write(output)?;
        write_array(output, "self.replicas", &self.replicas, true)?;
        write_array(output, "self.adding_replicas", &self.adding_replicas, true)?;
        write_array(output, "self.removing_replicas", &self.removing_replicas, true)?;
        self.is_new.write(output)?;
        self.leader_recovery_state.write(output)?;
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
        crate::test_utils::test_java_default::<LeaderAndIsrRequest>("LeaderAndIsrRequest", 6);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: LeaderAndIsrRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: LeaderAndIsrRequest) {
            crate::test_utils::test_java_arbitrary(&data, "LeaderAndIsrRequest", 6);
        }
    }
}
