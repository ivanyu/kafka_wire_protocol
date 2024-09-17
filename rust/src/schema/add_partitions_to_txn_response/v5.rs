// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AddPartitionsToTxnResponse, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnResponse {
    /// Duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The response top level error code.
    pub error_code: i16,
    /// Results categorized by transactional ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results_by_transaction: Vec<AddPartitionsToTxnResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AddPartitionsToTxnResponse {
    fn api_key(&self) -> i16 {
        24
    }
    
    fn version(&self) -> i16 {
        5
    }
}

impl Response for AddPartitionsToTxnResponse { }

impl Default for AddPartitionsToTxnResponse {
    fn default() -> Self {
        AddPartitionsToTxnResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            results_by_transaction: Vec::<AddPartitionsToTxnResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, results_by_transaction: Vec<AddPartitionsToTxnResult>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            results_by_transaction,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnResponse::new(
            0_i32,
            0_i16,
            Vec::<AddPartitionsToTxnResult>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnResponse::default());
    }
}

impl Readable for AddPartitionsToTxnResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let results_by_transaction = read_array::<AddPartitionsToTxnResult>(input, "results_by_transaction", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnResponse {
            throttle_time_ms, error_code, results_by_transaction, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.results_by_transaction", &self.results_by_transaction, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AddPartitionsToTxnResult, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnResult {
    /// The transactional id corresponding to the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transactional_id: String,
    /// The results for each topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topic_results: Vec<AddPartitionsToTxnTopicResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AddPartitionsToTxnResult {
    fn default() -> Self {
        AddPartitionsToTxnResult {
            transactional_id: String::from(""),
            topic_results: Vec::<AddPartitionsToTxnTopicResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnResult {
    pub fn new<S1: AsRef<str>>(transactional_id: S1, topic_results: Vec<AddPartitionsToTxnTopicResult>) -> Self {
        Self {
            transactional_id: transactional_id.as_ref().to_string(),
            topic_results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnResult::new(
            String::from(""),
            Vec::<AddPartitionsToTxnTopicResult>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnResult::default());
    }
}

impl Readable for AddPartitionsToTxnResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = String::read_ext(input, "transactional_id", true)?;
        let topic_results = read_array::<AddPartitionsToTxnTopicResult>(input, "topic_results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnResult {
            transactional_id, topic_results, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", true)?;
        write_array(output, "self.topic_results", &self.topic_results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AddPartitionsToTxnPartitionResult, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnPartitionResult {
    /// The partition indexes.
    pub partition_index: i32,
    /// The response error code.
    pub partition_error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AddPartitionsToTxnPartitionResult {
    fn default() -> Self {
        AddPartitionsToTxnPartitionResult {
            partition_index: 0_i32,
            partition_error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnPartitionResult {
    pub fn new(partition_index: i32, partition_error_code: i16) -> Self {
        Self {
            partition_index,
            partition_error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_partition_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnPartitionResult::new(
            0_i32,
            0_i16,
        );
        assert_eq!(d, AddPartitionsToTxnPartitionResult::default());
    }
}

impl Readable for AddPartitionsToTxnPartitionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let partition_error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnPartitionResult {
            partition_index, partition_error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnPartitionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.partition_error_code.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AddPartitionsToTxnTopicResult, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The results for each partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results_by_partition: Vec<AddPartitionsToTxnPartitionResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AddPartitionsToTxnTopicResult {
    fn default() -> Self {
        AddPartitionsToTxnTopicResult {
            name: String::from(""),
            results_by_partition: Vec::<AddPartitionsToTxnPartitionResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnTopicResult {
    pub fn new<S1: AsRef<str>>(name: S1, results_by_partition: Vec<AddPartitionsToTxnPartitionResult>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            results_by_partition,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnTopicResult::new(
            String::from(""),
            Vec::<AddPartitionsToTxnPartitionResult>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnTopicResult::default());
    }
}

impl Readable for AddPartitionsToTxnTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let results_by_partition = read_array::<AddPartitionsToTxnPartitionResult>(input, "results_by_partition", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnTopicResult {
            name, results_by_partition, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.results_by_partition", &self.results_by_partition, true)?;
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
        crate::test_utils::test_java_default::<AddPartitionsToTxnResponse>("AddPartitionsToTxnResponse", 5);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AddPartitionsToTxnResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AddPartitionsToTxnResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AddPartitionsToTxnResponse", 5);
        }
    }
}
