// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteRecordsResponse, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteRecordsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each topic that we wanted to delete records from.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<DeleteRecordsTopicResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteRecordsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        21
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        2
    }
}

impl Response for DeleteRecordsResponse { }

impl Default for DeleteRecordsResponse {
    fn default() -> Self {
        DeleteRecordsResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<DeleteRecordsTopicResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteRecordsResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<DeleteRecordsTopicResult>) -> Self {
        Self {
            throttle_time_ms,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_records_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteRecordsResponse::new(
            0_i32,
            Vec::<DeleteRecordsTopicResult>::new(),
        );
        assert_eq!(d, DeleteRecordsResponse::default());
    }
}

impl Readable for DeleteRecordsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<DeleteRecordsTopicResult>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteRecordsResponse {
            throttle_time_ms, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteRecordsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteRecordsTopicResult, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteRecordsTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition that we wanted to delete records from.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<DeleteRecordsPartitionResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteRecordsTopicResult {
    fn default() -> Self {
        DeleteRecordsTopicResult {
            name: String::from(""),
            partitions: Vec::<DeleteRecordsPartitionResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteRecordsTopicResult {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<DeleteRecordsPartitionResult>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_records_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteRecordsTopicResult::new(
            String::from(""),
            Vec::<DeleteRecordsPartitionResult>::new(),
        );
        assert_eq!(d, DeleteRecordsTopicResult::default());
    }
}

impl Readable for DeleteRecordsTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<DeleteRecordsPartitionResult>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteRecordsTopicResult {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteRecordsTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteRecordsPartitionResult, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteRecordsPartitionResult {
    /// The partition index.
    pub partition_index: i32,
    /// The partition low water mark.
    pub low_watermark: i64,
    /// The deletion error code, or 0 if the deletion succeeded.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteRecordsPartitionResult {
    fn default() -> Self {
        DeleteRecordsPartitionResult {
            partition_index: 0_i32,
            low_watermark: 0_i64,
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteRecordsPartitionResult {
    pub fn new(partition_index: i32, low_watermark: i64, error_code: i16) -> Self {
        Self {
            partition_index,
            low_watermark,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_records_partition_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteRecordsPartitionResult::new(
            0_i32,
            0_i64,
            0_i16,
        );
        assert_eq!(d, DeleteRecordsPartitionResult::default());
    }
}

impl Readable for DeleteRecordsPartitionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let low_watermark = i64::read(input)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteRecordsPartitionResult {
            partition_index, low_watermark, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteRecordsPartitionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.low_watermark.write(output)?;
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
        crate::test_utils::test_java_default::<DeleteRecordsResponse>("DeleteRecordsResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteRecordsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteRecordsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteRecordsResponse", 2);
        }
    }
}
