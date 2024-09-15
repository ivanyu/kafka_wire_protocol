// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteRecordsRequest {
    /// Each topic that we want to delete records from.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<DeleteRecordsTopic>,
    /// How long to wait for the deletion to complete, in milliseconds.
    pub timeout_ms: i32,
}

impl ApiMessage for DeleteRecordsRequest {
    fn api_key(&self) -> i16 {
        21
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for DeleteRecordsRequest { }

impl Default for DeleteRecordsRequest {
    fn default() -> Self {
        DeleteRecordsRequest {
            topics: Vec::<DeleteRecordsTopic>::new(),
            timeout_ms: 0_i32,
        }
    }
}

impl DeleteRecordsRequest {
    pub fn new(topics: Vec<DeleteRecordsTopic>, timeout_ms: i32) -> Self {
        Self {
            topics,
            timeout_ms,
        }
    }
}

#[cfg(test)]
mod tests_delete_records_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteRecordsRequest::new(
            Vec::<DeleteRecordsTopic>::new(),
            0_i32,
        );
        assert_eq!(d, DeleteRecordsRequest::default());
    }
}

impl Readable for DeleteRecordsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<DeleteRecordsTopic>(input, "topics", false)?;
        let timeout_ms = i32::read(input)?;
        Ok(DeleteRecordsRequest {
            topics, timeout_ms
        })
    }
}

impl Writable for DeleteRecordsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, false)?;
        self.timeout_ms.write(output)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteRecordsTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition that we want to delete records from.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<DeleteRecordsPartition>,
}

impl ApiMessage for DeleteRecordsTopic {
    fn api_key(&self) -> i16 {
        21
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for DeleteRecordsTopic { }

impl Default for DeleteRecordsTopic {
    fn default() -> Self {
        DeleteRecordsTopic {
            name: String::from(""),
            partitions: Vec::<DeleteRecordsPartition>::new(),
        }
    }
}

impl DeleteRecordsTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<DeleteRecordsPartition>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_delete_records_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteRecordsTopic::new(
            String::from(""),
            Vec::<DeleteRecordsPartition>::new(),
        );
        assert_eq!(d, DeleteRecordsTopic::default());
    }
}

impl Readable for DeleteRecordsTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<DeleteRecordsPartition>(input, "partitions", false)?;
        Ok(DeleteRecordsTopic {
            name, partitions
        })
    }
}

impl Writable for DeleteRecordsTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteRecordsPartition {
    /// The partition index.
    pub partition_index: i32,
    /// The deletion offset.
    pub offset: i64,
}

impl ApiMessage for DeleteRecordsPartition {
    fn api_key(&self) -> i16 {
        21
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for DeleteRecordsPartition { }

impl Default for DeleteRecordsPartition {
    fn default() -> Self {
        DeleteRecordsPartition {
            partition_index: 0_i32,
            offset: 0_i64,
        }
    }
}

impl DeleteRecordsPartition {
    pub fn new(partition_index: i32, offset: i64) -> Self {
        Self {
            partition_index,
            offset,
        }
    }
}

#[cfg(test)]
mod tests_delete_records_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteRecordsPartition::new(
            0_i32,
            0_i64,
        );
        assert_eq!(d, DeleteRecordsPartition::default());
    }
}

impl Readable for DeleteRecordsPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let offset = i64::read(input)?;
        Ok(DeleteRecordsPartition {
            partition_index, offset
        })
    }
}

impl Writable for DeleteRecordsPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.offset.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DeleteRecordsRequest>("DeleteRecordsRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteRecordsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteRecordsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteRecordsRequest", 1);
        }
    }
}
