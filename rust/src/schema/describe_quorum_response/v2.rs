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

/// DescribeQuorumResponse, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeQuorumResponse {
    /// The top level error code.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub nodes: Vec<Node>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeQuorumResponse {
    fn api_key(&self) -> i16 {
        55
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for DescribeQuorumResponse { }

impl Default for DescribeQuorumResponse {
    fn default() -> Self {
        DescribeQuorumResponse {
            error_code: 0_i16,
            error_message: Some(String::from("")),
            topics: Vec::<TopicData>::new(),
            nodes: Vec::<Node>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeQuorumResponse {
    pub fn new<S1: AsRef<str>>(error_code: i16, error_message: Option<S1>, topics: Vec<TopicData>, nodes: Vec<Node>) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            topics,
            nodes,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_quorum_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeQuorumResponse::new(
            0_i16,
            Some(String::from("")),
            Vec::<TopicData>::new(),
            Vec::<Node>::new(),
        );
        assert_eq!(d, DescribeQuorumResponse::default());
    }
}

impl Readable for DescribeQuorumResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let topics = read_array::<TopicData>(input, "topics", true)?;
        let nodes = read_array::<Node>(input, "nodes", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeQuorumResponse {
            error_code, error_message, topics, nodes, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeQuorumResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_array(output, "self.nodes", &self.nodes, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicData, version 2.
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

/// PartitionData, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index.
    pub partition_index: i32,
    /// 
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The ID of the current leader or -1 if the leader is unknown.
    pub leader_id: i32,
    /// The latest known leader epoch
    pub leader_epoch: i32,
    /// 
    pub high_watermark: i64,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub current_voters: Vec<ReplicaState>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub observers: Vec<ReplicaState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            leader_id: 0_i32,
            leader_epoch: 0_i32,
            high_watermark: 0_i64,
            current_voters: Vec::<ReplicaState>::new(),
            observers: Vec::<ReplicaState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new<S1: AsRef<str>>(partition_index: i32, error_code: i16, error_message: Option<S1>, leader_id: i32, leader_epoch: i32, high_watermark: i64, current_voters: Vec<ReplicaState>, observers: Vec<ReplicaState>) -> Self {
        Self {
            partition_index,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            leader_id,
            leader_epoch,
            high_watermark,
            current_voters,
            observers,
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
            Some(String::from("")),
            0_i32,
            0_i32,
            0_i64,
            Vec::<ReplicaState>::new(),
            Vec::<ReplicaState>::new(),
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let high_watermark = i64::read(input)?;
        let current_voters = read_array::<ReplicaState>(input, "current_voters", true)?;
        let observers = read_array::<ReplicaState>(input, "observers", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition_index, error_code, error_message, leader_id, leader_epoch, high_watermark, current_voters, observers, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        self.high_watermark.write(output)?;
        write_array(output, "self.current_voters", &self.current_voters, true)?;
        write_array(output, "self.observers", &self.observers, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Node, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Node {
    /// The ID of the associated node
    pub node_id: i32,
    /// The listeners of this controller
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub listeners: Vec<Listener>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            node_id: 0_i32,
            listeners: Vec::<Listener>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Node {
    pub fn new(node_id: i32, listeners: Vec<Listener>) -> Self {
        Self {
            node_id,
            listeners,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_node_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Node::new(
            0_i32,
            Vec::<Listener>::new(),
        );
        assert_eq!(d, Node::default());
    }
}

impl Readable for Node {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let node_id = i32::read(input)?;
        let listeners = read_array::<Listener>(input, "listeners", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Node {
            node_id, listeners, _unknown_tagged_fields
        })
    }
}

impl Writable for Node {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.node_id.write(output)?;
        write_array(output, "self.listeners", &self.listeners, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Listener, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Listener {
    /// The name of the endpoint
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The hostname
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The port
    pub port: u16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            name: String::from(""),
            host: String::from(""),
            port: 0_u16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Listener {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, host: S2, port: u16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            host: host.as_ref().to_string(),
            port,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_listener_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Listener::new(
            String::from(""),
            String::from(""),
            0_u16,
        );
        assert_eq!(d, Listener::default());
    }
}

impl Readable for Listener {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let host = String::read_ext(input, "host", true)?;
        let port = u16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Listener {
            name, host, port, _unknown_tagged_fields
        })
    }
}

impl Writable for Listener {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ReplicaState, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReplicaState {
    /// 
    pub replica_id: i32,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub replica_directory_id: Uuid,
    /// The last known log end offset of the follower or -1 if it is unknown
    pub log_end_offset: i64,
    /// The last known leader wall clock time time when a follower fetched from the leader. This is reported as -1 both for the current leader or if it is unknown for a voter
    pub last_fetch_timestamp: i64,
    /// The leader wall clock append time of the offset for which the follower made the most recent fetch request. This is reported as the current time for the leader and -1 if unknown for a voter
    pub last_caught_up_timestamp: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ReplicaState {
    fn default() -> Self {
        ReplicaState {
            replica_id: 0_i32,
            replica_directory_id: Uuid::nil(),
            log_end_offset: 0_i64,
            last_fetch_timestamp: -1_i64,
            last_caught_up_timestamp: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReplicaState {
    pub fn new(replica_id: i32, replica_directory_id: Uuid, log_end_offset: i64, last_fetch_timestamp: i64, last_caught_up_timestamp: i64) -> Self {
        Self {
            replica_id,
            replica_directory_id,
            log_end_offset,
            last_fetch_timestamp,
            last_caught_up_timestamp,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_replica_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReplicaState::new(
            0_i32,
            Uuid::nil(),
            0_i64,
            -1_i64,
            -1_i64,
        );
        assert_eq!(d, ReplicaState::default());
    }
}

impl Readable for ReplicaState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let replica_id = i32::read(input)?;
        let replica_directory_id = Uuid::read(input)?;
        let log_end_offset = i64::read(input)?;
        let last_fetch_timestamp = i64::read(input)?;
        let last_caught_up_timestamp = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReplicaState {
            replica_id, replica_directory_id, log_end_offset, last_fetch_timestamp, last_caught_up_timestamp, _unknown_tagged_fields
        })
    }
}

impl Writable for ReplicaState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        self.replica_directory_id.write(output)?;
        self.log_end_offset.write(output)?;
        self.last_fetch_timestamp.write(output)?;
        self.last_caught_up_timestamp.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeQuorumResponse>("DescribeQuorumResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeQuorumResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeQuorumResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeQuorumResponse", 2);
        }
    }
}
