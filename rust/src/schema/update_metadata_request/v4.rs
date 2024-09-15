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
        4
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateMetadataBroker {
    /// The broker id.
    pub id: i32,
    /// The broker endpoints.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub endpoints: Vec<UpdateMetadataEndpoint>,
    /// The rack which this broker belongs to.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack: Option<String>,
}

impl ApiMessage for UpdateMetadataBroker {
    fn api_key(&self) -> i16 {
        6
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for UpdateMetadataBroker { }

impl Default for UpdateMetadataBroker {
    fn default() -> Self {
        UpdateMetadataBroker {
            id: 0_i32,
            endpoints: Vec::<UpdateMetadataEndpoint>::new(),
            rack: Some(String::from("")),
        }
    }
}

impl UpdateMetadataBroker {
    pub fn new<S1: AsRef<str>>(id: i32, endpoints: Vec<UpdateMetadataEndpoint>, rack: Option<S1>) -> Self {
        Self {
            id,
            endpoints,
            rack: rack.map(|s| s.as_ref().to_string()),
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
            Vec::<UpdateMetadataEndpoint>::new(),
            Some(String::from("")),
        );
        assert_eq!(d, UpdateMetadataBroker::default());
    }
}

impl Readable for UpdateMetadataBroker {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let id = i32::read(input)?;
        let endpoints = read_array::<UpdateMetadataEndpoint>(input, "endpoints", false)?;
        let rack = Option::<String>::read_ext(input, "rack", false)?;
        Ok(UpdateMetadataBroker {
            id, endpoints, rack
        })
    }
}

impl Writable for UpdateMetadataBroker {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.id.write(output)?;
        write_array(output, "self.endpoints", &self.endpoints, false)?;
        self.rack.write_ext(output, "self.rack", false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateMetadataEndpoint {
    /// The port of this endpoint
    pub port: i32,
    /// The hostname of this endpoint
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The listener name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub listener: String,
    /// The security protocol type.
    pub security_protocol: i16,
}

impl ApiMessage for UpdateMetadataEndpoint {
    fn api_key(&self) -> i16 {
        6
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for UpdateMetadataEndpoint { }

impl Default for UpdateMetadataEndpoint {
    fn default() -> Self {
        UpdateMetadataEndpoint {
            port: 0_i32,
            host: String::from(""),
            listener: String::from(""),
            security_protocol: 0_i16,
        }
    }
}

impl UpdateMetadataEndpoint {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(port: i32, host: S1, listener: S2, security_protocol: i16) -> Self {
        Self {
            port,
            host: host.as_ref().to_string(),
            listener: listener.as_ref().to_string(),
            security_protocol,
        }
    }
}

#[cfg(test)]
mod tests_update_metadata_endpoint_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateMetadataEndpoint::new(
            0_i32,
            String::from(""),
            String::from(""),
            0_i16,
        );
        assert_eq!(d, UpdateMetadataEndpoint::default());
    }
}

impl Readable for UpdateMetadataEndpoint {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let port = i32::read(input)?;
        let host = String::read_ext(input, "host", false)?;
        let listener = String::read_ext(input, "listener", false)?;
        let security_protocol = i16::read(input)?;
        Ok(UpdateMetadataEndpoint {
            port, host, listener, security_protocol
        })
    }
}

impl Writable for UpdateMetadataEndpoint {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.port.write(output)?;
        self.host.write_ext(output, "self.host", false)?;
        self.listener.write_ext(output, "self.listener", false)?;
        self.security_protocol.write(output)?;
        Ok(())
    }
}

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
    /// The replicas of this partition which are offline.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub offline_replicas: Vec<i32>,
}

impl ApiMessage for UpdateMetadataPartitionState {
    fn api_key(&self) -> i16 {
        6
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for UpdateMetadataPartitionState { }

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
            offline_replicas: Vec::<i32>::new(),
        }
    }
}

impl UpdateMetadataPartitionState {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32, controller_epoch: i32, leader: i32, leader_epoch: i32, isr: Vec<i32>, zk_version: i32, replicas: Vec<i32>, offline_replicas: Vec<i32>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
            controller_epoch,
            leader,
            leader_epoch,
            isr,
            zk_version,
            replicas,
            offline_replicas,
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
        let offline_replicas = read_array::<i32>(input, "offline_replicas", false)?;
        Ok(UpdateMetadataPartitionState {
            topic_name, partition_index, controller_epoch, leader, leader_epoch, isr, zk_version, replicas, offline_replicas
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
        write_array(output, "self.offline_replicas", &self.offline_replicas, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<UpdateMetadataRequest>("UpdateMetadataRequest", 4);
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
            crate::test_utils::test_java_arbitrary(&data, "UpdateMetadataRequest", 4);
        }
    }
}
