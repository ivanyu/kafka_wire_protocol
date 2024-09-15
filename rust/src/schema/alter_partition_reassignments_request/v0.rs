// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterPartitionReassignmentsRequest {
    /// The time in ms to wait for the request to complete.
    pub timeout_ms: i32,
    /// The topics to reassign.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<ReassignableTopic>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterPartitionReassignmentsRequest {
    fn api_key(&self) -> i16 {
        45
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for AlterPartitionReassignmentsRequest { }

impl Default for AlterPartitionReassignmentsRequest {
    fn default() -> Self {
        AlterPartitionReassignmentsRequest {
            timeout_ms: 60000_i32,
            topics: Vec::<ReassignableTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterPartitionReassignmentsRequest {
    pub fn new(timeout_ms: i32, topics: Vec<ReassignableTopic>) -> Self {
        Self {
            timeout_ms,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_partition_reassignments_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterPartitionReassignmentsRequest::new(
            60000_i32,
            Vec::<ReassignableTopic>::new(),
        );
        assert_eq!(d, AlterPartitionReassignmentsRequest::default());
    }
}

impl Readable for AlterPartitionReassignmentsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let timeout_ms = i32::read(input)?;
        let topics = read_array::<ReassignableTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterPartitionReassignmentsRequest {
            timeout_ms, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterPartitionReassignmentsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.timeout_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReassignableTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partitions to reassign.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<ReassignablePartition>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ReassignableTopic {
    fn api_key(&self) -> i16 {
        45
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ReassignableTopic { }

impl Default for ReassignableTopic {
    fn default() -> Self {
        ReassignableTopic {
            name: String::from(""),
            partitions: Vec::<ReassignablePartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReassignableTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<ReassignablePartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_reassignable_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReassignableTopic::new(
            String::from(""),
            Vec::<ReassignablePartition>::new(),
        );
        assert_eq!(d, ReassignableTopic::default());
    }
}

impl Readable for ReassignableTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<ReassignablePartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReassignableTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for ReassignableTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReassignablePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The replicas to place the partitions on, or null to cancel a pending reassignment for this partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub replicas: Option<Vec<i32>>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ReassignablePartition {
    fn api_key(&self) -> i16 {
        45
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ReassignablePartition { }

impl Default for ReassignablePartition {
    fn default() -> Self {
        ReassignablePartition {
            partition_index: 0_i32,
            replicas: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReassignablePartition {
    pub fn new(partition_index: i32, replicas: Option<Vec<i32>>) -> Self {
        Self {
            partition_index,
            replicas,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_reassignable_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReassignablePartition::new(
            0_i32,
            None::<Vec::<i32>>,
        );
        assert_eq!(d, ReassignablePartition::default());
    }
}

impl Readable for ReassignablePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let replicas = read_nullable_array::<i32>(input, "replicas", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReassignablePartition {
            partition_index, replicas, _unknown_tagged_fields
        })
    }
}

impl Writable for ReassignablePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        write_nullable_array(output, "self.replicas", self.replicas.as_deref(), true)?;
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
        crate::test_utils::test_java_default::<AlterPartitionReassignmentsRequest>("AlterPartitionReassignmentsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterPartitionReassignmentsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterPartitionReassignmentsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AlterPartitionReassignmentsRequest", 0);
        }
    }
}
