// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetFetchResponse, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses per topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetFetchResponseTopic>,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OffsetFetchResponse {
    fn api_key(&self) -> i16 {
        9
    }
    
    fn version(&self) -> i16 {
        6
    }
}

impl Response for OffsetFetchResponse { }

impl Default for OffsetFetchResponse {
    fn default() -> Self {
        OffsetFetchResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<OffsetFetchResponseTopic>::new(),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<OffsetFetchResponseTopic>, error_code: i16) -> Self {
        Self {
            throttle_time_ms,
            topics,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchResponse::new(
            0_i32,
            Vec::<OffsetFetchResponseTopic>::new(),
            0_i16,
        );
        assert_eq!(d, OffsetFetchResponse::default());
    }
}

impl Readable for OffsetFetchResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<OffsetFetchResponseTopic>(input, "topics", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchResponse {
            throttle_time_ms, topics, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        self.error_code.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetFetchResponseTopic, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The responses per partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetFetchResponsePartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetFetchResponseTopic {
    fn default() -> Self {
        OffsetFetchResponseTopic {
            name: String::from(""),
            partitions: Vec::<OffsetFetchResponsePartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponseTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OffsetFetchResponsePartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_response_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchResponseTopic::new(
            String::from(""),
            Vec::<OffsetFetchResponsePartition>::new(),
        );
        assert_eq!(d, OffsetFetchResponseTopic::default());
    }
}

impl Readable for OffsetFetchResponseTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<OffsetFetchResponsePartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchResponseTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetFetchResponsePartition, version 6.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The committed message offset.
    pub committed_offset: i64,
    /// The leader epoch.
    pub committed_leader_epoch: i32,
    /// The partition metadata.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub metadata: Option<String>,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetFetchResponsePartition {
    fn default() -> Self {
        OffsetFetchResponsePartition {
            partition_index: 0_i32,
            committed_offset: 0_i64,
            committed_leader_epoch: -1_i32,
            metadata: Some(String::from("")),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponsePartition {
    pub fn new<S1: AsRef<str>>(partition_index: i32, committed_offset: i64, committed_leader_epoch: i32, metadata: Option<S1>, error_code: i16) -> Self {
        Self {
            partition_index,
            committed_offset,
            committed_leader_epoch,
            metadata: metadata.map(|s| s.as_ref().to_string()),
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_response_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchResponsePartition::new(
            0_i32,
            0_i64,
            -1_i32,
            Some(String::from("")),
            0_i16,
        );
        assert_eq!(d, OffsetFetchResponsePartition::default());
    }
}

impl Readable for OffsetFetchResponsePartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let committed_offset = i64::read(input)?;
        let committed_leader_epoch = i32::read(input)?;
        let metadata = Option::<String>::read_ext(input, "metadata", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchResponsePartition {
            partition_index, committed_offset, committed_leader_epoch, metadata, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.committed_offset.write(output)?;
        self.committed_leader_epoch.write(output)?;
        self.metadata.write_ext(output, "self.metadata", true)?;
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
        crate::test_utils::test_java_default::<OffsetFetchResponse>("OffsetFetchResponse", 6);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetFetchResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetFetchResponse) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetFetchResponse", 6);
        }
    }
}
