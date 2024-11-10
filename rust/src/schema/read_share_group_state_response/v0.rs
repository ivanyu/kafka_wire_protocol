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

/// ReadShareGroupStateResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReadShareGroupStateResponse {
    /// The read results
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<ReadStateResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ReadShareGroupStateResponse {
    fn api_key(&self) -> i16 {
        84
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ReadShareGroupStateResponse { }

impl Default for ReadShareGroupStateResponse {
    fn default() -> Self {
        ReadShareGroupStateResponse {
            results: Vec::<ReadStateResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReadShareGroupStateResponse {
    pub fn new(results: Vec<ReadStateResult>) -> Self {
        Self {
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_read_share_group_state_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReadShareGroupStateResponse::new(
            Vec::<ReadStateResult>::new(),
        );
        assert_eq!(d, ReadShareGroupStateResponse::default());
    }
}

impl Readable for ReadShareGroupStateResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let results = read_array::<ReadStateResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReadShareGroupStateResponse {
            results, _unknown_tagged_fields
        })
    }
}

impl Writable for ReadShareGroupStateResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ReadStateResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReadStateResult {
    /// The topic identifier
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The results for the partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ReadStateResult {
    fn default() -> Self {
        ReadStateResult {
            topic_id: Uuid::nil(),
            partitions: Vec::<PartitionResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReadStateResult {
    pub fn new(topic_id: Uuid, partitions: Vec<PartitionResult>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_read_state_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReadStateResult::new(
            Uuid::nil(),
            Vec::<PartitionResult>::new(),
        );
        assert_eq!(d, ReadStateResult::default());
    }
}

impl Readable for ReadStateResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<PartitionResult>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReadStateResult {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for ReadStateResult {
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
    /// The state epoch for this share-partition.
    pub state_epoch: i32,
    /// The share-partition start offset, which can be -1 if it is not yet initialized.
    pub start_offset: i64,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub state_batches: Vec<StateBatch>,
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
            state_batches: Vec::<StateBatch>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionResult {
    pub fn new<S1: AsRef<str>>(partition: i32, error_code: i16, error_message: Option<S1>, state_epoch: i32, start_offset: i64, state_batches: Vec<StateBatch>) -> Self {
        Self {
            partition,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            state_epoch,
            start_offset,
            state_batches,
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
            Vec::<StateBatch>::new(),
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
        let state_batches = read_array::<StateBatch>(input, "state_batches", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionResult {
            partition, error_code, error_message, state_epoch, start_offset, state_batches, _unknown_tagged_fields
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
        write_array(output, "self.state_batches", &self.state_batches, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// StateBatch, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StateBatch {
    /// The base offset of this state batch.
    pub first_offset: i64,
    /// The last offset of this state batch.
    pub last_offset: i64,
    /// The state - 0:Available,2:Acked,4:Archived.
    pub delivery_state: i8,
    /// The delivery count.
    pub delivery_count: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for StateBatch {
    fn default() -> Self {
        StateBatch {
            first_offset: 0_i64,
            last_offset: 0_i64,
            delivery_state: 0_i8,
            delivery_count: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl StateBatch {
    pub fn new(first_offset: i64, last_offset: i64, delivery_state: i8, delivery_count: i16) -> Self {
        Self {
            first_offset,
            last_offset,
            delivery_state,
            delivery_count,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_state_batch_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StateBatch::new(
            0_i64,
            0_i64,
            0_i8,
            0_i16,
        );
        assert_eq!(d, StateBatch::default());
    }
}

impl Readable for StateBatch {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let first_offset = i64::read(input)?;
        let last_offset = i64::read(input)?;
        let delivery_state = i8::read(input)?;
        let delivery_count = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(StateBatch {
            first_offset, last_offset, delivery_state, delivery_count, _unknown_tagged_fields
        })
    }
}

impl Writable for StateBatch {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.first_offset.write(output)?;
        self.last_offset.write(output)?;
        self.delivery_state.write(output)?;
        self.delivery_count.write(output)?;
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
        crate::test_utils::test_java_default::<ReadShareGroupStateResponse>("ReadShareGroupStateResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ReadShareGroupStateResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ReadShareGroupStateResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ReadShareGroupStateResponse", 0);
        }
    }
}
