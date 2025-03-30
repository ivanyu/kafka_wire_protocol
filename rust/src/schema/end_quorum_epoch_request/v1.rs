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

/// EndQuorumEpochRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EndQuorumEpochRequest {
    /// The cluster id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub cluster_id: Option<String>,
    /// The topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
    /// Endpoints for the leader.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub leader_endpoints: Vec<LeaderEndpoint>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EndQuorumEpochRequest {
    fn api_key(&self) -> i16 {
        54
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for EndQuorumEpochRequest { }

impl Default for EndQuorumEpochRequest {
    fn default() -> Self {
        EndQuorumEpochRequest {
            cluster_id: None,
            topics: Vec::<TopicData>::new(),
            leader_endpoints: Vec::<LeaderEndpoint>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl EndQuorumEpochRequest {
    pub fn new<S1: AsRef<str>>(cluster_id: Option<S1>, topics: Vec<TopicData>, leader_endpoints: Vec<LeaderEndpoint>) -> Self {
        Self {
            cluster_id: cluster_id.map(|s| s.as_ref().to_string()),
            topics,
            leader_endpoints,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_end_quorum_epoch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EndQuorumEpochRequest::new(
            None::<String>,
            Vec::<TopicData>::new(),
            Vec::<LeaderEndpoint>::new(),
        );
        assert_eq!(d, EndQuorumEpochRequest::default());
    }
}

impl Readable for EndQuorumEpochRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let cluster_id = Option::<String>::read_ext(input, "cluster_id", true)?;
        let topics = read_array::<TopicData>(input, "topics", true)?;
        let leader_endpoints = read_array::<LeaderEndpoint>(input, "leader_endpoints", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(EndQuorumEpochRequest {
            cluster_id, topics, leader_endpoints, _unknown_tagged_fields
        })
    }
}

impl Writable for EndQuorumEpochRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_array(output, "self.leader_endpoints", &self.leader_endpoints, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
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
    /// The partitions.
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
    /// The current leader ID that is resigning.
    pub leader_id: i32,
    /// The current epoch.
    pub leader_epoch: i32,
    /// A sorted list of preferred candidates to start the election.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub preferred_candidates: Vec<ReplicaInfo>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            leader_id: 0_i32,
            leader_epoch: 0_i32,
            preferred_candidates: Vec::<ReplicaInfo>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new(partition_index: i32, leader_id: i32, leader_epoch: i32, preferred_candidates: Vec<ReplicaInfo>) -> Self {
        Self {
            partition_index,
            leader_id,
            leader_epoch,
            preferred_candidates,
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
            0_i32,
            0_i32,
            Vec::<ReplicaInfo>::new(),
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let preferred_candidates = read_array::<ReplicaInfo>(input, "preferred_candidates", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition_index, leader_id, leader_epoch, preferred_candidates, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.preferred_candidates", &self.preferred_candidates, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ReplicaInfo, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReplicaInfo {
    /// The ID of the candidate replica.
    pub candidate_id: i32,
    /// The directory ID of the candidate replica.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub candidate_directory_id: Uuid,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ReplicaInfo {
    fn default() -> Self {
        ReplicaInfo {
            candidate_id: 0_i32,
            candidate_directory_id: Uuid::nil(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReplicaInfo {
    pub fn new(candidate_id: i32, candidate_directory_id: Uuid) -> Self {
        Self {
            candidate_id,
            candidate_directory_id,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_replica_info_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReplicaInfo::new(
            0_i32,
            Uuid::nil(),
        );
        assert_eq!(d, ReplicaInfo::default());
    }
}

impl Readable for ReplicaInfo {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let candidate_id = i32::read(input)?;
        let candidate_directory_id = Uuid::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReplicaInfo {
            candidate_id, candidate_directory_id, _unknown_tagged_fields
        })
    }
}

impl Writable for ReplicaInfo {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.candidate_id.write(output)?;
        self.candidate_directory_id.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// LeaderEndpoint, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderEndpoint {
    /// The name of the endpoint.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The node's hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The node's port.
    pub port: u16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for LeaderEndpoint {
    fn default() -> Self {
        LeaderEndpoint {
            name: String::from(""),
            host: String::from(""),
            port: 0_u16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderEndpoint {
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
mod tests_leader_endpoint_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderEndpoint::new(
            String::from(""),
            String::from(""),
            0_u16,
        );
        assert_eq!(d, LeaderEndpoint::default());
    }
}

impl Readable for LeaderEndpoint {
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
        Ok(LeaderEndpoint {
            name, host, port, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderEndpoint {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
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
        crate::test_utils::test_java_default::<EndQuorumEpochRequest>("EndQuorumEpochRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: EndQuorumEpochRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: EndQuorumEpochRequest) {
            crate::test_utils::test_java_arbitrary(&data, "EndQuorumEpochRequest", 1);
        }
    }
}
