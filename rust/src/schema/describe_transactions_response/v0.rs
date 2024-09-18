// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeTransactionsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeTransactionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub transaction_states: Vec<TransactionState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeTransactionsResponse {
    fn api_key(&self) -> i16 {
        65
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DescribeTransactionsResponse { }

impl Default for DescribeTransactionsResponse {
    fn default() -> Self {
        DescribeTransactionsResponse {
            throttle_time_ms: 0_i32,
            transaction_states: Vec::<TransactionState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeTransactionsResponse {
    pub fn new(throttle_time_ms: i32, transaction_states: Vec<TransactionState>) -> Self {
        Self {
            throttle_time_ms,
            transaction_states,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_transactions_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeTransactionsResponse::new(
            0_i32,
            Vec::<TransactionState>::new(),
        );
        assert_eq!(d, DescribeTransactionsResponse::default());
    }
}

impl Readable for DescribeTransactionsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let transaction_states = read_array::<TransactionState>(input, "transaction_states", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeTransactionsResponse {
            throttle_time_ms, transaction_states, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeTransactionsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.transaction_states", &self.transaction_states, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TransactionState, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TransactionState {
    /// 
    pub error_code: i16,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transactional_id: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transaction_state: String,
    /// 
    pub transaction_timeout_ms: i32,
    /// 
    pub transaction_start_time_ms: i64,
    /// 
    pub producer_id: i64,
    /// 
    pub producer_epoch: i16,
    /// The set of partitions included in the current transaction (if active). When a transaction is preparing to commit or abort, this will include only partitions which do not have markers.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TransactionState {
    fn default() -> Self {
        TransactionState {
            error_code: 0_i16,
            transactional_id: String::from(""),
            transaction_state: String::from(""),
            transaction_timeout_ms: 0_i32,
            transaction_start_time_ms: 0_i64,
            producer_id: 0_i64,
            producer_epoch: 0_i16,
            topics: Vec::<TopicData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TransactionState {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(error_code: i16, transactional_id: S1, transaction_state: S2, transaction_timeout_ms: i32, transaction_start_time_ms: i64, producer_id: i64, producer_epoch: i16, topics: Vec<TopicData>) -> Self {
        Self {
            error_code,
            transactional_id: transactional_id.as_ref().to_string(),
            transaction_state: transaction_state.as_ref().to_string(),
            transaction_timeout_ms,
            transaction_start_time_ms,
            producer_id,
            producer_epoch,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_transaction_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TransactionState::new(
            0_i16,
            String::from(""),
            String::from(""),
            0_i32,
            0_i64,
            0_i64,
            0_i16,
            Vec::<TopicData>::new(),
        );
        assert_eq!(d, TransactionState::default());
    }
}

impl Readable for TransactionState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let transactional_id = String::read_ext(input, "transactional_id", true)?;
        let transaction_state = String::read_ext(input, "transaction_state", true)?;
        let transaction_timeout_ms = i32::read(input)?;
        let transaction_start_time_ms = i64::read(input)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let topics = read_array::<TopicData>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TransactionState {
            error_code, transactional_id, transaction_state, transaction_timeout_ms, transaction_start_time_ms, producer_id, producer_epoch, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for TransactionState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.transactional_id.write_ext(output, "self.transactional_id", true)?;
        self.transaction_state.write_ext(output, "self.transaction_state", true)?;
        self.transaction_timeout_ms.write(output)?;
        self.transaction_start_time_ms.write(output)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicData {
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicData {
    fn default() -> Self {
        TopicData {
            topic: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicData {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicData::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, TopicData::default());
    }
}

impl Readable for TopicData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicData {
            topic, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
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
        crate::test_utils::test_java_default::<DescribeTransactionsResponse>("DescribeTransactionsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeTransactionsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeTransactionsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeTransactionsResponse", 0);
        }
    }
}
