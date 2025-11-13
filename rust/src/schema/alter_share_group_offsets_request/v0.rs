// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AlterShareGroupOffsetsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterShareGroupOffsetsRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The topics to alter offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<AlterShareGroupOffsetsRequestTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterShareGroupOffsetsRequest {
    fn api_key(&self) -> i16 {
        91
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for AlterShareGroupOffsetsRequest { }

impl Default for AlterShareGroupOffsetsRequest {
    fn default() -> Self {
        AlterShareGroupOffsetsRequest {
            group_id: String::from(""),
            topics: Vec::<AlterShareGroupOffsetsRequestTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterShareGroupOffsetsRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<AlterShareGroupOffsetsRequestTopic>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_share_group_offsets_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterShareGroupOffsetsRequest::new(
            String::from(""),
            Vec::<AlterShareGroupOffsetsRequestTopic>::new(),
        );
        assert_eq!(d, AlterShareGroupOffsetsRequest::default());
    }
}

impl Readable for AlterShareGroupOffsetsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_array::<AlterShareGroupOffsetsRequestTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterShareGroupOffsetsRequest {
            group_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterShareGroupOffsetsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AlterShareGroupOffsetsRequestTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterShareGroupOffsetsRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// Each partition to alter offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<AlterShareGroupOffsetsRequestPartition>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AlterShareGroupOffsetsRequestTopic {
    fn default() -> Self {
        AlterShareGroupOffsetsRequestTopic {
            topic_name: String::from(""),
            partitions: Vec::<AlterShareGroupOffsetsRequestPartition>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterShareGroupOffsetsRequestTopic {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partitions: Vec<AlterShareGroupOffsetsRequestPartition>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_share_group_offsets_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterShareGroupOffsetsRequestTopic::new(
            String::from(""),
            Vec::<AlterShareGroupOffsetsRequestPartition>::new(),
        );
        assert_eq!(d, AlterShareGroupOffsetsRequestTopic::default());
    }
}

impl Readable for AlterShareGroupOffsetsRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partitions = read_array::<AlterShareGroupOffsetsRequestPartition>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterShareGroupOffsetsRequestTopic {
            topic_name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterShareGroupOffsetsRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AlterShareGroupOffsetsRequestPartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterShareGroupOffsetsRequestPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The share-partition start offset.
    pub start_offset: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AlterShareGroupOffsetsRequestPartition {
    fn default() -> Self {
        AlterShareGroupOffsetsRequestPartition {
            partition_index: 0_i32,
            start_offset: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterShareGroupOffsetsRequestPartition {
    pub fn new(partition_index: i32, start_offset: i64) -> Self {
        Self {
            partition_index,
            start_offset,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_share_group_offsets_request_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterShareGroupOffsetsRequestPartition::new(
            0_i32,
            0_i64,
        );
        assert_eq!(d, AlterShareGroupOffsetsRequestPartition::default());
    }
}

impl Readable for AlterShareGroupOffsetsRequestPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let start_offset = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterShareGroupOffsetsRequestPartition {
            partition_index, start_offset, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterShareGroupOffsetsRequestPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.start_offset.write(output)?;
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
        crate::test_utils::test_java_default::<AlterShareGroupOffsetsRequest>("AlterShareGroupOffsetsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterShareGroupOffsetsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterShareGroupOffsetsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AlterShareGroupOffsetsRequest", 0);
        }
    }
}
