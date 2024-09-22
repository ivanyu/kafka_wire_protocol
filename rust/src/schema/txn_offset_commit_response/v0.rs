// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// TxnOffsetCommitResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TxnOffsetCommitResponseTopic>,
}

impl ApiMessage for TxnOffsetCommitResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        28
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for TxnOffsetCommitResponse { }

impl Default for TxnOffsetCommitResponse {
    fn default() -> Self {
        TxnOffsetCommitResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<TxnOffsetCommitResponseTopic>::new(),
        }
    }
}

impl TxnOffsetCommitResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<TxnOffsetCommitResponseTopic>) -> Self {
        Self {
            throttle_time_ms,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_txn_offset_commit_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TxnOffsetCommitResponse::new(
            0_i32,
            Vec::<TxnOffsetCommitResponseTopic>::new(),
        );
        assert_eq!(d, TxnOffsetCommitResponse::default());
    }
}

impl Readable for TxnOffsetCommitResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<TxnOffsetCommitResponseTopic>(input, "topics", false)?;
        Ok(TxnOffsetCommitResponse {
            throttle_time_ms, topics
        })
    }
}

impl Writable for TxnOffsetCommitResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// TxnOffsetCommitResponseTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The responses for each partition in the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<TxnOffsetCommitResponsePartition>,
}

impl Default for TxnOffsetCommitResponseTopic {
    fn default() -> Self {
        TxnOffsetCommitResponseTopic {
            name: String::from(""),
            partitions: Vec::<TxnOffsetCommitResponsePartition>::new(),
        }
    }
}

impl TxnOffsetCommitResponseTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<TxnOffsetCommitResponsePartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_txn_offset_commit_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TxnOffsetCommitResponseTopic::new(
            String::from(""),
            Vec::<TxnOffsetCommitResponsePartition>::new(),
        );
        assert_eq!(d, TxnOffsetCommitResponseTopic::default());
    }
}

impl Readable for TxnOffsetCommitResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<TxnOffsetCommitResponsePartition>(input, "partitions", false)?;
        Ok(TxnOffsetCommitResponseTopic {
            name, partitions
        })
    }
}

impl Writable for TxnOffsetCommitResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// TxnOffsetCommitResponsePartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TxnOffsetCommitResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl Default for TxnOffsetCommitResponsePartition {
    fn default() -> Self {
        TxnOffsetCommitResponsePartition {
            partition_index: 0_i32,
            error_code: 0_i16,
        }
    }
}

impl TxnOffsetCommitResponsePartition {
    pub fn new(partition_index: i32, error_code: i16) -> Self {
        Self {
            partition_index,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_txn_offset_commit_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TxnOffsetCommitResponsePartition::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, TxnOffsetCommitResponsePartition::default());
    }
}

impl Readable for TxnOffsetCommitResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        Ok(TxnOffsetCommitResponsePartition {
            partition_index, error_code
        })
    }
}

impl Writable for TxnOffsetCommitResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<TxnOffsetCommitResponse>("TxnOffsetCommitResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: TxnOffsetCommitResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: TxnOffsetCommitResponse) {
            crate::test_utils::test_java_arbitrary(&data, "TxnOffsetCommitResponse", 0);
        }
    }
}
