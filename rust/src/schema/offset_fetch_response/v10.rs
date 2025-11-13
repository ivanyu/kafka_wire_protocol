// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetFetchResponse, version 10.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The responses per group id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<OffsetFetchResponseGroup>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OffsetFetchResponse {
    fn api_key(&self) -> i16 {
        9
    }
    
    fn version(&self) -> i16 {
        10
    }
}

impl Response for OffsetFetchResponse { }

impl Default for OffsetFetchResponse {
    fn default() -> Self {
        OffsetFetchResponse {
            throttle_time_ms: 0_i32,
            groups: Vec::<OffsetFetchResponseGroup>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponse {
    pub fn new(throttle_time_ms: i32, groups: Vec<OffsetFetchResponseGroup>) -> Self {
        Self {
            throttle_time_ms,
            groups,
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
            Vec::<OffsetFetchResponseGroup>::new(),
        );
        assert_eq!(d, OffsetFetchResponse::default());
    }
}

impl Readable for OffsetFetchResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let groups = read_array::<OffsetFetchResponseGroup>(input, "groups", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchResponse {
            throttle_time_ms, groups, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.groups", &self.groups, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetFetchResponseGroup, version 10.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponseGroup {
    /// The group ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The responses per topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetFetchResponseTopics>,
    /// The group-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetFetchResponseGroup {
    fn default() -> Self {
        OffsetFetchResponseGroup {
            group_id: String::from(""),
            topics: Vec::<OffsetFetchResponseTopics>::new(),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponseGroup {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<OffsetFetchResponseTopics>, error_code: i16) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_response_group_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchResponseGroup::new(
            String::from(""),
            Vec::<OffsetFetchResponseTopics>::new(),
            0_i16,
        );
        assert_eq!(d, OffsetFetchResponseGroup::default());
    }
}

impl Readable for OffsetFetchResponseGroup {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_array::<OffsetFetchResponseTopics>(input, "topics", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchResponseGroup {
            group_id, topics, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchResponseGroup {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        self.error_code.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetFetchResponseTopics, version 10.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponseTopics {
    /// The topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The responses per partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetFetchResponsePartitions>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetFetchResponseTopics {
    fn default() -> Self {
        OffsetFetchResponseTopics {
            topic_id: Uuid::nil(),
            partitions: Vec::<OffsetFetchResponsePartitions>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponseTopics {
    pub fn new(topic_id: Uuid, partitions: Vec<OffsetFetchResponsePartitions>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_response_topics_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchResponseTopics::new(
            Uuid::nil(),
            Vec::<OffsetFetchResponsePartitions>::new(),
        );
        assert_eq!(d, OffsetFetchResponseTopics::default());
    }
}

impl Readable for OffsetFetchResponseTopics {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<OffsetFetchResponsePartitions>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchResponseTopics {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchResponseTopics {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetFetchResponsePartitions, version 10.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponsePartitions {
    /// The partition index.
    pub partition_index: i32,
    /// The committed message offset.
    pub committed_offset: i64,
    /// The leader epoch.
    pub committed_leader_epoch: i32,
    /// The partition metadata.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub metadata: Option<String>,
    /// The partition-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetFetchResponsePartitions {
    fn default() -> Self {
        OffsetFetchResponsePartitions {
            partition_index: 0_i32,
            committed_offset: 0_i64,
            committed_leader_epoch: -1_i32,
            metadata: Some(String::from("")),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchResponsePartitions {
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
mod tests_offset_fetch_response_partitions_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchResponsePartitions::new(
            0_i32,
            0_i64,
            -1_i32,
            Some(String::from("")),
            0_i16,
        );
        assert_eq!(d, OffsetFetchResponsePartitions::default());
    }
}

impl Readable for OffsetFetchResponsePartitions {
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
        Ok(OffsetFetchResponsePartitions {
            partition_index, committed_offset, committed_leader_epoch, metadata, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchResponsePartitions {
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
        crate::test_utils::test_java_default::<OffsetFetchResponse>("OffsetFetchResponse", 10);
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
            crate::test_utils::test_java_arbitrary(&data, "OffsetFetchResponse", 10);
        }
    }
}
