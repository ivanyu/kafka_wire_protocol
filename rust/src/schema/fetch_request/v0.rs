// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// FetchRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    pub replica_id: i32,
    /// The maximum time in milliseconds to wait for the response.
    pub max_wait_ms: i32,
    /// The minimum bytes to accumulate in the response.
    pub min_bytes: i32,
    /// The topics to fetch.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<FetchTopic>,
}

impl ApiMessage for FetchRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        1
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for FetchRequest { }

impl Default for FetchRequest {
    fn default() -> Self {
        FetchRequest {
            replica_id: -1_i32,
            max_wait_ms: 0_i32,
            min_bytes: 0_i32,
            topics: Vec::<FetchTopic>::new(),
        }
    }
}

impl FetchRequest {
    pub fn new(replica_id: i32, max_wait_ms: i32, min_bytes: i32, topics: Vec<FetchTopic>) -> Self {
        Self {
            replica_id,
            max_wait_ms,
            min_bytes,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_fetch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FetchRequest::new(
            -1_i32,
            0_i32,
            0_i32,
            Vec::<FetchTopic>::new(),
        );
        assert_eq!(d, FetchRequest::default());
    }
}

impl Readable for FetchRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let replica_id = i32::read(input)?;
        let max_wait_ms = i32::read(input)?;
        let min_bytes = i32::read(input)?;
        let topics = read_array::<FetchTopic>(input, "topics", false)?;
        Ok(FetchRequest {
            replica_id, max_wait_ms, min_bytes, topics
        })
    }
}

impl Writable for FetchRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        self.max_wait_ms.write(output)?;
        self.min_bytes.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// FetchTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchTopic {
    /// The name of the topic to fetch.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// The partitions to fetch.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<FetchPartition>,
}

impl Default for FetchTopic {
    fn default() -> Self {
        FetchTopic {
            topic: String::from(""),
            partitions: Vec::<FetchPartition>::new(),
        }
    }
}

impl FetchTopic {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<FetchPartition>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_fetch_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FetchTopic::new(
            String::from(""),
            Vec::<FetchPartition>::new(),
        );
        assert_eq!(d, FetchTopic::default());
    }
}

impl Readable for FetchTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", false)?;
        let partitions = read_array::<FetchPartition>(input, "partitions", false)?;
        Ok(FetchTopic {
            topic, partitions
        })
    }
}

impl Writable for FetchTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// FetchPartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchPartition {
    /// The partition index.
    pub partition: i32,
    /// The message offset.
    pub fetch_offset: i64,
    /// The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored.
    pub partition_max_bytes: i32,
}

impl Default for FetchPartition {
    fn default() -> Self {
        FetchPartition {
            partition: 0_i32,
            fetch_offset: 0_i64,
            partition_max_bytes: 0_i32,
        }
    }
}

impl FetchPartition {
    pub fn new(partition: i32, fetch_offset: i64, partition_max_bytes: i32) -> Self {
        Self {
            partition,
            fetch_offset,
            partition_max_bytes,
        }
    }
}

#[cfg(test)]
mod tests_fetch_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FetchPartition::new(
            0_i32,
            0_i64,
            0_i32,
        );
        assert_eq!(d, FetchPartition::default());
    }
}

impl Readable for FetchPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let fetch_offset = i64::read(input)?;
        let partition_max_bytes = i32::read(input)?;
        Ok(FetchPartition {
            partition, fetch_offset, partition_max_bytes
        })
    }
}

impl Writable for FetchPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.fetch_offset.write(output)?;
        self.partition_max_bytes.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<FetchRequest>("FetchRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: FetchRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: FetchRequest) {
            crate::test_utils::test_java_arbitrary(&data, "FetchRequest", 0);
        }
    }
}
