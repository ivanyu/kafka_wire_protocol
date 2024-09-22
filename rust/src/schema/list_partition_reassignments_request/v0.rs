// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListPartitionReassignmentsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListPartitionReassignmentsRequest {
    /// The time in ms to wait for the request to complete.
    pub timeout_ms: i32,
    /// The topics to list partition reassignments for, or null to list everything.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topics: Option<Vec<ListPartitionReassignmentsTopics>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListPartitionReassignmentsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        46
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ListPartitionReassignmentsRequest { }

impl Default for ListPartitionReassignmentsRequest {
    fn default() -> Self {
        ListPartitionReassignmentsRequest {
            timeout_ms: 60000_i32,
            topics: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListPartitionReassignmentsRequest {
    pub fn new(timeout_ms: i32, topics: Option<Vec<ListPartitionReassignmentsTopics>>) -> Self {
        Self {
            timeout_ms,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_partition_reassignments_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListPartitionReassignmentsRequest::new(
            60000_i32,
            None::<Vec::<ListPartitionReassignmentsTopics>>,
        );
        assert_eq!(d, ListPartitionReassignmentsRequest::default());
    }
}

impl Readable for ListPartitionReassignmentsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let timeout_ms = i32::read(input)?;
        let topics = read_nullable_array::<ListPartitionReassignmentsTopics>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListPartitionReassignmentsRequest {
            timeout_ms, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for ListPartitionReassignmentsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.timeout_ms.write(output)?;
        write_nullable_array(output, "self.topics", self.topics.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ListPartitionReassignmentsTopics, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListPartitionReassignmentsTopics {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partitions to list partition reassignments for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_indexes: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ListPartitionReassignmentsTopics {
    fn default() -> Self {
        ListPartitionReassignmentsTopics {
            name: String::from(""),
            partition_indexes: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListPartitionReassignmentsTopics {
    pub fn new<S1: AsRef<str>>(name: S1, partition_indexes: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_indexes,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_partition_reassignments_topics_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListPartitionReassignmentsTopics::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, ListPartitionReassignmentsTopics::default());
    }
}

impl Readable for ListPartitionReassignmentsTopics {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partition_indexes = read_array::<i32>(input, "partition_indexes", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListPartitionReassignmentsTopics {
            name, partition_indexes, _unknown_tagged_fields
        })
    }
}

impl Writable for ListPartitionReassignmentsTopics {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partition_indexes", &self.partition_indexes, true)?;
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
        crate::test_utils::test_java_default::<ListPartitionReassignmentsRequest>("ListPartitionReassignmentsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListPartitionReassignmentsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListPartitionReassignmentsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ListPartitionReassignmentsRequest", 0);
        }
    }
}
