// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetDeleteRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetDeleteRequest {
    /// The unique group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The topics to delete offsets for
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetDeleteRequestTopic>,
}

impl ApiMessage for OffsetDeleteRequest {
    fn api_key(&self) -> i16 {
        47
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for OffsetDeleteRequest { }

impl Default for OffsetDeleteRequest {
    fn default() -> Self {
        OffsetDeleteRequest {
            group_id: String::from(""),
            topics: Vec::<OffsetDeleteRequestTopic>::new(),
        }
    }
}

impl OffsetDeleteRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<OffsetDeleteRequestTopic>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
        }
    }
}

#[cfg(test)]
mod tests_offset_delete_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetDeleteRequest::new(
            String::from(""),
            Vec::<OffsetDeleteRequestTopic>::new(),
        );
        assert_eq!(d, OffsetDeleteRequest::default());
    }
}

impl Readable for OffsetDeleteRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", false)?;
        let topics = read_array::<OffsetDeleteRequestTopic>(input, "topics", false)?;
        Ok(OffsetDeleteRequest {
            group_id, topics
        })
    }
}

impl Writable for OffsetDeleteRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", false)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// OffsetDeleteRequestTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetDeleteRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition to delete offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetDeleteRequestPartition>,
}

impl Default for OffsetDeleteRequestTopic {
    fn default() -> Self {
        OffsetDeleteRequestTopic {
            name: String::from(""),
            partitions: Vec::<OffsetDeleteRequestPartition>::new(),
        }
    }
}

impl OffsetDeleteRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OffsetDeleteRequestPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_offset_delete_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetDeleteRequestTopic::new(
            String::from(""),
            Vec::<OffsetDeleteRequestPartition>::new(),
        );
        assert_eq!(d, OffsetDeleteRequestTopic::default());
    }
}

impl Readable for OffsetDeleteRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<OffsetDeleteRequestPartition>(input, "partitions", false)?;
        Ok(OffsetDeleteRequestTopic {
            name, partitions
        })
    }
}

impl Writable for OffsetDeleteRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// OffsetDeleteRequestPartition, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetDeleteRequestPartition {
    /// The partition index.
    pub partition_index: i32,
}

impl Default for OffsetDeleteRequestPartition {
    fn default() -> Self {
        OffsetDeleteRequestPartition {
            partition_index: 0_i32,
        }
    }
}

impl OffsetDeleteRequestPartition {
    pub fn new(partition_index: i32) -> Self {
        Self {
            partition_index,
        }
    }
}

#[cfg(test)]
mod tests_offset_delete_request_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetDeleteRequestPartition::new(
            0_i32,
        );
        assert_eq!(d, OffsetDeleteRequestPartition::default());
    }
}

impl Readable for OffsetDeleteRequestPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        Ok(OffsetDeleteRequestPartition {
            partition_index
        })
    }
}

impl Writable for OffsetDeleteRequestPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<OffsetDeleteRequest>("OffsetDeleteRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetDeleteRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetDeleteRequest) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetDeleteRequest", 0);
        }
    }
}
