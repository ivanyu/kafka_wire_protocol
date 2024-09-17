// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListOffsetsResponse, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each topic in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<ListOffsetsTopicResponse>,
}

impl ApiMessage for ListOffsetsResponse {
    fn api_key(&self) -> i16 {
        2
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Response for ListOffsetsResponse { }

impl Default for ListOffsetsResponse {
    fn default() -> Self {
        ListOffsetsResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<ListOffsetsTopicResponse>::new(),
        }
    }
}

impl ListOffsetsResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<ListOffsetsTopicResponse>) -> Self {
        Self {
            throttle_time_ms,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_list_offsets_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListOffsetsResponse::new(
            0_i32,
            Vec::<ListOffsetsTopicResponse>::new(),
        );
        assert_eq!(d, ListOffsetsResponse::default());
    }
}

impl Readable for ListOffsetsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<ListOffsetsTopicResponse>(input, "topics", false)?;
        Ok(ListOffsetsResponse {
            throttle_time_ms, topics
        })
    }
}

impl Writable for ListOffsetsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// ListOffsetsTopicResponse, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsTopicResponse {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<ListOffsetsPartitionResponse>,
}

impl Default for ListOffsetsTopicResponse {
    fn default() -> Self {
        ListOffsetsTopicResponse {
            name: String::from(""),
            partitions: Vec::<ListOffsetsPartitionResponse>::new(),
        }
    }
}

impl ListOffsetsTopicResponse {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<ListOffsetsPartitionResponse>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_list_offsets_topic_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListOffsetsTopicResponse::new(
            String::from(""),
            Vec::<ListOffsetsPartitionResponse>::new(),
        );
        assert_eq!(d, ListOffsetsTopicResponse::default());
    }
}

impl Readable for ListOffsetsTopicResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<ListOffsetsPartitionResponse>(input, "partitions", false)?;
        Ok(ListOffsetsTopicResponse {
            name, partitions
        })
    }
}

impl Writable for ListOffsetsTopicResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// ListOffsetsPartitionResponse, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListOffsetsPartitionResponse {
    /// The partition index.
    pub partition_index: i32,
    /// The partition error code, or 0 if there was no error.
    pub error_code: i16,
    /// The timestamp associated with the returned offset.
    pub timestamp: i64,
    /// The returned offset.
    pub offset: i64,
    /// 
    pub leader_epoch: i32,
}

impl Default for ListOffsetsPartitionResponse {
    fn default() -> Self {
        ListOffsetsPartitionResponse {
            partition_index: 0_i32,
            error_code: 0_i16,
            timestamp: -1_i64,
            offset: -1_i64,
            leader_epoch: -1_i32,
        }
    }
}

impl ListOffsetsPartitionResponse {
    pub fn new(partition_index: i32, error_code: i16, timestamp: i64, offset: i64, leader_epoch: i32) -> Self {
        Self {
            partition_index,
            error_code,
            timestamp,
            offset,
            leader_epoch,
        }
    }
}

#[cfg(test)]
mod tests_list_offsets_partition_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListOffsetsPartitionResponse::new(
            0_i32,
            0_i16,
            -1_i64,
            -1_i64,
            -1_i32,
        );
        assert_eq!(d, ListOffsetsPartitionResponse::default());
    }
}

impl Readable for ListOffsetsPartitionResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let timestamp = i64::read(input)?;
        let offset = i64::read(input)?;
        let leader_epoch = i32::read(input)?;
        Ok(ListOffsetsPartitionResponse {
            partition_index, error_code, timestamp, offset, leader_epoch
        })
    }
}

impl Writable for ListOffsetsPartitionResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.timestamp.write(output)?;
        self.offset.write(output)?;
        self.leader_epoch.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ListOffsetsResponse>("ListOffsetsResponse", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListOffsetsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListOffsetsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ListOffsetsResponse", 4);
        }
    }
}
