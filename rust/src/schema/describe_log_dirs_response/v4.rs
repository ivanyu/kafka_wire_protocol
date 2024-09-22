// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeLogDirsResponse, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeLogDirsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The log directories.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<DescribeLogDirsResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeLogDirsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        35
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        4
    }
}

impl Response for DescribeLogDirsResponse { }

impl Default for DescribeLogDirsResponse {
    fn default() -> Self {
        DescribeLogDirsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            results: Vec::<DescribeLogDirsResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeLogDirsResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, results: Vec<DescribeLogDirsResult>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_log_dirs_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeLogDirsResponse::new(
            0_i32,
            0_i16,
            Vec::<DescribeLogDirsResult>::new(),
        );
        assert_eq!(d, DescribeLogDirsResponse::default());
    }
}

impl Readable for DescribeLogDirsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let results = read_array::<DescribeLogDirsResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeLogDirsResponse {
            throttle_time_ms, error_code, results, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeLogDirsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeLogDirsResult, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeLogDirsResult {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The absolute log directory path.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub log_dir: String,
    /// Each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<DescribeLogDirsTopic>,
    /// The total size in bytes of the volume the log directory is in.
    pub total_bytes: i64,
    /// The usable size in bytes of the volume the log directory is in.
    pub usable_bytes: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeLogDirsResult {
    fn default() -> Self {
        DescribeLogDirsResult {
            error_code: 0_i16,
            log_dir: String::from(""),
            topics: Vec::<DescribeLogDirsTopic>::new(),
            total_bytes: -1_i64,
            usable_bytes: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeLogDirsResult {
    pub fn new<S1: AsRef<str>>(error_code: i16, log_dir: S1, topics: Vec<DescribeLogDirsTopic>, total_bytes: i64, usable_bytes: i64) -> Self {
        Self {
            error_code,
            log_dir: log_dir.as_ref().to_string(),
            topics,
            total_bytes,
            usable_bytes,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_log_dirs_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeLogDirsResult::new(
            0_i16,
            String::from(""),
            Vec::<DescribeLogDirsTopic>::new(),
            -1_i64,
            -1_i64,
        );
        assert_eq!(d, DescribeLogDirsResult::default());
    }
}

impl Readable for DescribeLogDirsResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let log_dir = String::read_ext(input, "log_dir", true)?;
        let topics = read_array::<DescribeLogDirsTopic>(input, "topics", true)?;
        let total_bytes = i64::read(input)?;
        let usable_bytes = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeLogDirsResult {
            error_code, log_dir, topics, total_bytes, usable_bytes, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeLogDirsResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.log_dir.write_ext(output, "self.log_dir", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        self.total_bytes.write(output)?;
        self.usable_bytes.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeLogDirsTopic, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeLogDirsTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<DescribeLogDirsPartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeLogDirsTopic {
    fn default() -> Self {
        DescribeLogDirsTopic {
            name: String::from(""),
            partitions: Vec::<DescribeLogDirsPartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeLogDirsTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<DescribeLogDirsPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_log_dirs_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeLogDirsTopic::new(
            String::from(""),
            Vec::<DescribeLogDirsPartition>::new(),
        );
        assert_eq!(d, DescribeLogDirsTopic::default());
    }
}

impl Readable for DescribeLogDirsTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<DescribeLogDirsPartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeLogDirsTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeLogDirsTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeLogDirsPartition, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeLogDirsPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The size of the log segments in this partition in bytes.
    pub partition_size: i64,
    /// The lag of the log's LEO w.r.t. partition's HW (if it is the current log for the partition) or current replica's LEO (if it is the future log for the partition)
    pub offset_lag: i64,
    /// True if this log is created by AlterReplicaLogDirsRequest and will replace the current log of the replica in the future.
    pub is_future_key: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeLogDirsPartition {
    fn default() -> Self {
        DescribeLogDirsPartition {
            partition_index: 0_i32,
            partition_size: 0_i64,
            offset_lag: 0_i64,
            is_future_key: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeLogDirsPartition {
    pub fn new(partition_index: i32, partition_size: i64, offset_lag: i64, is_future_key: bool) -> Self {
        Self {
            partition_index,
            partition_size,
            offset_lag,
            is_future_key,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_log_dirs_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeLogDirsPartition::new(
            0_i32,
            0_i64,
            0_i64,
            false,
        );
        assert_eq!(d, DescribeLogDirsPartition::default());
    }
}

impl Readable for DescribeLogDirsPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let partition_size = i64::read(input)?;
        let offset_lag = i64::read(input)?;
        let is_future_key = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeLogDirsPartition {
            partition_index, partition_size, offset_lag, is_future_key, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeLogDirsPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.partition_size.write(output)?;
        self.offset_lag.write(output)?;
        self.is_future_key.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeLogDirsResponse>("DescribeLogDirsResponse", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeLogDirsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeLogDirsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeLogDirsResponse", 4);
        }
    }
}
