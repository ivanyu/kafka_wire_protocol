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
pub struct LeaderAndIsrRequest {
    /// The current controller ID.
    pub controller_id: i32,
    /// The current controller epoch.
    pub controller_epoch: i32,
    /// The state of each partition, in a v0 or v1 message.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub ungrouped_partition_states: Vec<LeaderAndIsrPartitionState>,
    /// The current live leaders.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub live_leaders: Vec<LeaderAndIsrLiveLeader>,
}

impl ApiMessage for LeaderAndIsrRequest {
    fn api_key(&self) -> i16 {
        4
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for LeaderAndIsrRequest { }

impl Default for LeaderAndIsrRequest {
    fn default() -> Self {
        LeaderAndIsrRequest {
            controller_id: 0_i32,
            controller_epoch: 0_i32,
            ungrouped_partition_states: Vec::<LeaderAndIsrPartitionState>::new(),
            live_leaders: Vec::<LeaderAndIsrLiveLeader>::new(),
        }
    }
}

impl LeaderAndIsrRequest {
    pub fn new(controller_id: i32, controller_epoch: i32, ungrouped_partition_states: Vec<LeaderAndIsrPartitionState>, live_leaders: Vec<LeaderAndIsrLiveLeader>) -> Self {
        Self {
            controller_id,
            controller_epoch,
            ungrouped_partition_states,
            live_leaders,
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
            Vec::<LeaderAndIsrPartitionState>::new(),
            Vec::<LeaderAndIsrLiveLeader>::new(),
        );
        assert_eq!(d, LeaderAndIsrRequest::default());
    }
}

impl Readable for LeaderAndIsrRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let controller_id = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let ungrouped_partition_states = read_array::<LeaderAndIsrPartitionState>(input, "ungrouped_partition_states", false)?;
        let live_leaders = read_array::<LeaderAndIsrLiveLeader>(input, "live_leaders", false)?;
        Ok(LeaderAndIsrRequest {
            controller_id, controller_epoch, ungrouped_partition_states, live_leaders
        })
    }
}

impl Writable for LeaderAndIsrRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.controller_id.write(output)?;
        self.controller_epoch.write(output)?;
        write_array(output, "self.ungrouped_partition_states", &self.ungrouped_partition_states, false)?;
        write_array(output, "self.live_leaders", &self.live_leaders, false)?;
        Ok(())
    }
}

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
}

impl ApiMessage for LeaderAndIsrLiveLeader {
    fn api_key(&self) -> i16 {
        4
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for LeaderAndIsrLiveLeader { }

impl Default for LeaderAndIsrLiveLeader {
    fn default() -> Self {
        LeaderAndIsrLiveLeader {
            broker_id: 0_i32,
            host_name: String::from(""),
            port: 0_i32,
        }
    }
}

impl LeaderAndIsrLiveLeader {
    pub fn new<S1: AsRef<str>>(broker_id: i32, host_name: S1, port: i32) -> Self {
        Self {
            broker_id,
            host_name: host_name.as_ref().to_string(),
            port,
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
        let host_name = String::read_ext(input, "host_name", false)?;
        let port = i32::read(input)?;
        Ok(LeaderAndIsrLiveLeader {
            broker_id, host_name, port
        })
    }
}

impl Writable for LeaderAndIsrLiveLeader {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.host_name.write_ext(output, "self.host_name", false)?;
        self.port.write(output)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderAndIsrPartitionState {
    /// The topic name.  This is only present in v0 or v1.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
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
}

impl ApiMessage for LeaderAndIsrPartitionState {
    fn api_key(&self) -> i16 {
        4
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for LeaderAndIsrPartitionState { }

impl Default for LeaderAndIsrPartitionState {
    fn default() -> Self {
        LeaderAndIsrPartitionState {
            topic_name: String::from(""),
            partition_index: 0_i32,
            controller_epoch: 0_i32,
            leader: 0_i32,
            leader_epoch: 0_i32,
            isr: Vec::<i32>::new(),
            partition_epoch: 0_i32,
            replicas: Vec::<i32>::new(),
        }
    }
}

impl LeaderAndIsrPartitionState {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32, controller_epoch: i32, leader: i32, leader_epoch: i32, isr: Vec<i32>, partition_epoch: i32, replicas: Vec<i32>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
            controller_epoch,
            leader,
            leader_epoch,
            isr,
            partition_epoch,
            replicas,
        }
    }
}

#[cfg(test)]
mod tests_leader_and_isr_partition_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderAndIsrPartitionState::new(
            String::from(""),
            0_i32,
            0_i32,
            0_i32,
            0_i32,
            Vec::<i32>::new(),
            0_i32,
            Vec::<i32>::new(),
        );
        assert_eq!(d, LeaderAndIsrPartitionState::default());
    }
}

impl Readable for LeaderAndIsrPartitionState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", false)?;
        let partition_index = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let leader = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let isr = read_array::<i32>(input, "isr", false)?;
        let partition_epoch = i32::read(input)?;
        let replicas = read_array::<i32>(input, "replicas", false)?;
        Ok(LeaderAndIsrPartitionState {
            topic_name, partition_index, controller_epoch, leader, leader_epoch, isr, partition_epoch, replicas
        })
    }
}

impl Writable for LeaderAndIsrPartitionState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", false)?;
        self.partition_index.write(output)?;
        self.controller_epoch.write(output)?;
        self.leader.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.isr", &self.isr, false)?;
        self.partition_epoch.write(output)?;
        write_array(output, "self.replicas", &self.replicas, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<LeaderAndIsrRequest>("LeaderAndIsrRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "LeaderAndIsrRequest", 0);
        }
    }
}
