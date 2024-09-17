// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListOffsetsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsRequest {
    /// The broker ID of the requester, or -1 if this request is being made by a normal consumer.
    pub replica_id: i32,
    /// Each topic in the request.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<ListOffsetsTopic>,
}

impl ApiMessage for ListOffsetsRequest {
    fn api_key(&self) -> i16 {
        2
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ListOffsetsRequest { }

impl Default for ListOffsetsRequest {
    fn default() -> Self {
        ListOffsetsRequest {
            replica_id: 0_i32,
            topics: Vec::<ListOffsetsTopic>::new(),
        }
    }
}

impl ListOffsetsRequest {
    pub fn new(replica_id: i32, topics: Vec<ListOffsetsTopic>) -> Self {
        Self {
            replica_id,
            topics,
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
            Vec::<ListOffsetsTopic>::new(),
        );
        assert_eq!(d, ListOffsetsRequest::default());
    }
}

impl Readable for ListOffsetsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let replica_id = i32::read(input)?;
        let topics = read_array::<ListOffsetsTopic>(input, "topics", false)?;
        Ok(ListOffsetsRequest {
            replica_id, topics
        })
    }
}

impl Writable for ListOffsetsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// ListOffsetsTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition in the request.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<ListOffsetsPartition>,
}

impl Default for ListOffsetsTopic {
    fn default() -> Self {
        ListOffsetsTopic {
            name: String::from(""),
            partitions: Vec::<ListOffsetsPartition>::new(),
        }
    }
}

impl ListOffsetsTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<ListOffsetsPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
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
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<ListOffsetsPartition>(input, "partitions", false)?;
        Ok(ListOffsetsTopic {
            name, partitions
        })
    }
}

impl Writable for ListOffsetsTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// ListOffsetsPartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The current timestamp.
    pub timestamp: i64,
    /// The maximum number of offsets to report.
    pub max_num_offsets: i32,
}

impl Default for ListOffsetsPartition {
    fn default() -> Self {
        ListOffsetsPartition {
            partition_index: 0_i32,
            timestamp: 0_i64,
            max_num_offsets: 1_i32,
        }
    }
}

impl ListOffsetsPartition {
    pub fn new(partition_index: i32, timestamp: i64, max_num_offsets: i32) -> Self {
        Self {
            partition_index,
            timestamp,
            max_num_offsets,
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
            0_i64,
            1_i32,
        );
        assert_eq!(d, ListOffsetsPartition::default());
    }
}

impl Readable for ListOffsetsPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let timestamp = i64::read(input)?;
        let max_num_offsets = i32::read(input)?;
        Ok(ListOffsetsPartition {
            partition_index, timestamp, max_num_offsets
        })
    }
}

impl Writable for ListOffsetsPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.timestamp.write(output)?;
        self.max_num_offsets.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ListOffsetsRequest>("ListOffsetsRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "ListOffsetsRequest", 0);
        }
    }
}
