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

/// ShareAcknowledgeResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ShareAcknowledgeResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top level response error code.
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The response topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<ShareAcknowledgeTopicResponse>,
    /// Endpoints for all current leaders enumerated in PartitionData with error NOT_LEADER_OR_FOLLOWER.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub node_endpoints: Vec<NodeEndpoint>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ShareAcknowledgeResponse {
    fn api_key(&self) -> i16 {
        79
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ShareAcknowledgeResponse { }

impl Default for ShareAcknowledgeResponse {
    fn default() -> Self {
        ShareAcknowledgeResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: None,
            responses: Vec::<ShareAcknowledgeTopicResponse>::new(),
            node_endpoints: Vec::<NodeEndpoint>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ShareAcknowledgeResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, responses: Vec<ShareAcknowledgeTopicResponse>, node_endpoints: Vec<NodeEndpoint>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            responses,
            node_endpoints,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_share_acknowledge_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ShareAcknowledgeResponse::new(
            0_i32,
            0_i16,
            None::<String>,
            Vec::<ShareAcknowledgeTopicResponse>::new(),
            Vec::<NodeEndpoint>::new(),
        );
        assert_eq!(d, ShareAcknowledgeResponse::default());
    }
}

impl Readable for ShareAcknowledgeResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let responses = read_array::<ShareAcknowledgeTopicResponse>(input, "responses", true)?;
        let node_endpoints = read_array::<NodeEndpoint>(input, "node_endpoints", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ShareAcknowledgeResponse {
            throttle_time_ms, error_code, error_message, responses, node_endpoints, _unknown_tagged_fields
        })
    }
}

impl Writable for ShareAcknowledgeResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.responses", &self.responses, true)?;
        write_array(output, "self.node_endpoints", &self.node_endpoints, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ShareAcknowledgeTopicResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ShareAcknowledgeTopicResponse {
    /// The unique topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The topic partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ShareAcknowledgeTopicResponse {
    fn default() -> Self {
        ShareAcknowledgeTopicResponse {
            topic_id: Uuid::nil(),
            partitions: Vec::<PartitionData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ShareAcknowledgeTopicResponse {
    pub fn new(topic_id: Uuid, partitions: Vec<PartitionData>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_share_acknowledge_topic_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ShareAcknowledgeTopicResponse::new(
            Uuid::nil(),
            Vec::<PartitionData>::new(),
        );
        assert_eq!(d, ShareAcknowledgeTopicResponse::default());
    }
}

impl Readable for ShareAcknowledgeTopicResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<PartitionData>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ShareAcknowledgeTopicResponse {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for ShareAcknowledgeTopicResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The current leader of the partition.
    pub current_leader: LeaderIdAndEpoch,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            error_code: 0_i16,
            error_message: None,
            current_leader: LeaderIdAndEpoch::default(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new<S1: AsRef<str>>(partition_index: i32, error_code: i16, error_message: Option<S1>, current_leader: LeaderIdAndEpoch) -> Self {
        Self {
            partition_index,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            current_leader,
            _unknown_tagged_fields: vec![],
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
            None::<String>,
            LeaderIdAndEpoch::default(),
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let current_leader = LeaderIdAndEpoch::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition_index, error_code, error_message, current_leader, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        self.current_leader.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// LeaderIdAndEpoch, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderIdAndEpoch {
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: i32,
    /// The latest known leader epoch.
    pub leader_epoch: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for LeaderIdAndEpoch {
    fn default() -> Self {
        LeaderIdAndEpoch {
            leader_id: 0_i32,
            leader_epoch: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderIdAndEpoch {
    pub fn new(leader_id: i32, leader_epoch: i32) -> Self {
        Self {
            leader_id,
            leader_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_id_and_epoch_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderIdAndEpoch::new(
            0_i32,
            0_i32,
        );
        assert_eq!(d, LeaderIdAndEpoch::default());
    }
}

impl Readable for LeaderIdAndEpoch {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderIdAndEpoch {
            leader_id, leader_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderIdAndEpoch {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// NodeEndpoint, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct NodeEndpoint {
    /// The ID of the associated node.
    pub node_id: i32,
    /// The node's hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The node's port.
    pub port: i32,
    /// The rack of the node, or null if it has not been assigned to a rack.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for NodeEndpoint {
    fn default() -> Self {
        NodeEndpoint {
            node_id: 0_i32,
            host: String::from(""),
            port: 0_i32,
            rack: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl NodeEndpoint {
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
mod tests_node_endpoint_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = NodeEndpoint::new(
            0_i32,
            String::from(""),
            0_i32,
            None::<String>,
        );
        assert_eq!(d, NodeEndpoint::default());
    }
}

impl Readable for NodeEndpoint {
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
        Ok(NodeEndpoint {
            node_id, host, port, rack, _unknown_tagged_fields
        })
    }
}

impl Writable for NodeEndpoint {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.node_id.write(output)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
        self.rack.write_ext(output, "self.rack", true)?;
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
        crate::test_utils::test_java_default::<ShareAcknowledgeResponse>("ShareAcknowledgeResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ShareAcknowledgeResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ShareAcknowledgeResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ShareAcknowledgeResponse", 0);
        }
    }
}
