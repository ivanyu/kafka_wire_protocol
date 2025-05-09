// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// MetadataResponse, version 13.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// A list of brokers present in the cluster.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub brokers: Vec<MetadataResponseBroker>,
    /// The cluster ID that responding broker belongs to.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub cluster_id: Option<String>,
    /// The ID of the controller broker.
    pub controller_id: i32,
    /// Each topic in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<MetadataResponseTopic>,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for MetadataResponse {
    fn api_key(&self) -> i16 {
        3
    }
    
    fn version(&self) -> i16 {
        13
    }
}

impl Response for MetadataResponse { }

impl Default for MetadataResponse {
    fn default() -> Self {
        MetadataResponse {
            throttle_time_ms: 0_i32,
            brokers: Vec::<MetadataResponseBroker>::new(),
            cluster_id: None,
            controller_id: -1_i32,
            topics: Vec::<MetadataResponseTopic>::new(),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MetadataResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, brokers: Vec<MetadataResponseBroker>, cluster_id: Option<S1>, controller_id: i32, topics: Vec<MetadataResponseTopic>, error_code: i16) -> Self {
        Self {
            throttle_time_ms,
            brokers,
            cluster_id: cluster_id.map(|s| s.as_ref().to_string()),
            controller_id,
            topics,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_metadata_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MetadataResponse::new(
            0_i32,
            Vec::<MetadataResponseBroker>::new(),
            None::<String>,
            -1_i32,
            Vec::<MetadataResponseTopic>::new(),
            0_i16,
        );
        assert_eq!(d, MetadataResponse::default());
    }
}

impl Readable for MetadataResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let brokers = read_array::<MetadataResponseBroker>(input, "brokers", true)?;
        let cluster_id = Option::<String>::read_ext(input, "cluster_id", true)?;
        let controller_id = i32::read(input)?;
        let topics = read_array::<MetadataResponseTopic>(input, "topics", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MetadataResponse {
            throttle_time_ms, brokers, cluster_id, controller_id, topics, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for MetadataResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.brokers", &self.brokers, true)?;
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
        self.controller_id.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        self.error_code.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// MetadataResponseBroker, version 13.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataResponseBroker {
    /// The broker ID.
    pub node_id: i32,
    /// The broker hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The broker port.
    pub port: i32,
    /// The rack of the broker, or null if it has not been assigned to a rack.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for MetadataResponseBroker {
    fn default() -> Self {
        MetadataResponseBroker {
            node_id: 0_i32,
            host: String::from(""),
            port: 0_i32,
            rack: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MetadataResponseBroker {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(node_id: i32, host: S1, port: i32, rack: Option<S2>) -> Self {
        Self {
            node_id,
            host: host.as_ref().to_string(),
            port,
            rack: rack.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_metadata_response_broker_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MetadataResponseBroker::new(
            0_i32,
            String::from(""),
            0_i32,
            None::<String>,
        );
        assert_eq!(d, MetadataResponseBroker::default());
    }
}

impl Readable for MetadataResponseBroker {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let node_id = i32::read(input)?;
        let host = String::read_ext(input, "host", true)?;
        let port = i32::read(input)?;
        let rack = Option::<String>::read_ext(input, "rack", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MetadataResponseBroker {
            node_id, host, port, rack, _unknown_tagged_fields
        })
    }
}

impl Writable for MetadataResponseBroker {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.node_id.write(output)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
        self.rack.write_ext(output, "self.rack", true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// MetadataResponseTopic, version 13.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: i16,
    /// The topic name. Null for non-existing topics queried by ID. This is never null when ErrorCode is zero. One of Name and TopicId is always populated.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub name: Option<String>,
    /// The topic id. Zero for non-existing topics queried by name. This is never zero when ErrorCode is zero. One of Name and TopicId is always populated.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// True if the topic is internal.
    pub is_internal: bool,
    /// Each partition in the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<MetadataResponsePartition>,
    /// 32-bit bitfield to represent authorized operations for this topic.
    pub topic_authorized_operations: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for MetadataResponseTopic {
    fn default() -> Self {
        MetadataResponseTopic {
            error_code: 0_i16,
            name: Some(String::from("")),
            topic_id: Uuid::nil(),
            is_internal: false,
            partitions: Vec::<MetadataResponsePartition>::new(),
            topic_authorized_operations: -2147483648_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MetadataResponseTopic {
    pub fn new<S1: AsRef<str>>(error_code: i16, name: Option<S1>, topic_id: Uuid, is_internal: bool, partitions: Vec<MetadataResponsePartition>, topic_authorized_operations: i32) -> Self {
        Self {
            error_code,
            name: name.map(|s| s.as_ref().to_string()),
            topic_id,
            is_internal,
            partitions,
            topic_authorized_operations,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_metadata_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MetadataResponseTopic::new(
            0_i16,
            Some(String::from("")),
            Uuid::nil(),
            false,
            Vec::<MetadataResponsePartition>::new(),
            -2147483648_i32,
        );
        assert_eq!(d, MetadataResponseTopic::default());
    }
}

impl Readable for MetadataResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let name = Option::<String>::read_ext(input, "name", true)?;
        let topic_id = Uuid::read(input)?;
        let is_internal = bool::read(input)?;
        let partitions = read_array::<MetadataResponsePartition>(input, "partitions", true)?;
        let topic_authorized_operations = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MetadataResponseTopic {
            error_code, name, topic_id, is_internal, partitions, topic_authorized_operations, _unknown_tagged_fields
        })
    }
}

impl Writable for MetadataResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.name.write_ext(output, "self.name", true)?;
        self.topic_id.write(output)?;
        self.is_internal.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        self.topic_authorized_operations.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// MetadataResponsePartition, version 13.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataResponsePartition {
    /// The partition error, or 0 if there was no error.
    pub error_code: i16,
    /// The partition index.
    pub partition_index: i32,
    /// The ID of the leader broker.
    pub leader_id: i32,
    /// The leader epoch of this partition.
    pub leader_epoch: i32,
    /// The set of all nodes that host this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub replica_nodes: Vec<i32>,
    /// The set of nodes that are in sync with the leader for this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub isr_nodes: Vec<i32>,
    /// The set of offline replicas of this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub offline_replicas: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for MetadataResponsePartition {
    fn default() -> Self {
        MetadataResponsePartition {
            error_code: 0_i16,
            partition_index: 0_i32,
            leader_id: 0_i32,
            leader_epoch: -1_i32,
            replica_nodes: Vec::<i32>::new(),
            isr_nodes: Vec::<i32>::new(),
            offline_replicas: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MetadataResponsePartition {
    pub fn new(error_code: i16, partition_index: i32, leader_id: i32, leader_epoch: i32, replica_nodes: Vec<i32>, isr_nodes: Vec<i32>, offline_replicas: Vec<i32>) -> Self {
        Self {
            error_code,
            partition_index,
            leader_id,
            leader_epoch,
            replica_nodes,
            isr_nodes,
            offline_replicas,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_metadata_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MetadataResponsePartition::new(
            0_i16,
            0_i32,
            0_i32,
            -1_i32,
            Vec::<i32>::new(),
            Vec::<i32>::new(),
            Vec::<i32>::new(),
        );
        assert_eq!(d, MetadataResponsePartition::default());
    }
}

impl Readable for MetadataResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let partition_index = i32::read(input)?;
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let replica_nodes = read_array::<i32>(input, "replica_nodes", true)?;
        let isr_nodes = read_array::<i32>(input, "isr_nodes", true)?;
        let offline_replicas = read_array::<i32>(input, "offline_replicas", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MetadataResponsePartition {
            error_code, partition_index, leader_id, leader_epoch, replica_nodes, isr_nodes, offline_replicas, _unknown_tagged_fields
        })
    }
}

impl Writable for MetadataResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.partition_index.write(output)?;
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.replica_nodes", &self.replica_nodes, true)?;
        write_array(output, "self.isr_nodes", &self.isr_nodes, true)?;
        write_array(output, "self.offline_replicas", &self.offline_replicas, true)?;
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
        crate::test_utils::test_java_default::<MetadataResponse>("MetadataResponse", 13);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: MetadataResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: MetadataResponse) {
            crate::test_utils::test_java_arbitrary(&data, "MetadataResponse", 13);
        }
    }
}
