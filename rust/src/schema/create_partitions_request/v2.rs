// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// CreatePartitionsRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatePartitionsRequest {
    /// Each topic that we want to create new partitions inside.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<CreatePartitionsTopic>,
    /// The time in ms to wait for the partitions to be created.
    pub timeout_ms: i32,
    /// If true, then validate the request, but don't actually increase the number of partitions.
    pub validate_only: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreatePartitionsRequest {
    fn api_key(&self) -> i16 {
        37
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for CreatePartitionsRequest { }

impl Default for CreatePartitionsRequest {
    fn default() -> Self {
        CreatePartitionsRequest {
            topics: Vec::<CreatePartitionsTopic>::new(),
            timeout_ms: 0_i32,
            validate_only: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatePartitionsRequest {
    pub fn new(topics: Vec<CreatePartitionsTopic>, timeout_ms: i32, validate_only: bool) -> Self {
        Self {
            topics,
            timeout_ms,
            validate_only,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_create_partitions_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatePartitionsRequest::new(
            Vec::<CreatePartitionsTopic>::new(),
            0_i32,
            false,
        );
        assert_eq!(d, CreatePartitionsRequest::default());
    }
}

impl Readable for CreatePartitionsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<CreatePartitionsTopic>(input, "topics", true)?;
        let timeout_ms = i32::read(input)?;
        let validate_only = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatePartitionsRequest {
            topics, timeout_ms, validate_only, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatePartitionsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, true)?;
        self.timeout_ms.write(output)?;
        self.validate_only.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// CreatePartitionsTopic, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatePartitionsTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The new partition count.
    pub count: i32,
    /// The new partition assignments.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub assignments: Option<Vec<CreatePartitionsAssignment>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for CreatePartitionsTopic {
    fn default() -> Self {
        CreatePartitionsTopic {
            name: String::from(""),
            count: 0_i32,
            assignments: Some(Vec::<CreatePartitionsAssignment>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatePartitionsTopic {
    pub fn new<S1: AsRef<str>>(name: S1, count: i32, assignments: Option<Vec<CreatePartitionsAssignment>>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            count,
            assignments,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_create_partitions_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatePartitionsTopic::new(
            String::from(""),
            0_i32,
            Some(Vec::<CreatePartitionsAssignment>::new()),
        );
        assert_eq!(d, CreatePartitionsTopic::default());
    }
}

impl Readable for CreatePartitionsTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let count = i32::read(input)?;
        let assignments = read_nullable_array::<CreatePartitionsAssignment>(input, "assignments", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatePartitionsTopic {
            name, count, assignments, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatePartitionsTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.count.write(output)?;
        write_nullable_array(output, "self.assignments", self.assignments.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// CreatePartitionsAssignment, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatePartitionsAssignment {
    /// The assigned broker IDs.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub broker_ids: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for CreatePartitionsAssignment {
    fn default() -> Self {
        CreatePartitionsAssignment {
            broker_ids: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatePartitionsAssignment {
    pub fn new(broker_ids: Vec<i32>) -> Self {
        Self {
            broker_ids,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_create_partitions_assignment_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatePartitionsAssignment::new(
            Vec::<i32>::new(),
        );
        assert_eq!(d, CreatePartitionsAssignment::default());
    }
}

impl Readable for CreatePartitionsAssignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_ids = read_array::<i32>(input, "broker_ids", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatePartitionsAssignment {
            broker_ids, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatePartitionsAssignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.broker_ids", &self.broker_ids, true)?;
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
        crate::test_utils::test_java_default::<CreatePartitionsRequest>("CreatePartitionsRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreatePartitionsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreatePartitionsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "CreatePartitionsRequest", 2);
        }
    }
}
