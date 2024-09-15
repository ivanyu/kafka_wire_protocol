// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchRequest {
    /// The broker ID of the follower, of -1 if this request is from a consumer.
    pub replica_id: i32,
    /// The maximum time in milliseconds to wait for the response.
    pub max_wait_ms: i32,
    /// The minimum bytes to accumulate in the response.
    pub min_bytes: i32,
    /// The maximum bytes to fetch.  See KIP-74 for cases where this limit may not be honored.
    pub max_bytes: i32,
    /// This setting controls the visibility of transactional records. Using READ_UNCOMMITTED (isolation_level = 0) makes all records visible. With READ_COMMITTED (isolation_level = 1), non-transactional and COMMITTED transactional records are visible. To be more concrete, READ_COMMITTED returns all data from offsets smaller than the current LSO (last stable offset), and enables the inclusion of the list of aborted transactions in the result, which allows consumers to discard ABORTED transactional records
    pub isolation_level: i8,
    /// The fetch session ID.
    pub session_id: i32,
    /// The fetch session epoch, which is used for ordering requests in a session.
    pub session_epoch: i32,
    /// The topics to fetch.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<FetchTopic>,
    /// In an incremental fetch request, the partitions to remove.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub forgotten_topics_data: Vec<ForgottenTopic>,
}

impl ApiMessage for FetchRequest {
    fn api_key(&self) -> i16 {
        1
    }
    
    fn version(&self) -> i16 {
        8
    }
}

impl Request for FetchRequest { }

impl Default for FetchRequest {
    fn default() -> Self {
        FetchRequest {
            replica_id: -1_i32,
            max_wait_ms: 0_i32,
            min_bytes: 0_i32,
            max_bytes: 0x7fffffff_i32,
            isolation_level: 0_i8,
            session_id: 0_i32,
            session_epoch: -1_i32,
            topics: Vec::<FetchTopic>::new(),
            forgotten_topics_data: Vec::<ForgottenTopic>::new(),
        }
    }
}

impl FetchRequest {
    pub fn new(replica_id: i32, max_wait_ms: i32, min_bytes: i32, max_bytes: i32, isolation_level: i8, session_id: i32, session_epoch: i32, topics: Vec<FetchTopic>, forgotten_topics_data: Vec<ForgottenTopic>) -> Self {
        Self {
            replica_id,
            max_wait_ms,
            min_bytes,
            max_bytes,
            isolation_level,
            session_id,
            session_epoch,
            topics,
            forgotten_topics_data,
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
            0x7fffffff_i32,
            0_i8,
            0_i32,
            -1_i32,
            Vec::<FetchTopic>::new(),
            Vec::<ForgottenTopic>::new(),
        );
        assert_eq!(d, FetchRequest::default());
    }
}

impl Readable for FetchRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let replica_id = i32::read(input)?;
        let max_wait_ms = i32::read(input)?;
        let min_bytes = i32::read(input)?;
        let max_bytes = i32::read(input)?;
        let isolation_level = i8::read(input)?;
        let session_id = i32::read(input)?;
        let session_epoch = i32::read(input)?;
        let topics = read_array::<FetchTopic>(input, "topics", false)?;
        let forgotten_topics_data = read_array::<ForgottenTopic>(input, "forgotten_topics_data", false)?;
        Ok(FetchRequest {
            replica_id, max_wait_ms, min_bytes, max_bytes, isolation_level, session_id, session_epoch, topics, forgotten_topics_data
        })
    }
}

impl Writable for FetchRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        self.max_wait_ms.write(output)?;
        self.min_bytes.write(output)?;
        self.max_bytes.write(output)?;
        self.isolation_level.write(output)?;
        self.session_id.write(output)?;
        self.session_epoch.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        write_array(output, "self.forgotten_topics_data", &self.forgotten_topics_data, false)?;
        Ok(())
    }
}

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

impl ApiMessage for FetchTopic {
    fn api_key(&self) -> i16 {
        1
    }
    
    fn version(&self) -> i16 {
        8
    }
}

impl Request for FetchTopic { }

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchPartition {
    /// The partition index.
    pub partition: i32,
    /// The message offset.
    pub fetch_offset: i64,
    /// The earliest available offset of the follower replica.  The field is only used when the request is sent by the follower.
    pub log_start_offset: i64,
    /// The maximum bytes to fetch from this partition.  See KIP-74 for cases where this limit may not be honored.
    pub partition_max_bytes: i32,
}

impl ApiMessage for FetchPartition {
    fn api_key(&self) -> i16 {
        1
    }
    
    fn version(&self) -> i16 {
        8
    }
}

impl Request for FetchPartition { }

impl Default for FetchPartition {
    fn default() -> Self {
        FetchPartition {
            partition: 0_i32,
            fetch_offset: 0_i64,
            log_start_offset: -1_i64,
            partition_max_bytes: 0_i32,
        }
    }
}

impl FetchPartition {
    pub fn new(partition: i32, fetch_offset: i64, log_start_offset: i64, partition_max_bytes: i32) -> Self {
        Self {
            partition,
            fetch_offset,
            log_start_offset,
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
            -1_i64,
            0_i32,
        );
        assert_eq!(d, FetchPartition::default());
    }
}

impl Readable for FetchPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let fetch_offset = i64::read(input)?;
        let log_start_offset = i64::read(input)?;
        let partition_max_bytes = i32::read(input)?;
        Ok(FetchPartition {
            partition, fetch_offset, log_start_offset, partition_max_bytes
        })
    }
}

impl Writable for FetchPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.fetch_offset.write(output)?;
        self.log_start_offset.write(output)?;
        self.partition_max_bytes.write(output)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ForgottenTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// The partitions indexes to forget.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
}

impl ApiMessage for ForgottenTopic {
    fn api_key(&self) -> i16 {
        1
    }
    
    fn version(&self) -> i16 {
        8
    }
}

impl Request for ForgottenTopic { }

impl Default for ForgottenTopic {
    fn default() -> Self {
        ForgottenTopic {
            topic: String::from(""),
            partitions: Vec::<i32>::new(),
        }
    }
}

impl ForgottenTopic {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_forgotten_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ForgottenTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, ForgottenTopic::default());
    }
}

impl Readable for ForgottenTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", false)?;
        let partitions = read_array::<i32>(input, "partitions", false)?;
        Ok(ForgottenTopic {
            topic, partitions
        })
    }
}

impl Writable for ForgottenTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<FetchRequest>("FetchRequest", 8);
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
            crate::test_utils::test_java_arbitrary(&data, "FetchRequest", 8);
        }
    }
}
