// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataResponse {
    /// A list of brokers present in the cluster.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub brokers: Vec<MetadataResponseBroker>,
    /// Each topic in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<MetadataResponseTopic>,
}

impl ApiMessage for MetadataResponse {
    fn api_key(&self) -> i16 {
        3
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for MetadataResponse { }

impl Default for MetadataResponse {
    fn default() -> Self {
        MetadataResponse {
            brokers: Vec::<MetadataResponseBroker>::new(),
            topics: Vec::<MetadataResponseTopic>::new(),
        }
    }
}

impl MetadataResponse {
    pub fn new(brokers: Vec<MetadataResponseBroker>, topics: Vec<MetadataResponseTopic>) -> Self {
        Self {
            brokers,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_metadata_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MetadataResponse::new(
            Vec::<MetadataResponseBroker>::new(),
            Vec::<MetadataResponseTopic>::new(),
        );
        assert_eq!(d, MetadataResponse::default());
    }
}

impl Readable for MetadataResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let brokers = read_array::<MetadataResponseBroker>(input, "brokers", false)?;
        let topics = read_array::<MetadataResponseTopic>(input, "topics", false)?;
        Ok(MetadataResponse {
            brokers, topics
        })
    }
}

impl Writable for MetadataResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.brokers", &self.brokers, false)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

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
}

impl ApiMessage for MetadataResponseBroker {
    fn api_key(&self) -> i16 {
        3
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for MetadataResponseBroker { }

impl Default for MetadataResponseBroker {
    fn default() -> Self {
        MetadataResponseBroker {
            node_id: 0_i32,
            host: String::from(""),
            port: 0_i32,
        }
    }
}

impl MetadataResponseBroker {
    pub fn new<S1: AsRef<str>>(node_id: i32, host: S1, port: i32) -> Self {
        Self {
            node_id,
            host: host.as_ref().to_string(),
            port,
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
        );
        assert_eq!(d, MetadataResponseBroker::default());
    }
}

impl Readable for MetadataResponseBroker {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let node_id = i32::read(input)?;
        let host = String::read_ext(input, "host", false)?;
        let port = i32::read(input)?;
        Ok(MetadataResponseBroker {
            node_id, host, port
        })
    }
}

impl Writable for MetadataResponseBroker {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.node_id.write(output)?;
        self.host.write_ext(output, "self.host", false)?;
        self.port.write(output)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: i16,
    /// The topic name. Null for non-existing topics queried by ID. This is never null when ErrorCode is zero. One of Name and TopicId is always populated.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition in the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<MetadataResponsePartition>,
}

impl ApiMessage for MetadataResponseTopic {
    fn api_key(&self) -> i16 {
        3
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for MetadataResponseTopic { }

impl Default for MetadataResponseTopic {
    fn default() -> Self {
        MetadataResponseTopic {
            error_code: 0_i16,
            name: String::from(""),
            partitions: Vec::<MetadataResponsePartition>::new(),
        }
    }
}

impl MetadataResponseTopic {
    pub fn new<S1: AsRef<str>>(error_code: i16, name: S1, partitions: Vec<MetadataResponsePartition>) -> Self {
        Self {
            error_code,
            name: name.as_ref().to_string(),
            partitions,
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
            String::from(""),
            Vec::<MetadataResponsePartition>::new(),
        );
        assert_eq!(d, MetadataResponseTopic::default());
    }
}

impl Readable for MetadataResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<MetadataResponsePartition>(input, "partitions", false)?;
        Ok(MetadataResponseTopic {
            error_code, name, partitions
        })
    }
}

impl Writable for MetadataResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataResponsePartition {
    /// The partition error, or 0 if there was no error.
    pub error_code: i16,
    /// The partition index.
    pub partition_index: i32,
    /// The ID of the leader broker.
    pub leader_id: i32,
    /// The set of all nodes that host this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub replica_nodes: Vec<i32>,
    /// The set of nodes that are in sync with the leader for this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub isr_nodes: Vec<i32>,
}

impl ApiMessage for MetadataResponsePartition {
    fn api_key(&self) -> i16 {
        3
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for MetadataResponsePartition { }

impl Default for MetadataResponsePartition {
    fn default() -> Self {
        MetadataResponsePartition {
            error_code: 0_i16,
            partition_index: 0_i32,
            leader_id: 0_i32,
            replica_nodes: Vec::<i32>::new(),
            isr_nodes: Vec::<i32>::new(),
        }
    }
}

impl MetadataResponsePartition {
    pub fn new(error_code: i16, partition_index: i32, leader_id: i32, replica_nodes: Vec<i32>, isr_nodes: Vec<i32>) -> Self {
        Self {
            error_code,
            partition_index,
            leader_id,
            replica_nodes,
            isr_nodes,
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
        let replica_nodes = read_array::<i32>(input, "replica_nodes", false)?;
        let isr_nodes = read_array::<i32>(input, "isr_nodes", false)?;
        Ok(MetadataResponsePartition {
            error_code, partition_index, leader_id, replica_nodes, isr_nodes
        })
    }
}

impl Writable for MetadataResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.partition_index.write(output)?;
        self.leader_id.write(output)?;
        write_array(output, "self.replica_nodes", &self.replica_nodes, false)?;
        write_array(output, "self.isr_nodes", &self.isr_nodes, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<MetadataResponse>("MetadataResponse", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "MetadataResponse", 0);
        }
    }
}
