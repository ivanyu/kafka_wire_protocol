// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// UpdateMetadataRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateMetadataRequest {
    /// The controller id.
    pub controller_id: i32,
    /// The controller epoch.
    pub controller_epoch: i32,
    /// In older versions of this RPC, each partition that we would like to update.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub ungrouped_partition_states: Vec<UpdateMetadataPartitionState>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub live_brokers: Vec<UpdateMetadataBroker>,
}

impl ApiMessage for UpdateMetadataRequest {
    fn api_key(&self) -> i16 {
        6
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for UpdateMetadataRequest { }

impl Default for UpdateMetadataRequest {
    fn default() -> Self {
        UpdateMetadataRequest {
            controller_id: 0_i32,
            controller_epoch: 0_i32,
            ungrouped_partition_states: Vec::<UpdateMetadataPartitionState>::new(),
            live_brokers: Vec::<UpdateMetadataBroker>::new(),
        }
    }
}

impl UpdateMetadataRequest {
    pub fn new(controller_id: i32, controller_epoch: i32, ungrouped_partition_states: Vec<UpdateMetadataPartitionState>, live_brokers: Vec<UpdateMetadataBroker>) -> Self {
        Self {
            controller_id,
            controller_epoch,
            ungrouped_partition_states,
            live_brokers,
        }
    }
}

#[cfg(test)]
mod tests_update_metadata_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateMetadataRequest::new(
            0_i32,
            0_i32,
            Vec::<UpdateMetadataPartitionState>::new(),
            Vec::<UpdateMetadataBroker>::new(),
        );
        assert_eq!(d, UpdateMetadataRequest::default());
    }
}

impl Readable for UpdateMetadataRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let controller_id = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let ungrouped_partition_states = read_array::<UpdateMetadataPartitionState>(input, "ungrouped_partition_states", false)?;
        let live_brokers = read_array::<UpdateMetadataBroker>(input, "live_brokers", false)?;
        Ok(UpdateMetadataRequest {
            controller_id, controller_epoch, ungrouped_partition_states, live_brokers
        })
    }
}

impl Writable for UpdateMetadataRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.controller_id.write(output)?;
        self.controller_epoch.write(output)?;
        write_array(output, "self.ungrouped_partition_states", &self.ungrouped_partition_states, false)?;
        write_array(output, "self.live_brokers", &self.live_brokers, false)?;
        Ok(())
    }
}

/// UpdateMetadataBroker, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateMetadataBroker {
    /// The broker id.
    pub id: i32,
    /// The broker hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub v0_host: String,
    /// The broker port.
    pub v0_port: i32,
}

impl Default for UpdateMetadataBroker {
    fn default() -> Self {
        UpdateMetadataBroker {
            id: 0_i32,
            v0_host: String::from(""),
            v0_port: 0_i32,
        }
    }
}

impl UpdateMetadataBroker {
    pub fn new<S1: AsRef<str>>(id: i32, v0_host: S1, v0_port: i32) -> Self {
        Self {
            id,
            v0_host: v0_host.as_ref().to_string(),
            v0_port,
        }
    }
}

#[cfg(test)]
mod tests_update_metadata_broker_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateMetadataBroker::new(
            0_i32,
            String::from(""),
            0_i32,
        );
        assert_eq!(d, UpdateMetadataBroker::default());
    }
}

impl Readable for UpdateMetadataBroker {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let id = i32::read(input)?;
        let v0_host = String::read_ext(input, "v0_host", false)?;
        let v0_port = i32::read(input)?;
        Ok(UpdateMetadataBroker {
            id, v0_host, v0_port
        })
    }
}

impl Writable for UpdateMetadataBroker {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.id.write(output)?;
        self.v0_host.write_ext(output, "self.v0_host", false)?;
        self.v0_port.write(output)?;
        Ok(())
    }
}

/// UpdateMetadataPartitionState, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateMetadataPartitionState {
    /// In older versions of this RPC, the topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partition index.
    pub partition_index: i32,
    /// The controller epoch.
    pub controller_epoch: i32,
    /// The ID of the broker which is the current partition leader.
    pub leader: i32,
    /// The leader epoch of this partition.
    pub leader_epoch: i32,
    /// The brokers which are in the ISR for this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub isr: Vec<i32>,
    /// The Zookeeper version.
    pub zk_version: i32,
    /// All the replicas of this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub replicas: Vec<i32>,
}

impl Default for UpdateMetadataPartitionState {
    fn default() -> Self {
        UpdateMetadataPartitionState {
            topic_name: String::from(""),
            partition_index: 0_i32,
            controller_epoch: 0_i32,
            leader: 0_i32,
            leader_epoch: 0_i32,
            isr: Vec::<i32>::new(),
            zk_version: 0_i32,
            replicas: Vec::<i32>::new(),
        }
    }
}

impl UpdateMetadataPartitionState {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32, controller_epoch: i32, leader: i32, leader_epoch: i32, isr: Vec<i32>, zk_version: i32, replicas: Vec<i32>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
            controller_epoch,
            leader,
            leader_epoch,
            isr,
            zk_version,
            replicas,
        }
    }
}

#[cfg(test)]
mod tests_update_metadata_partition_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateMetadataPartitionState::new(
            String::from(""),
            0_i32,
            0_i32,
            0_i32,
            0_i32,
            Vec::<i32>::new(),
            0_i32,
            Vec::<i32>::new(),
        );
        assert_eq!(d, UpdateMetadataPartitionState::default());
    }
}

impl Readable for UpdateMetadataPartitionState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", false)?;
        let partition_index = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let leader = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let isr = read_array::<i32>(input, "isr", false)?;
        let zk_version = i32::read(input)?;
        let replicas = read_array::<i32>(input, "replicas", false)?;
        Ok(UpdateMetadataPartitionState {
            topic_name, partition_index, controller_epoch, leader, leader_epoch, isr, zk_version, replicas
        })
    }
}

impl Writable for UpdateMetadataPartitionState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", false)?;
        self.partition_index.write(output)?;
        self.controller_epoch.write(output)?;
        self.leader.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.isr", &self.isr, false)?;
        self.zk_version.write(output)?;
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
        crate::test_utils::test_java_default::<UpdateMetadataRequest>("UpdateMetadataRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UpdateMetadataRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UpdateMetadataRequest) {
            crate::test_utils::test_java_arbitrary(&data, "UpdateMetadataRequest", 0);
        }
    }
}
