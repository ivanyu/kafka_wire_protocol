// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetCommitResponseTopic>,
}

impl ApiMessage for OffsetCommitResponse {
    fn api_key(&self) -> i16 {
        8
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Response for OffsetCommitResponse { }

impl Default for OffsetCommitResponse {
    fn default() -> Self {
        OffsetCommitResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<OffsetCommitResponseTopic>::new(),
        }
    }
}

impl OffsetCommitResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<OffsetCommitResponseTopic>) -> Self {
        Self {
            throttle_time_ms,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_offset_commit_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetCommitResponse::new(
            0_i32,
            Vec::<OffsetCommitResponseTopic>::new(),
        );
        assert_eq!(d, OffsetCommitResponse::default());
    }
}

impl Readable for OffsetCommitResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<OffsetCommitResponseTopic>(input, "topics", false)?;
        Ok(OffsetCommitResponse {
            throttle_time_ms, topics
        })
    }
}

impl Writable for OffsetCommitResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The responses for each partition in the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetCommitResponsePartition>,
}

impl ApiMessage for OffsetCommitResponseTopic {
    fn api_key(&self) -> i16 {
        8
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Response for OffsetCommitResponseTopic { }

impl Default for OffsetCommitResponseTopic {
    fn default() -> Self {
        OffsetCommitResponseTopic {
            name: String::from(""),
            partitions: Vec::<OffsetCommitResponsePartition>::new(),
        }
    }
}

impl OffsetCommitResponseTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OffsetCommitResponsePartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_offset_commit_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetCommitResponseTopic::new(
            String::from(""),
            Vec::<OffsetCommitResponsePartition>::new(),
        );
        assert_eq!(d, OffsetCommitResponseTopic::default());
    }
}

impl Readable for OffsetCommitResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<OffsetCommitResponsePartition>(input, "partitions", false)?;
        Ok(OffsetCommitResponseTopic {
            name, partitions
        })
    }
}

impl Writable for OffsetCommitResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetCommitResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for OffsetCommitResponsePartition {
    fn api_key(&self) -> i16 {
        8
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Response for OffsetCommitResponsePartition { }

impl Default for OffsetCommitResponsePartition {
    fn default() -> Self {
        OffsetCommitResponsePartition {
            partition_index: 0_i32,
            error_code: 0_i16,
        }
    }
}

impl OffsetCommitResponsePartition {
    pub fn new(partition_index: i32, error_code: i16) -> Self {
        Self {
            partition_index,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_offset_commit_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetCommitResponsePartition::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, OffsetCommitResponsePartition::default());
    }
}

impl Readable for OffsetCommitResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        Ok(OffsetCommitResponsePartition {
            partition_index, error_code
        })
    }
}

impl Writable for OffsetCommitResponsePartition {
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
        crate::test_utils::test_java_default::<OffsetCommitResponse>("OffsetCommitResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetCommitResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetCommitResponse) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetCommitResponse", 3);
        }
    }
}
