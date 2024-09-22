// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeLogDirsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeLogDirsRequest {
    /// Each topic that we want to describe log directories for, or null for all topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topics: Option<Vec<DescribableLogDirTopic>>,
}

impl ApiMessage for DescribeLogDirsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        35
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DescribeLogDirsRequest { }

impl Default for DescribeLogDirsRequest {
    fn default() -> Self {
        DescribeLogDirsRequest {
            topics: Some(Vec::<DescribableLogDirTopic>::new()),
        }
    }
}

impl DescribeLogDirsRequest {
    pub fn new(topics: Option<Vec<DescribableLogDirTopic>>) -> Self {
        Self {
            topics,
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
        let topics = read_nullable_array::<DescribableLogDirTopic>(input, "topics", false)?;
        Ok(DescribeLogDirsRequest {
            topics
        })
    }
}

impl Writable for DescribeLogDirsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_nullable_array(output, "self.topics", self.topics.as_deref(), false)?;
        Ok(())
    }
}

/// DescribableLogDirTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribableLogDirTopic {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// The partition indexes.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
}

impl Default for DescribableLogDirTopic {
    fn default() -> Self {
        DescribableLogDirTopic {
            topic: String::from(""),
            partitions: Vec::<i32>::new(),
        }
    }
}

impl DescribableLogDirTopic {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
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
        let topic = String::read_ext(input, "topic", false)?;
        let partitions = read_array::<i32>(input, "partitions", false)?;
        Ok(DescribableLogDirTopic {
            topic, partitions
        })
    }
}

impl Writable for DescribableLogDirTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DescribeLogDirsRequest>("DescribeLogDirsRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "DescribeLogDirsRequest", 0);
        }
    }
}
