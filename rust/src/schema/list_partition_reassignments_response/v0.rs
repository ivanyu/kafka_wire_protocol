// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListPartitionReassignmentsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top-level error code, or 0 if there was no error
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The ongoing reassignments for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OngoingTopicReassignment>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListPartitionReassignmentsResponse {
    fn api_key(&self) -> i16 {
        46
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ListPartitionReassignmentsResponse { }

impl Default for ListPartitionReassignmentsResponse {
    fn default() -> Self {
        ListPartitionReassignmentsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            topics: Vec::<OngoingTopicReassignment>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListPartitionReassignmentsResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, topics: Vec<OngoingTopicReassignment>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_partition_reassignments_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListPartitionReassignmentsResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
            Vec::<OngoingTopicReassignment>::new(),
        );
        assert_eq!(d, ListPartitionReassignmentsResponse::default());
    }
}

impl Readable for ListPartitionReassignmentsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let topics = read_array::<OngoingTopicReassignment>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListPartitionReassignmentsResponse {
            throttle_time_ms, error_code, error_message, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for ListPartitionReassignmentsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OngoingTopicReassignment {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The ongoing reassignments for each partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OngoingPartitionReassignment>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OngoingTopicReassignment {
    fn api_key(&self) -> i16 {
        46
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for OngoingTopicReassignment { }

impl Default for OngoingTopicReassignment {
    fn default() -> Self {
        OngoingTopicReassignment {
            name: String::from(""),
            partitions: Vec::<OngoingPartitionReassignment>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OngoingTopicReassignment {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OngoingPartitionReassignment>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_ongoing_topic_reassignment_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OngoingTopicReassignment::new(
            String::from(""),
            Vec::<OngoingPartitionReassignment>::new(),
        );
        assert_eq!(d, OngoingTopicReassignment::default());
    }
}

impl Readable for OngoingTopicReassignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<OngoingPartitionReassignment>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OngoingTopicReassignment {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for OngoingTopicReassignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OngoingPartitionReassignment {
    /// The index of the partition.
    pub partition_index: i32,
    /// The current replica set.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub replicas: Vec<i32>,
    /// The set of replicas we are currently adding.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub adding_replicas: Vec<i32>,
    /// The set of replicas we are currently removing.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub removing_replicas: Vec<i32>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OngoingPartitionReassignment {
    fn api_key(&self) -> i16 {
        46
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for OngoingPartitionReassignment { }

impl Default for OngoingPartitionReassignment {
    fn default() -> Self {
        OngoingPartitionReassignment {
            partition_index: 0_i32,
            replicas: Vec::<i32>::new(),
            adding_replicas: Vec::<i32>::new(),
            removing_replicas: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OngoingPartitionReassignment {
    pub fn new(partition_index: i32, replicas: Vec<i32>, adding_replicas: Vec<i32>, removing_replicas: Vec<i32>) -> Self {
        Self {
            partition_index,
            replicas,
            adding_replicas,
            removing_replicas,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_ongoing_partition_reassignment_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OngoingPartitionReassignment::new(
            0_i32,
            Vec::<i32>::new(),
            Vec::<i32>::new(),
            Vec::<i32>::new(),
        );
        assert_eq!(d, OngoingPartitionReassignment::default());
    }
}

impl Readable for OngoingPartitionReassignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let replicas = read_array::<i32>(input, "replicas", true)?;
        let adding_replicas = read_array::<i32>(input, "adding_replicas", true)?;
        let removing_replicas = read_array::<i32>(input, "removing_replicas", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OngoingPartitionReassignment {
            partition_index, replicas, adding_replicas, removing_replicas, _unknown_tagged_fields
        })
    }
}

impl Writable for OngoingPartitionReassignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        write_array(output, "self.replicas", &self.replicas, true)?;
        write_array(output, "self.adding_replicas", &self.adding_replicas, true)?;
        write_array(output, "self.removing_replicas", &self.removing_replicas, true)?;
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
        crate::test_utils::test_java_default::<ListPartitionReassignmentsResponse>("ListPartitionReassignmentsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListPartitionReassignmentsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListPartitionReassignmentsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ListPartitionReassignmentsResponse", 0);
        }
    }
}
