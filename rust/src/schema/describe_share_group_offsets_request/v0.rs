// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeShareGroupOffsetsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeShareGroupOffsetsRequest {
    /// The groups to describe offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<DescribeShareGroupOffsetsRequestGroup>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeShareGroupOffsetsRequest {
    fn api_key(&self) -> i16 {
        90
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DescribeShareGroupOffsetsRequest { }

impl Default for DescribeShareGroupOffsetsRequest {
    fn default() -> Self {
        DescribeShareGroupOffsetsRequest {
            groups: Vec::<DescribeShareGroupOffsetsRequestGroup>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeShareGroupOffsetsRequest {
    pub fn new(groups: Vec<DescribeShareGroupOffsetsRequestGroup>) -> Self {
        Self {
            groups,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_share_group_offsets_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeShareGroupOffsetsRequest::new(
            Vec::<DescribeShareGroupOffsetsRequestGroup>::new(),
        );
        assert_eq!(d, DescribeShareGroupOffsetsRequest::default());
    }
}

impl Readable for DescribeShareGroupOffsetsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let groups = read_array::<DescribeShareGroupOffsetsRequestGroup>(input, "groups", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeShareGroupOffsetsRequest {
            groups, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeShareGroupOffsetsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.groups", &self.groups, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeShareGroupOffsetsRequestGroup, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeShareGroupOffsetsRequestGroup {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The topics to describe offsets for, or null for all topic-partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topics: Option<Vec<DescribeShareGroupOffsetsRequestTopic>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeShareGroupOffsetsRequestGroup {
    fn default() -> Self {
        DescribeShareGroupOffsetsRequestGroup {
            group_id: String::from(""),
            topics: Some(Vec::<DescribeShareGroupOffsetsRequestTopic>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeShareGroupOffsetsRequestGroup {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Option<Vec<DescribeShareGroupOffsetsRequestTopic>>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_share_group_offsets_request_group_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeShareGroupOffsetsRequestGroup::new(
            String::from(""),
            Some(Vec::<DescribeShareGroupOffsetsRequestTopic>::new()),
        );
        assert_eq!(d, DescribeShareGroupOffsetsRequestGroup::default());
    }
}

impl Readable for DescribeShareGroupOffsetsRequestGroup {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_nullable_array::<DescribeShareGroupOffsetsRequestTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeShareGroupOffsetsRequestGroup {
            group_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeShareGroupOffsetsRequestGroup {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_nullable_array(output, "self.topics", self.topics.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeShareGroupOffsetsRequestTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeShareGroupOffsetsRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeShareGroupOffsetsRequestTopic {
    fn default() -> Self {
        DescribeShareGroupOffsetsRequestTopic {
            topic_name: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeShareGroupOffsetsRequestTopic {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_share_group_offsets_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeShareGroupOffsetsRequestTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, DescribeShareGroupOffsetsRequestTopic::default());
    }
}

impl Readable for DescribeShareGroupOffsetsRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeShareGroupOffsetsRequestTopic {
            topic_name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeShareGroupOffsetsRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
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
        crate::test_utils::test_java_default::<DescribeShareGroupOffsetsRequest>("DescribeShareGroupOffsetsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeShareGroupOffsetsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeShareGroupOffsetsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeShareGroupOffsetsRequest", 0);
        }
    }
}
