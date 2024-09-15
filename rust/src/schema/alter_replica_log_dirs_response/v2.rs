// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDirsResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The results for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<AlterReplicaLogDirTopicResult>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterReplicaLogDirsResponse {
    fn api_key(&self) -> i16 {
        34
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for AlterReplicaLogDirsResponse { }

impl Default for AlterReplicaLogDirsResponse {
    fn default() -> Self {
        AlterReplicaLogDirsResponse {
            throttle_time_ms: 0_i32,
            results: Vec::<AlterReplicaLogDirTopicResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterReplicaLogDirsResponse {
    pub fn new(throttle_time_ms: i32, results: Vec<AlterReplicaLogDirTopicResult>) -> Self {
        Self {
            throttle_time_ms,
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_replica_log_dirs_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterReplicaLogDirsResponse::new(
            0_i32,
            Vec::<AlterReplicaLogDirTopicResult>::new(),
        );
        assert_eq!(d, AlterReplicaLogDirsResponse::default());
    }
}

impl Readable for AlterReplicaLogDirsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let results = read_array::<AlterReplicaLogDirTopicResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterReplicaLogDirsResponse {
            throttle_time_ms, results, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterReplicaLogDirsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDirTopicResult {
    /// The name of the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The results for each partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<AlterReplicaLogDirPartitionResult>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterReplicaLogDirTopicResult {
    fn api_key(&self) -> i16 {
        34
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for AlterReplicaLogDirTopicResult { }

impl Default for AlterReplicaLogDirTopicResult {
    fn default() -> Self {
        AlterReplicaLogDirTopicResult {
            topic_name: String::from(""),
            partitions: Vec::<AlterReplicaLogDirPartitionResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterReplicaLogDirTopicResult {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partitions: Vec<AlterReplicaLogDirPartitionResult>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_replica_log_dir_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterReplicaLogDirTopicResult::new(
            String::from(""),
            Vec::<AlterReplicaLogDirPartitionResult>::new(),
        );
        assert_eq!(d, AlterReplicaLogDirTopicResult::default());
    }
}

impl Readable for AlterReplicaLogDirTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partitions = read_array::<AlterReplicaLogDirPartitionResult>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterReplicaLogDirTopicResult {
            topic_name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterReplicaLogDirTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDirPartitionResult {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterReplicaLogDirPartitionResult {
    fn api_key(&self) -> i16 {
        34
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for AlterReplicaLogDirPartitionResult { }

impl Default for AlterReplicaLogDirPartitionResult {
    fn default() -> Self {
        AlterReplicaLogDirPartitionResult {
            partition_index: 0_i32,
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterReplicaLogDirPartitionResult {
    pub fn new(partition_index: i32, error_code: i16) -> Self {
        Self {
            partition_index,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_replica_log_dir_partition_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterReplicaLogDirPartitionResult::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, AlterReplicaLogDirPartitionResult::default());
    }
}

impl Readable for AlterReplicaLogDirPartitionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterReplicaLogDirPartitionResult {
            partition_index, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterReplicaLogDirPartitionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
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
        crate::test_utils::test_java_default::<AlterReplicaLogDirsResponse>("AlterReplicaLogDirsResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterReplicaLogDirsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterReplicaLogDirsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AlterReplicaLogDirsResponse", 2);
        }
    }
}
