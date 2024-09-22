// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetFetchResponse, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponse {
    /// The responses per topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<OffsetFetchResponseTopic>,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for OffsetFetchResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        9
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        2
    }
}

impl Response for OffsetFetchResponse { }

impl Default for OffsetFetchResponse {
    fn default() -> Self {
        OffsetFetchResponse {
            topics: Vec::<OffsetFetchResponseTopic>::new(),
            error_code: 0_i16,
        }
    }
}

impl OffsetFetchResponse {
    pub fn new(topics: Vec<OffsetFetchResponseTopic>, error_code: i16) -> Self {
        Self {
            topics,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchResponse::new(
            Vec::<OffsetFetchResponseTopic>::new(),
            0_i16,
        );
        assert_eq!(d, OffsetFetchResponse::default());
    }
}

impl Readable for OffsetFetchResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<OffsetFetchResponseTopic>(input, "topics", false)?;
        let error_code = i16::read(input)?;
        Ok(OffsetFetchResponse {
            topics, error_code
        })
    }
}

impl Writable for OffsetFetchResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, false)?;
        self.error_code.write(output)?;
        Ok(())
    }
}

/// OffsetFetchResponseTopic, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponseTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The responses per partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<OffsetFetchResponsePartition>,
}

impl Default for OffsetFetchResponseTopic {
    fn default() -> Self {
        OffsetFetchResponseTopic {
            name: String::from(""),
            partitions: Vec::<OffsetFetchResponsePartition>::new(),
        }
    }
}

impl OffsetFetchResponseTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<OffsetFetchResponsePartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
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
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<OffsetFetchResponsePartition>(input, "partitions", false)?;
        Ok(OffsetFetchResponseTopic {
            name, partitions
        })
    }
}

impl Writable for OffsetFetchResponseTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// OffsetFetchResponsePartition, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchResponsePartition {
    /// The partition index.
    pub partition_index: i32,
    /// The committed message offset.
    pub committed_offset: i64,
    /// The partition metadata.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub metadata: Option<String>,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl Default for OffsetFetchResponsePartition {
    fn default() -> Self {
        OffsetFetchResponsePartition {
            partition_index: 0_i32,
            committed_offset: 0_i64,
            metadata: Some(String::from("")),
            error_code: 0_i16,
        }
    }
}

impl OffsetFetchResponsePartition {
    pub fn new<S1: AsRef<str>>(partition_index: i32, committed_offset: i64, metadata: Option<S1>, error_code: i16) -> Self {
        Self {
            partition_index,
            committed_offset,
            metadata: metadata.map(|s| s.as_ref().to_string()),
            error_code,
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
        let metadata = Option::<String>::read_ext(input, "metadata", false)?;
        let error_code = i16::read(input)?;
        Ok(OffsetFetchResponsePartition {
            partition_index, committed_offset, metadata, error_code
        })
    }
}

impl Writable for OffsetFetchResponsePartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.committed_offset.write(output)?;
        self.metadata.write_ext(output, "self.metadata", false)?;
        self.error_code.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<OffsetFetchResponse>("OffsetFetchResponse", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "OffsetFetchResponse", 2);
        }
    }
}
