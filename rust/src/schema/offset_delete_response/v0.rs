// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetDeleteResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetDeleteResponse {
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetDeleteResponseTopic>,
}

impl ApiMessage for OffsetDeleteResponse {
    fn api_key(&self) -> i16 {
        47
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for OffsetDeleteResponse { }

impl Default for OffsetDeleteResponse {
    fn default() -> Self {
        OffsetDeleteResponse {
            error_code: 0_i16,
            throttle_time_ms: 0_i32,
            topics: Vec::<OffsetDeleteResponseTopic>::new(),
        }
    }
}

impl OffsetDeleteResponse {
    pub fn new(error_code: i16, throttle_time_ms: i32, topics: Vec<OffsetDeleteResponseTopic>) -> Self {
        Self {
            error_code,
            throttle_time_ms,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_offset_delete_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetDeleteResponse::new(
            0_i16,
            0_i32,
            Vec::<OffsetDeleteResponseTopic>::new(),
        );
        assert_eq!(d, OffsetDeleteResponse::default());
    }
}

impl Readable for OffsetDeleteResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<OffsetDeleteResponseTopic>(input, "topics", false)?;
        Ok(OffsetDeleteResponse {
            error_code, throttle_time_ms, topics
        })
    }
}

impl Writable for OffsetDeleteResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// OffsetDeleteResponseTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetDeleteResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The responses for each partition in the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetDeleteResponsePartition>,
}

impl Default for OffsetDeleteResponseTopic {
    fn default() -> Self {
        OffsetDeleteResponseTopic {
            name: String::from(""),
            partitions: Vec::<OffsetDeleteResponsePartition>::new(),
        }
    }
}

impl OffsetDeleteResponseTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OffsetDeleteResponsePartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_offset_delete_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetDeleteResponseTopic::new(
            String::from(""),
            Vec::<OffsetDeleteResponsePartition>::new(),
        );
        assert_eq!(d, OffsetDeleteResponseTopic::default());
    }
}

impl Readable for OffsetDeleteResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<OffsetDeleteResponsePartition>(input, "partitions", false)?;
        Ok(OffsetDeleteResponseTopic {
            name, partitions
        })
    }
}

impl Writable for OffsetDeleteResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// OffsetDeleteResponsePartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetDeleteResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl Default for OffsetDeleteResponsePartition {
    fn default() -> Self {
        OffsetDeleteResponsePartition {
            partition_index: 0_i32,
            error_code: 0_i16,
        }
    }
}

impl OffsetDeleteResponsePartition {
    pub fn new(partition_index: i32, error_code: i16) -> Self {
        Self {
            partition_index,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_offset_delete_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetDeleteResponsePartition::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, OffsetDeleteResponsePartition::default());
    }
}

impl Readable for OffsetDeleteResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        Ok(OffsetDeleteResponsePartition {
            partition_index, error_code
        })
    }
}

impl Writable for OffsetDeleteResponsePartition {
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
        crate::test_utils::test_java_default::<OffsetDeleteResponse>("OffsetDeleteResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetDeleteResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetDeleteResponse) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetDeleteResponse", 0);
        }
    }
}
