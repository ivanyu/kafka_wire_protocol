// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeProducersResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeProducersResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each topic in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicResponse>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeProducersResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        61
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DescribeProducersResponse { }

impl Default for DescribeProducersResponse {
    fn default() -> Self {
        DescribeProducersResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<TopicResponse>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeProducersResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<TopicResponse>) -> Self {
        Self {
            throttle_time_ms,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_producers_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeProducersResponse::new(
            0_i32,
            Vec::<TopicResponse>::new(),
        );
        assert_eq!(d, DescribeProducersResponse::default());
    }
}

impl Readable for DescribeProducersResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<TopicResponse>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeProducersResponse {
            throttle_time_ms, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeProducersResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicResponse {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionResponse>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicResponse {
    fn default() -> Self {
        TopicResponse {
            name: String::from(""),
            partitions: Vec::<PartitionResponse>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicResponse {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<PartitionResponse>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicResponse::new(
            String::from(""),
            Vec::<PartitionResponse>::new(),
        );
        assert_eq!(d, TopicResponse::default());
    }
}

impl Readable for TopicResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<PartitionResponse>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicResponse {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionResponse {
    /// The partition index.
    pub partition_index: i32,
    /// The partition error code, or 0 if there was no error.
    pub error_code: i16,
    /// The partition error message, which may be null if no additional details are available
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub active_producers: Vec<ProducerState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionResponse {
    fn default() -> Self {
        PartitionResponse {
            partition_index: 0_i32,
            error_code: 0_i16,
            error_message: None,
            active_producers: Vec::<ProducerState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionResponse {
    pub fn new<S1: AsRef<str>>(partition_index: i32, error_code: i16, error_message: Option<S1>, active_producers: Vec<ProducerState>) -> Self {
        Self {
            partition_index,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            active_producers,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_partition_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionResponse::new(
            0_i32,
            0_i16,
            None::<String>,
            Vec::<ProducerState>::new(),
        );
        assert_eq!(d, PartitionResponse::default());
    }
}

impl Readable for PartitionResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let active_producers = read_array::<ProducerState>(input, "active_producers", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionResponse {
            partition_index, error_code, error_message, active_producers, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.active_producers", &self.active_producers, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ProducerState, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ProducerState {
    /// 
    pub producer_id: i64,
    /// 
    pub producer_epoch: i32,
    /// 
    pub last_sequence: i32,
    /// 
    pub last_timestamp: i64,
    /// 
    pub coordinator_epoch: i32,
    /// 
    pub current_txn_start_offset: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ProducerState {
    fn default() -> Self {
        ProducerState {
            producer_id: 0_i64,
            producer_epoch: 0_i32,
            last_sequence: -1_i32,
            last_timestamp: -1_i64,
            coordinator_epoch: 0_i32,
            current_txn_start_offset: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ProducerState {
    pub fn new(producer_id: i64, producer_epoch: i32, last_sequence: i32, last_timestamp: i64, coordinator_epoch: i32, current_txn_start_offset: i64) -> Self {
        Self {
            producer_id,
            producer_epoch,
            last_sequence,
            last_timestamp,
            coordinator_epoch,
            current_txn_start_offset,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_producer_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ProducerState::new(
            0_i64,
            0_i32,
            -1_i32,
            -1_i64,
            0_i32,
            -1_i64,
        );
        assert_eq!(d, ProducerState::default());
    }
}

impl Readable for ProducerState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let producer_id = i64::read(input)?;
        let producer_epoch = i32::read(input)?;
        let last_sequence = i32::read(input)?;
        let last_timestamp = i64::read(input)?;
        let coordinator_epoch = i32::read(input)?;
        let current_txn_start_offset = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ProducerState {
            producer_id, producer_epoch, last_sequence, last_timestamp, coordinator_epoch, current_txn_start_offset, _unknown_tagged_fields
        })
    }
}

impl Writable for ProducerState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        self.last_sequence.write(output)?;
        self.last_timestamp.write(output)?;
        self.coordinator_epoch.write(output)?;
        self.current_txn_start_offset.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeProducersResponse>("DescribeProducersResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeProducersResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeProducersResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeProducersResponse", 0);
        }
    }
}
