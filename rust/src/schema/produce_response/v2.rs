// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ProduceResponse, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ProduceResponse {
    /// Each produce response
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<TopicProduceResponse>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

impl ApiMessage for ProduceResponse {
    fn api_key(&self) -> i16 {
        0
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for ProduceResponse { }

impl Default for ProduceResponse {
    fn default() -> Self {
        ProduceResponse {
            responses: Vec::<TopicProduceResponse>::new(),
            throttle_time_ms: 0_i32,
        }
    }
}

impl ProduceResponse {
    pub fn new(responses: Vec<TopicProduceResponse>, throttle_time_ms: i32) -> Self {
        Self {
            responses,
            throttle_time_ms,
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
        let responses = read_array::<TopicProduceResponse>(input, "responses", false)?;
        let throttle_time_ms = i32::read(input)?;
        Ok(ProduceResponse {
            responses, throttle_time_ms
        })
    }
}

impl Writable for ProduceResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.responses", &self.responses, false)?;
        self.throttle_time_ms.write(output)?;
        Ok(())
    }
}

/// TopicProduceResponse, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicProduceResponse {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Each partition that we produced to within the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_responses: Vec<PartitionProduceResponse>,
}

impl Default for TopicProduceResponse {
    fn default() -> Self {
        TopicProduceResponse {
            name: String::from(""),
            partition_responses: Vec::<PartitionProduceResponse>::new(),
        }
    }
}

impl TopicProduceResponse {
    pub fn new<S1: AsRef<str>>(name: S1, partition_responses: Vec<PartitionProduceResponse>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_responses,
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
        let name = String::read_ext(input, "name", false)?;
        let partition_responses = read_array::<PartitionProduceResponse>(input, "partition_responses", false)?;
        Ok(TopicProduceResponse {
            name, partition_responses
        })
    }
}

impl Writable for TopicProduceResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partition_responses", &self.partition_responses, false)?;
        Ok(())
    }
}

/// PartitionProduceResponse, version 2.
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
}

impl Default for PartitionProduceResponse {
    fn default() -> Self {
        PartitionProduceResponse {
            index: 0_i32,
            error_code: 0_i16,
            base_offset: 0_i64,
            log_append_time_ms: -1_i64,
        }
    }
}

impl PartitionProduceResponse {
    pub fn new(index: i32, error_code: i16, base_offset: i64, log_append_time_ms: i64) -> Self {
        Self {
            index,
            error_code,
            base_offset,
            log_append_time_ms,
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
        Ok(PartitionProduceResponse {
            index, error_code, base_offset, log_append_time_ms
        })
    }
}

impl Writable for PartitionProduceResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.index.write(output)?;
        self.error_code.write(output)?;
        self.base_offset.write(output)?;
        self.log_append_time_ms.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ProduceResponse>("ProduceResponse", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "ProduceResponse", 2);
        }
    }
}
