// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeLogDirsRequest {
    /// Each topic that we want to describe log directories for, or null for all topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topics: Option<Vec<DescribableLogDirTopic>>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeLogDirsRequest {
    fn api_key(&self) -> i16 {
        35
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for DescribeLogDirsRequest { }

impl Default for DescribeLogDirsRequest {
    fn default() -> Self {
        DescribeLogDirsRequest {
            topics: Some(Vec::<DescribableLogDirTopic>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeLogDirsRequest {
    pub fn new(topics: Option<Vec<DescribableLogDirTopic>>) -> Self {
        Self {
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_log_dirs_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeLogDirsRequest::new(
            Some(Vec::<DescribableLogDirTopic>::new()),
        );
        assert_eq!(d, DescribeLogDirsRequest::default());
    }
}

impl Readable for DescribeLogDirsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_nullable_array::<DescribableLogDirTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeLogDirsRequest {
            topics, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeLogDirsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_nullable_array(output, "self.topics", self.topics.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribableLogDirTopic {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// The partition indexes.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribableLogDirTopic {
    fn api_key(&self) -> i16 {
        35
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for DescribableLogDirTopic { }

impl Default for DescribableLogDirTopic {
    fn default() -> Self {
        DescribableLogDirTopic {
            topic: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribableLogDirTopic {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describable_log_dir_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribableLogDirTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, DescribableLogDirTopic::default());
    }
}

impl Readable for DescribableLogDirTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribableLogDirTopic {
            topic, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribableLogDirTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", true)?;
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
        crate::test_utils::test_java_default::<DescribeLogDirsRequest>("DescribeLogDirsRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeLogDirsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeLogDirsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeLogDirsRequest", 2);
        }
    }
}
