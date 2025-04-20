// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ReadShareGroupStateSummaryResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReadShareGroupStateSummaryResponse {
    /// The read results.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<ReadStateSummaryResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ReadShareGroupStateSummaryResponse {
    fn api_key(&self) -> i16 {
        87
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ReadShareGroupStateSummaryResponse { }

impl Default for ReadShareGroupStateSummaryResponse {
    fn default() -> Self {
        ReadShareGroupStateSummaryResponse {
            results: Vec::<ReadStateSummaryResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReadShareGroupStateSummaryResponse {
    pub fn new(results: Vec<ReadStateSummaryResult>) -> Self {
        Self {
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_read_share_group_state_summary_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReadShareGroupStateSummaryResponse::new(
            Vec::<ReadStateSummaryResult>::new(),
        );
        assert_eq!(d, ReadShareGroupStateSummaryResponse::default());
    }
}

impl Readable for ReadShareGroupStateSummaryResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let results = read_array::<ReadStateSummaryResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReadShareGroupStateSummaryResponse {
            results, _unknown_tagged_fields
        })
    }
}

impl Writable for ReadShareGroupStateSummaryResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ReadStateSummaryResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReadStateSummaryResult {
    /// The topic identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The results for the partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ReadStateSummaryResult {
    fn default() -> Self {
        ReadStateSummaryResult {
            topic_id: Uuid::nil(),
            partitions: Vec::<PartitionResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReadStateSummaryResult {
    pub fn new(topic_id: Uuid, partitions: Vec<PartitionResult>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_read_state_summary_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReadStateSummaryResult::new(
            Uuid::nil(),
            Vec::<PartitionResult>::new(),
        );
        assert_eq!(d, ReadStateSummaryResult::default());
    }
}

impl Readable for ReadStateSummaryResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<PartitionResult>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReadStateSummaryResult {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for ReadStateSummaryResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionResult {
    /// The partition index.
    pub partition: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The state epoch of the share-partition.
    pub state_epoch: i32,
    /// The share-partition start offset.
    pub start_offset: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionResult {
    fn default() -> Self {
        PartitionResult {
            partition: 0_i32,
            error_code: 0_i16,
            error_message: None,
            state_epoch: 0_i32,
            start_offset: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionResult {
    pub fn new<S1: AsRef<str>>(partition: i32, error_code: i16, error_message: Option<S1>, state_epoch: i32, start_offset: i64) -> Self {
        Self {
            partition,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            state_epoch,
            start_offset,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_partition_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionResult::new(
            0_i32,
            0_i16,
            None::<String>,
            0_i32,
            0_i64,
        );
        assert_eq!(d, PartitionResult::default());
    }
}

impl Readable for PartitionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let state_epoch = i32::read(input)?;
        let start_offset = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionResult {
            partition, error_code, error_message, state_epoch, start_offset, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        self.state_epoch.write(output)?;
        self.start_offset.write(output)?;
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
        crate::test_utils::test_java_default::<ReadShareGroupStateSummaryResponse>("ReadShareGroupStateSummaryResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ReadShareGroupStateSummaryResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ReadShareGroupStateSummaryResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ReadShareGroupStateSummaryResponse", 0);
        }
    }
}
