// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeTopicPartitionsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeTopicPartitionsRequest {
    /// The topics to fetch details for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicRequest>,
    /// The maximum number of partitions included in the response.
    pub response_partition_limit: i32,
    /// The first topic and partition index to fetch details for.
    pub cursor: Option<Cursor>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeTopicPartitionsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        75
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DescribeTopicPartitionsRequest { }

impl Default for DescribeTopicPartitionsRequest {
    fn default() -> Self {
        DescribeTopicPartitionsRequest {
            topics: Vec::<TopicRequest>::new(),
            response_partition_limit: 2000_i32,
            cursor: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeTopicPartitionsRequest {
    pub fn new(topics: Vec<TopicRequest>, response_partition_limit: i32, cursor: Option<Cursor>) -> Self {
        Self {
            topics,
            response_partition_limit,
            cursor,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_topic_partitions_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeTopicPartitionsRequest::new(
            Vec::<TopicRequest>::new(),
            2000_i32,
            None::<Cursor>,
        );
        assert_eq!(d, DescribeTopicPartitionsRequest::default());
    }
}

impl Readable for DescribeTopicPartitionsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<TopicRequest>(input, "topics", true)?;
        let response_partition_limit = i32::read(input)?;
        let cursor = (if i8::read(input)? < 0 { Ok(None) } else { Cursor::read(input).map(Some) })?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeTopicPartitionsRequest {
            topics, response_partition_limit, cursor, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeTopicPartitionsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, true)?;
        self.response_partition_limit.write(output)?;
        (if let Some(v) = &self.cursor { 1_i8.write(output)?; v.write(output) } else { (-1_i8).write(output) })?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicRequest {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicRequest {
    fn default() -> Self {
        TopicRequest {
            name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicRequest {
    pub fn new<S1: AsRef<str>>(name: S1) -> Self {
        Self {
            name: name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicRequest::new(
            String::from(""),
        );
        assert_eq!(d, TopicRequest::default());
    }
}

impl Readable for TopicRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicRequest {
            name, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Cursor, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Cursor {
    /// The name for the first topic to process
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partition index to start with
    pub partition_index: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            topic_name: String::from(""),
            partition_index: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Cursor {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_cursor_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Cursor::new(
            String::from(""),
            0_i32,
        );
        assert_eq!(d, Cursor::default());
    }
}

impl Readable for Cursor {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partition_index = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Cursor {
            topic_name, partition_index, _unknown_tagged_fields
        })
    }
}

impl Writable for Cursor {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
        self.partition_index.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeTopicPartitionsRequest>("DescribeTopicPartitionsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeTopicPartitionsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeTopicPartitionsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeTopicPartitionsRequest", 0);
        }
    }
}
