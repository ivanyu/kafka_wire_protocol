// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// WriteTxnMarkersResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WriteTxnMarkersResponse {
    /// The results for writing makers.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub markers: Vec<WritableTxnMarkerResult>,
}

impl ApiMessage for WriteTxnMarkersResponse {
    fn api_key(&self) -> i16 {
        27
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for WriteTxnMarkersResponse { }

impl Default for WriteTxnMarkersResponse {
    fn default() -> Self {
        WriteTxnMarkersResponse {
            markers: Vec::<WritableTxnMarkerResult>::new(),
        }
    }
}

impl WriteTxnMarkersResponse {
    pub fn new(markers: Vec<WritableTxnMarkerResult>) -> Self {
        Self {
            markers,
        }
    }
}

#[cfg(test)]
mod tests_write_txn_markers_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WriteTxnMarkersResponse::new(
            Vec::<WritableTxnMarkerResult>::new(),
        );
        assert_eq!(d, WriteTxnMarkersResponse::default());
    }
}

impl Readable for WriteTxnMarkersResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let markers = read_array::<WritableTxnMarkerResult>(input, "markers", false)?;
        Ok(WriteTxnMarkersResponse {
            markers
        })
    }
}

impl Writable for WriteTxnMarkersResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.markers", &self.markers, false)?;
        Ok(())
    }
}

/// WritableTxnMarkerResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WritableTxnMarkerResult {
    /// The current producer ID in use by the transactional ID.
    pub producer_id: i64,
    /// The results by topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<WritableTxnMarkerTopicResult>,
}

impl Default for WritableTxnMarkerResult {
    fn default() -> Self {
        WritableTxnMarkerResult {
            producer_id: 0_i64,
            topics: Vec::<WritableTxnMarkerTopicResult>::new(),
        }
    }
}

impl WritableTxnMarkerResult {
    pub fn new(producer_id: i64, topics: Vec<WritableTxnMarkerTopicResult>) -> Self {
        Self {
            producer_id,
            topics,
        }
    }
}

#[cfg(test)]
mod tests_writable_txn_marker_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WritableTxnMarkerResult::new(
            0_i64,
            Vec::<WritableTxnMarkerTopicResult>::new(),
        );
        assert_eq!(d, WritableTxnMarkerResult::default());
    }
}

impl Readable for WritableTxnMarkerResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let producer_id = i64::read(input)?;
        let topics = read_array::<WritableTxnMarkerTopicResult>(input, "topics", false)?;
        Ok(WritableTxnMarkerResult {
            producer_id, topics
        })
    }
}

impl Writable for WritableTxnMarkerResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.producer_id.write(output)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// WritableTxnMarkerTopicResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WritableTxnMarkerTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The results by partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<WritableTxnMarkerPartitionResult>,
}

impl Default for WritableTxnMarkerTopicResult {
    fn default() -> Self {
        WritableTxnMarkerTopicResult {
            name: String::from(""),
            partitions: Vec::<WritableTxnMarkerPartitionResult>::new(),
        }
    }
}

impl WritableTxnMarkerTopicResult {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<WritableTxnMarkerPartitionResult>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_writable_txn_marker_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WritableTxnMarkerTopicResult::new(
            String::from(""),
            Vec::<WritableTxnMarkerPartitionResult>::new(),
        );
        assert_eq!(d, WritableTxnMarkerTopicResult::default());
    }
}

impl Readable for WritableTxnMarkerTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<WritableTxnMarkerPartitionResult>(input, "partitions", false)?;
        Ok(WritableTxnMarkerTopicResult {
            name, partitions
        })
    }
}

impl Writable for WritableTxnMarkerTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

/// WritableTxnMarkerPartitionResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WritableTxnMarkerPartitionResult {
    /// The partition index.
    pub partition_index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl Default for WritableTxnMarkerPartitionResult {
    fn default() -> Self {
        WritableTxnMarkerPartitionResult {
            partition_index: 0_i32,
            error_code: 0_i16,
        }
    }
}

impl WritableTxnMarkerPartitionResult {
    pub fn new(partition_index: i32, error_code: i16) -> Self {
        Self {
            partition_index,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_writable_txn_marker_partition_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WritableTxnMarkerPartitionResult::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, WritableTxnMarkerPartitionResult::default());
    }
}

impl Readable for WritableTxnMarkerPartitionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        Ok(WritableTxnMarkerPartitionResult {
            partition_index, error_code
        })
    }
}

impl Writable for WritableTxnMarkerPartitionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
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
        crate::test_utils::test_java_default::<WriteTxnMarkersResponse>("WriteTxnMarkersResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: WriteTxnMarkersResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: WriteTxnMarkersResponse) {
            crate::test_utils::test_java_arbitrary(&data, "WriteTxnMarkersResponse", 0);
        }
    }
}
