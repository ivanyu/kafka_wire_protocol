// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeTopicPartitionsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeTopicPartitionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each topic in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<DescribeTopicPartitionsResponseTopic>,
    /// The next topic and partition index to fetch details for.
    pub next_cursor: Option<Cursor>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeTopicPartitionsResponse {
    fn api_key(&self) -> i16 {
        75
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DescribeTopicPartitionsResponse { }

impl Default for DescribeTopicPartitionsResponse {
    fn default() -> Self {
        DescribeTopicPartitionsResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<DescribeTopicPartitionsResponseTopic>::new(),
            next_cursor: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeTopicPartitionsResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<DescribeTopicPartitionsResponseTopic>, next_cursor: Option<Cursor>) -> Self {
        Self {
            throttle_time_ms,
            topics,
            next_cursor,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_topic_partitions_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeTopicPartitionsResponse::new(
            0_i32,
            Vec::<DescribeTopicPartitionsResponseTopic>::new(),
            None::<Cursor>,
        );
        assert_eq!(d, DescribeTopicPartitionsResponse::default());
    }
}

impl Readable for DescribeTopicPartitionsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<DescribeTopicPartitionsResponseTopic>(input, "topics", true)?;
        let next_cursor = (if i8::read(input)? < 0 { Ok(None) } else { Cursor::read(input).map(Some) })?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeTopicPartitionsResponse {
            throttle_time_ms, topics, next_cursor, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeTopicPartitionsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        (if let Some(v) = &self.next_cursor { 1_i8.write(output)?; v.write(output) } else { (-1_i8).write(output) })?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeTopicPartitionsResponseTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeTopicPartitionsResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: i16,
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub name: Option<String>,
    /// The topic id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// True if the topic is internal.
    pub is_internal: bool,
    /// Each partition in the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<DescribeTopicPartitionsResponsePartition>,
    /// 32-bit bitfield to represent authorized operations for this topic.
    pub topic_authorized_operations: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeTopicPartitionsResponseTopic {
    fn default() -> Self {
        DescribeTopicPartitionsResponseTopic {
            error_code: 0_i16,
            name: Some(String::from("")),
            topic_id: Uuid::nil(),
            is_internal: false,
            partitions: Vec::<DescribeTopicPartitionsResponsePartition>::new(),
            topic_authorized_operations: -2147483648_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeTopicPartitionsResponseTopic {
    pub fn new<S1: AsRef<str>>(error_code: i16, name: Option<S1>, topic_id: Uuid, is_internal: bool, partitions: Vec<DescribeTopicPartitionsResponsePartition>, topic_authorized_operations: i32) -> Self {
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
mod tests_describe_topic_partitions_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeTopicPartitionsResponseTopic::new(
            0_i16,
            Some(String::from("")),
            Uuid::nil(),
            false,
            Vec::<DescribeTopicPartitionsResponsePartition>::new(),
            -2147483648_i32,
        );
        assert_eq!(d, DescribeTopicPartitionsResponseTopic::default());
    }
}

impl Readable for DescribeTopicPartitionsResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let name = Option::<String>::read_ext(input, "name", true)?;
        let topic_id = Uuid::read(input)?;
        let is_internal = bool::read(input)?;
        let partitions = read_array::<DescribeTopicPartitionsResponsePartition>(input, "partitions", true)?;
        let topic_authorized_operations = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeTopicPartitionsResponseTopic {
            error_code, name, topic_id, is_internal, partitions, topic_authorized_operations, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeTopicPartitionsResponseTopic {
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

/// DescribeTopicPartitionsResponsePartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeTopicPartitionsResponsePartition {
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
    /// The new eligible leader replicas otherwise.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub eligible_leader_replicas: Option<Vec<i32>>,
    /// The last known ELR.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub last_known_elr: Option<Vec<i32>>,
    /// The set of offline replicas of this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub offline_replicas: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeTopicPartitionsResponsePartition {
    fn default() -> Self {
        DescribeTopicPartitionsResponsePartition {
            error_code: 0_i16,
            partition_index: 0_i32,
            leader_id: 0_i32,
            leader_epoch: -1_i32,
            replica_nodes: Vec::<i32>::new(),
            isr_nodes: Vec::<i32>::new(),
            eligible_leader_replicas: None,
            last_known_elr: None,
            offline_replicas: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeTopicPartitionsResponsePartition {
    pub fn new(error_code: i16, partition_index: i32, leader_id: i32, leader_epoch: i32, replica_nodes: Vec<i32>, isr_nodes: Vec<i32>, eligible_leader_replicas: Option<Vec<i32>>, last_known_elr: Option<Vec<i32>>, offline_replicas: Vec<i32>) -> Self {
        Self {
            error_code,
            partition_index,
            leader_id,
            leader_epoch,
            replica_nodes,
            isr_nodes,
            eligible_leader_replicas,
            last_known_elr,
            offline_replicas,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_topic_partitions_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeTopicPartitionsResponsePartition::new(
            0_i16,
            0_i32,
            0_i32,
            -1_i32,
            Vec::<i32>::new(),
            Vec::<i32>::new(),
            None::<Vec::<i32>>,
            None::<Vec::<i32>>,
            Vec::<i32>::new(),
        );
        assert_eq!(d, DescribeTopicPartitionsResponsePartition::default());
    }
}

impl Readable for DescribeTopicPartitionsResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let partition_index = i32::read(input)?;
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let replica_nodes = read_array::<i32>(input, "replica_nodes", true)?;
        let isr_nodes = read_array::<i32>(input, "isr_nodes", true)?;
        let eligible_leader_replicas = read_nullable_array::<i32>(input, "eligible_leader_replicas", true)?;
        let last_known_elr = read_nullable_array::<i32>(input, "last_known_elr", true)?;
        let offline_replicas = read_array::<i32>(input, "offline_replicas", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeTopicPartitionsResponsePartition {
            error_code, partition_index, leader_id, leader_epoch, replica_nodes, isr_nodes, eligible_leader_replicas, last_known_elr, offline_replicas, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeTopicPartitionsResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.partition_index.write(output)?;
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.replica_nodes", &self.replica_nodes, true)?;
        write_array(output, "self.isr_nodes", &self.isr_nodes, true)?;
        write_nullable_array(output, "self.eligible_leader_replicas", self.eligible_leader_replicas.as_deref(), true)?;
        write_nullable_array(output, "self.last_known_elr", self.last_known_elr.as_deref(), true)?;
        write_array(output, "self.offline_replicas", &self.offline_replicas, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Cursor, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Cursor {
    /// The name for the first topic to process
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partition index to start with
    pub partition_index: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            topic_name: String::from(""),
            partition_index: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Cursor {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_cursor_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Cursor::new(
            String::from(""),
            0_i32,
        );
        assert_eq!(d, Cursor::default());
    }
}

impl Readable for Cursor {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partition_index = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Cursor {
            topic_name, partition_index, _unknown_tagged_fields
        })
    }
}

impl Writable for Cursor {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        self.partition_index.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeTopicPartitionsResponse>("DescribeTopicPartitionsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeTopicPartitionsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeTopicPartitionsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeTopicPartitionsResponse", 0);
        }
    }
}
