// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListOffsetsRequest, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsRequest {
    /// The broker ID of the requester, or -1 if this request is being made by a normal consumer.
    pub replica_id: i32,
    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records.
    pub isolation_level: i8,
    /// Each topic in the request.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<ListOffsetsTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListOffsetsRequest {
    fn api_key(&self) -> i16 {
        2
    }
    
    fn version(&self) -> i16 {
        6
    }
}

impl Request for ListOffsetsRequest { }

impl Default for ListOffsetsRequest {
    fn default() -> Self {
        ListOffsetsRequest {
            replica_id: 0_i32,
            isolation_level: 0_i8,
            topics: Vec::<ListOffsetsTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListOffsetsRequest {
    pub fn new(replica_id: i32, isolation_level: i8, topics: Vec<ListOffsetsTopic>) -> Self {
        Self {
            replica_id,
            isolation_level,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_offsets_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListOffsetsRequest::new(
            0_i32,
            0_i8,
            Vec::<ListOffsetsTopic>::new(),
        );
        assert_eq!(d, ListOffsetsRequest::default());
    }
}

impl Readable for ListOffsetsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let replica_id = i32::read(input)?;
        let isolation_level = i8::read(input)?;
        let topics = read_array::<ListOffsetsTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListOffsetsRequest {
            replica_id, isolation_level, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for ListOffsetsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        self.isolation_level.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ListOffsetsTopic, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition in the request.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<ListOffsetsPartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ListOffsetsTopic {
    fn default() -> Self {
        ListOffsetsTopic {
            name: String::from(""),
            partitions: Vec::<ListOffsetsPartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListOffsetsTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<ListOffsetsPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_offsets_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListOffsetsTopic::new(
            String::from(""),
            Vec::<ListOffsetsPartition>::new(),
        );
        assert_eq!(d, ListOffsetsTopic::default());
    }
}

impl Readable for ListOffsetsTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<ListOffsetsPartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListOffsetsTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for ListOffsetsTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ListOffsetsPartition, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The current leader epoch.
    pub current_leader_epoch: i32,
    /// The current timestamp.
    pub timestamp: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ListOffsetsPartition {
    fn default() -> Self {
        ListOffsetsPartition {
            partition_index: 0_i32,
            current_leader_epoch: -1_i32,
            timestamp: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListOffsetsPartition {
    pub fn new(partition_index: i32, current_leader_epoch: i32, timestamp: i64) -> Self {
        Self {
            partition_index,
            current_leader_epoch,
            timestamp,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_offsets_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListOffsetsPartition::new(
            0_i32,
            -1_i32,
            0_i64,
        );
        assert_eq!(d, ListOffsetsPartition::default());
    }
}

impl Readable for ListOffsetsPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let current_leader_epoch = i32::read(input)?;
        let timestamp = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListOffsetsPartition {
            partition_index, current_leader_epoch, timestamp, _unknown_tagged_fields
        })
    }
}

impl Writable for ListOffsetsPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.current_leader_epoch.write(output)?;
        self.timestamp.write(output)?;
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
        crate::test_utils::test_java_default::<ListOffsetsRequest>("ListOffsetsRequest", 6);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListOffsetsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListOffsetsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ListOffsetsRequest", 6);
        }
    }
}
