// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeProducersRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeProducersRequest {
    /// The topics to list producers for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicRequest>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeProducersRequest {
    fn api_key(&self) -> i16 {
        61
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DescribeProducersRequest { }

impl Default for DescribeProducersRequest {
    fn default() -> Self {
        DescribeProducersRequest {
            topics: Vec::<TopicRequest>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeProducersRequest {
    pub fn new(topics: Vec<TopicRequest>) -> Self {
        Self {
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_producers_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeProducersRequest::new(
            Vec::<TopicRequest>::new(),
        );
        assert_eq!(d, DescribeProducersRequest::default());
    }
}

impl Readable for DescribeProducersRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<TopicRequest>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeProducersRequest {
            topics, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeProducersRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicRequest {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The indexes of the partitions to list producers for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_indexes: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicRequest {
    fn default() -> Self {
        TopicRequest {
            name: String::from(""),
            partition_indexes: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicRequest {
    pub fn new<S1: AsRef<str>>(name: S1, partition_indexes: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_indexes,
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
            Vec::<i32>::new(),
        );
        assert_eq!(d, TopicRequest::default());
    }
}

impl Readable for TopicRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partition_indexes = read_array::<i32>(input, "partition_indexes", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicRequest {
            name, partition_indexes, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partition_indexes", &self.partition_indexes, true)?;
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
        crate::test_utils::test_java_default::<DescribeProducersRequest>("DescribeProducersRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeProducersRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeProducersRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeProducersRequest", 0);
        }
    }
}
