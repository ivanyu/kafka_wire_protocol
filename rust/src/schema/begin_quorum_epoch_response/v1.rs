// This file was generated. Do not edit.

use std::io::{Cursor, Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// BeginQuorumEpochResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BeginQuorumEpochResponse {
    /// The top level error code.
    pub error_code: i16,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
    /// Endpoints for all leaders enumerated in PartitionData
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub node_endpoints: Vec<NodeEndpoint>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for BeginQuorumEpochResponse {
    fn api_key(&self) -> i16 {
        53
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for BeginQuorumEpochResponse { }

impl Default for BeginQuorumEpochResponse {
    fn default() -> Self {
        BeginQuorumEpochResponse {
            error_code: 0_i16,
            topics: Vec::<TopicData>::new(),
            node_endpoints: Vec::<NodeEndpoint>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl BeginQuorumEpochResponse {
    pub fn new(error_code: i16, topics: Vec<TopicData>, node_endpoints: Vec<NodeEndpoint>) -> Self {
        Self {
            error_code,
            topics,
            node_endpoints,
            _unknown_tagged_fields: vec![],
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
            Vec::<NodeEndpoint>::new(),
        );
        assert_eq!(d, BeginQuorumEpochResponse::default());
    }
}

impl Readable for BeginQuorumEpochResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let topics = read_array::<TopicData>(input, "topics", true)?;
        let mut node_endpoints = Vec::<NodeEndpoint>::new();
        let tagged_fields_callback = |tag: i32, tag_data: &[u8]| {
            match tag {
                0 => {
                    let mut cur = Cursor::new(tag_data);
                    node_endpoints = read_array::<NodeEndpoint>(&mut cur, "node_endpoints", true)?;
                    Ok(true)
                },
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(BeginQuorumEpochResponse {
            error_code, topics, node_endpoints, _unknown_tagged_fields
        })
    }
}

impl Writable for BeginQuorumEpochResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        let mut known_tagged_fields = Vec::<RawTaggedField>::new();
        if !self.node_endpoints.is_empty() {
            let mut cur = Cursor::new(Vec::<u8>::new());
            write_array(&mut cur, "self.node_endpoints", &self.node_endpoints, true)?;
            known_tagged_fields.push(RawTaggedField { tag: 0, data: cur.into_inner() });
        }
        write_tagged_fields(output, &known_tagged_fields, &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicData, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicData {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicData {
    fn default() -> Self {
        TopicData {
            topic_name: String::from(""),
            partitions: Vec::<PartitionData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicData {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partitions: Vec<PartitionData>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
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
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partitions = read_array::<PartitionData>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicData {
            topic_name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionData, version 1.
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
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            error_code: 0_i16,
            leader_id: 0_i32,
            leader_epoch: 0_i32,
            _unknown_tagged_fields: Vec::new(),
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
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition_index, error_code, leader_id, leader_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// NodeEndpoint, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct NodeEndpoint {
    /// The ID of the associated node
    pub node_id: i32,
    /// The node's hostname
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The node's port
    pub port: u16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for NodeEndpoint {
    fn default() -> Self {
        NodeEndpoint {
            node_id: 0_i32,
            host: String::from(""),
            port: 0_u16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl NodeEndpoint {
    pub fn new<S1: AsRef<str>>(node_id: i32, host: S1, port: u16) -> Self {
        Self {
            node_id,
            host: host.as_ref().to_string(),
            port,
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
            0_u16,
        );
        assert_eq!(d, NodeEndpoint::default());
    }
}

impl Readable for NodeEndpoint {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let node_id = i32::read(input)?;
        let host = String::read_ext(input, "host", true)?;
        let port = u16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(NodeEndpoint {
            node_id, host, port, _unknown_tagged_fields
        })
    }
}

impl Writable for NodeEndpoint {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.node_id.write(output)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
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
        crate::test_utils::test_java_default::<BeginQuorumEpochResponse>("BeginQuorumEpochResponse", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "BeginQuorumEpochResponse", 1);
        }
    }
}
