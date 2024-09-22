// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// CreateTopicsRequest, version 7.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateTopicsRequest {
    /// The topics to create.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<CreatableTopic>,
    /// How long to wait in milliseconds before timing out the request.
    pub timeout_ms: i32,
    /// If true, check that the topics can be created as specified, but don't create anything.
    pub validate_only: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreateTopicsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        19
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        7
    }
}

impl Request for CreateTopicsRequest { }

impl Default for CreateTopicsRequest {
    fn default() -> Self {
        CreateTopicsRequest {
            topics: Vec::<CreatableTopic>::new(),
            timeout_ms: 60000_i32,
            validate_only: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreateTopicsRequest {
    pub fn new(topics: Vec<CreatableTopic>, timeout_ms: i32, validate_only: bool) -> Self {
        Self {
            topics,
            timeout_ms,
            validate_only,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_create_topics_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateTopicsRequest::new(
            Vec::<CreatableTopic>::new(),
            60000_i32,
            false,
        );
        assert_eq!(d, CreateTopicsRequest::default());
    }
}

impl Readable for CreateTopicsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<CreatableTopic>(input, "topics", true)?;
        let timeout_ms = i32::read(input)?;
        let validate_only = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreateTopicsRequest {
            topics, timeout_ms, validate_only, _unknown_tagged_fields
        })
    }
}

impl Writable for CreateTopicsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, true)?;
        self.timeout_ms.write(output)?;
        self.validate_only.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// CreatableTopic, version 7.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatableTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The number of partitions to create in the topic, or -1 if we are either specifying a manual partition assignment or using the default partitions.
    pub num_partitions: i32,
    /// The number of replicas to create for each partition in the topic, or -1 if we are either specifying a manual partition assignment or using the default replication factor.
    pub replication_factor: i16,
    /// The manual partition assignment, or the empty array if we are using automatic assignment.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub assignments: Vec<CreatableReplicaAssignment>,
    /// The custom topic configurations to set.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub configs: Vec<CreateableTopicConfig>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for CreatableTopic {
    fn default() -> Self {
        CreatableTopic {
            name: String::from(""),
            num_partitions: 0_i32,
            replication_factor: 0_i16,
            assignments: Vec::<CreatableReplicaAssignment>::new(),
            configs: Vec::<CreateableTopicConfig>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatableTopic {
    pub fn new<S1: AsRef<str>>(name: S1, num_partitions: i32, replication_factor: i16, assignments: Vec<CreatableReplicaAssignment>, configs: Vec<CreateableTopicConfig>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            num_partitions,
            replication_factor,
            assignments,
            configs,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_creatable_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatableTopic::new(
            String::from(""),
            0_i32,
            0_i16,
            Vec::<CreatableReplicaAssignment>::new(),
            Vec::<CreateableTopicConfig>::new(),
        );
        assert_eq!(d, CreatableTopic::default());
    }
}

impl Readable for CreatableTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let num_partitions = i32::read(input)?;
        let replication_factor = i16::read(input)?;
        let assignments = read_array::<CreatableReplicaAssignment>(input, "assignments", true)?;
        let configs = read_array::<CreateableTopicConfig>(input, "configs", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatableTopic {
            name, num_partitions, replication_factor, assignments, configs, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatableTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.num_partitions.write(output)?;
        self.replication_factor.write(output)?;
        write_array(output, "self.assignments", &self.assignments, true)?;
        write_array(output, "self.configs", &self.configs, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// CreatableReplicaAssignment, version 7.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatableReplicaAssignment {
    /// The partition index.
    pub partition_index: i32,
    /// The brokers to place the partition on.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub broker_ids: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for CreatableReplicaAssignment {
    fn default() -> Self {
        CreatableReplicaAssignment {
            partition_index: 0_i32,
            broker_ids: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatableReplicaAssignment {
    pub fn new(partition_index: i32, broker_ids: Vec<i32>) -> Self {
        Self {
            partition_index,
            broker_ids,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_creatable_replica_assignment_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatableReplicaAssignment::new(
            0_i32,
            Vec::<i32>::new(),
        );
        assert_eq!(d, CreatableReplicaAssignment::default());
    }
}

impl Readable for CreatableReplicaAssignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let broker_ids = read_array::<i32>(input, "broker_ids", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatableReplicaAssignment {
            partition_index, broker_ids, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatableReplicaAssignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        write_array(output, "self.broker_ids", &self.broker_ids, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// CreateableTopicConfig, version 7.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateableTopicConfig {
    /// The configuration name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The configuration value.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub value: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for CreateableTopicConfig {
    fn default() -> Self {
        CreateableTopicConfig {
            name: String::from(""),
            value: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreateableTopicConfig {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, value: Option<S2>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value: value.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_createable_topic_config_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateableTopicConfig::new(
            String::from(""),
            Some(String::from("")),
        );
        assert_eq!(d, CreateableTopicConfig::default());
    }
}

impl Readable for CreateableTopicConfig {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let value = Option::<String>::read_ext(input, "value", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreateableTopicConfig {
            name, value, _unknown_tagged_fields
        })
    }
}

impl Writable for CreateableTopicConfig {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.value.write_ext(output, "self.value", true)?;
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
        crate::test_utils::test_java_default::<CreateTopicsRequest>("CreateTopicsRequest", 7);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreateTopicsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreateTopicsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "CreateTopicsRequest", 7);
        }
    }
}
