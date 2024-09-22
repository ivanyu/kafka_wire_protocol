// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetFetchRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchRequest {
    /// The group to fetch offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetFetchRequestTopic>,
}

impl ApiMessage for OffsetFetchRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        9
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for OffsetFetchRequest { }

impl Default for OffsetFetchRequest {
    fn default() -> Self {
        OffsetFetchRequest {
            group_id: String::from(""),
            topics: Vec::<OffsetFetchRequestTopic>::new(),
        }
    }
}

impl OffsetFetchRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<OffsetFetchRequestTopic>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchRequest::new(
            String::from(""),
            Vec::<OffsetFetchRequestTopic>::new(),
        );
        assert_eq!(d, OffsetFetchRequest::default());
    }
}

impl Readable for OffsetFetchRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", false)?;
        let topics = read_array::<OffsetFetchRequestTopic>(input, "topics", false)?;
        Ok(OffsetFetchRequest {
            group_id, topics
        })
    }
}

impl Writable for OffsetFetchRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", false)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// OffsetFetchRequestTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partition indexes we would like to fetch offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_indexes: Vec<i32>,
}

impl Default for OffsetFetchRequestTopic {
    fn default() -> Self {
        OffsetFetchRequestTopic {
            name: String::from(""),
            partition_indexes: Vec::<i32>::new(),
        }
    }
}

impl OffsetFetchRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partition_indexes: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_indexes,
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchRequestTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, OffsetFetchRequestTopic::default());
    }
}

impl Readable for OffsetFetchRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partition_indexes = read_array::<i32>(input, "partition_indexes", false)?;
        Ok(OffsetFetchRequestTopic {
            name, partition_indexes
        })
    }
}

impl Writable for OffsetFetchRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partition_indexes", &self.partition_indexes, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<OffsetFetchRequest>("OffsetFetchRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetFetchRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetFetchRequest) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetFetchRequest", 0);
        }
    }
}
