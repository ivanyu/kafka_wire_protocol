// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ProduceResponse, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ProduceResponse {
    /// Each produce response
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<TopicProduceResponse>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ProduceResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        0
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        9
    }
}

impl Response for ProduceResponse { }

impl Default for ProduceResponse {
    fn default() -> Self {
        ProduceResponse {
            responses: Vec::<TopicProduceResponse>::new(),
            throttle_time_ms: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ProduceResponse {
    pub fn new(responses: Vec<TopicProduceResponse>, throttle_time_ms: i32) -> Self {
        Self {
            responses,
            throttle_time_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_produce_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ProduceResponse::new(
            Vec::<TopicProduceResponse>::new(),
            0_i32,
        );
        assert_eq!(d, ProduceResponse::default());
    }
}

impl Readable for ProduceResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let responses = read_array::<TopicProduceResponse>(input, "responses", true)?;
        let throttle_time_ms = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ProduceResponse {
            responses, throttle_time_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for ProduceResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.responses", &self.responses, true)?;
        self.throttle_time_ms.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicProduceResponse, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicProduceResponse {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition that we produced to within the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_responses: Vec<PartitionProduceResponse>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicProduceResponse {
    fn default() -> Self {
        TopicProduceResponse {
            name: String::from(""),
            partition_responses: Vec::<PartitionProduceResponse>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicProduceResponse {
    pub fn new<S1: AsRef<str>>(name: S1, partition_responses: Vec<PartitionProduceResponse>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_responses,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_produce_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicProduceResponse::new(
            String::from(""),
            Vec::<PartitionProduceResponse>::new(),
        );
        assert_eq!(d, TopicProduceResponse::default());
    }
}

impl Readable for TopicProduceResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partition_responses = read_array::<PartitionProduceResponse>(input, "partition_responses", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicProduceResponse {
            name, partition_responses, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicProduceResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partition_responses", &self.partition_responses, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionProduceResponse, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionProduceResponse {
    /// The partition index.
    pub index: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The base offset.
    pub base_offset: i64,
    /// The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
    pub log_append_time_ms: i64,
    /// The log start offset.
    pub log_start_offset: i64,
    /// The batch indices of records that caused the batch to be dropped
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub record_errors: Vec<BatchIndexAndErrorMessage>,
    /// The global error message summarizing the common root cause of the records that caused the batch to be dropped
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionProduceResponse {
    fn default() -> Self {
        PartitionProduceResponse {
            index: 0_i32,
            error_code: 0_i16,
            base_offset: 0_i64,
            log_append_time_ms: -1_i64,
            log_start_offset: -1_i64,
            record_errors: Vec::<BatchIndexAndErrorMessage>::new(),
            error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionProduceResponse {
    pub fn new<S1: AsRef<str>>(index: i32, error_code: i16, base_offset: i64, log_append_time_ms: i64, log_start_offset: i64, record_errors: Vec<BatchIndexAndErrorMessage>, error_message: Option<S1>) -> Self {
        Self {
            index,
            error_code,
            base_offset,
            log_append_time_ms,
            log_start_offset,
            record_errors,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_partition_produce_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionProduceResponse::new(
            0_i32,
            0_i16,
            0_i64,
            -1_i64,
            -1_i64,
            Vec::<BatchIndexAndErrorMessage>::new(),
            None::<String>,
        );
        assert_eq!(d, PartitionProduceResponse::default());
    }
}

impl Readable for PartitionProduceResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let base_offset = i64::read(input)?;
        let log_append_time_ms = i64::read(input)?;
        let log_start_offset = i64::read(input)?;
        let record_errors = read_array::<BatchIndexAndErrorMessage>(input, "record_errors", true)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionProduceResponse {
            index, error_code, base_offset, log_append_time_ms, log_start_offset, record_errors, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionProduceResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.index.write(output)?;
        self.error_code.write(output)?;
        self.base_offset.write(output)?;
        self.log_append_time_ms.write(output)?;
        self.log_start_offset.write(output)?;
        write_array(output, "self.record_errors", &self.record_errors, true)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// BatchIndexAndErrorMessage, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BatchIndexAndErrorMessage {
    /// The batch index of the record that cause the batch to be dropped
    pub batch_index: i32,
    /// The error message of the record that caused the batch to be dropped
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub batch_index_error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for BatchIndexAndErrorMessage {
    fn default() -> Self {
        BatchIndexAndErrorMessage {
            batch_index: 0_i32,
            batch_index_error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl BatchIndexAndErrorMessage {
    pub fn new<S1: AsRef<str>>(batch_index: i32, batch_index_error_message: Option<S1>) -> Self {
        Self {
            batch_index,
            batch_index_error_message: batch_index_error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_batch_index_and_error_message_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = BatchIndexAndErrorMessage::new(
            0_i32,
            None::<String>,
        );
        assert_eq!(d, BatchIndexAndErrorMessage::default());
    }
}

impl Readable for BatchIndexAndErrorMessage {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let batch_index = i32::read(input)?;
        let batch_index_error_message = Option::<String>::read_ext(input, "batch_index_error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(BatchIndexAndErrorMessage {
            batch_index, batch_index_error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for BatchIndexAndErrorMessage {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.batch_index.write(output)?;
        self.batch_index_error_message.write_ext(output, "self.batch_index_error_message", true)?;
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
        crate::test_utils::test_java_default::<ProduceResponse>("ProduceResponse", 9);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ProduceResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ProduceResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ProduceResponse", 9);
        }
    }
}
