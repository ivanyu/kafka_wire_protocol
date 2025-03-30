// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AddPartitionsToTxnRequest, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnRequest {
    /// List of transactions to add partitions to.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub transactions: Vec<AddPartitionsToTxnTransaction>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AddPartitionsToTxnRequest {
    fn api_key(&self) -> i16 {
        24
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for AddPartitionsToTxnRequest { }

impl Default for AddPartitionsToTxnRequest {
    fn default() -> Self {
        AddPartitionsToTxnRequest {
            transactions: Vec::<AddPartitionsToTxnTransaction>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnRequest {
    pub fn new(transactions: Vec<AddPartitionsToTxnTransaction>) -> Self {
        Self {
            transactions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnRequest::new(
            Vec::<AddPartitionsToTxnTransaction>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnRequest::default());
    }
}

impl Readable for AddPartitionsToTxnRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactions = read_array::<AddPartitionsToTxnTransaction>(input, "transactions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnRequest {
            transactions, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.transactions", &self.transactions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AddPartitionsToTxnTransaction, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnTransaction {
    /// The transactional id corresponding to the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transactional_id: String,
    /// Current producer id in use by the transactional id.
    pub producer_id: i64,
    /// Current epoch associated with the producer id.
    pub producer_epoch: i16,
    /// Boolean to signify if we want to check if the partition is in the transaction rather than add it.
    pub verify_only: bool,
    /// The partitions to add to the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<AddPartitionsToTxnTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AddPartitionsToTxnTransaction {
    fn default() -> Self {
        AddPartitionsToTxnTransaction {
            transactional_id: String::from(""),
            producer_id: 0_i64,
            producer_epoch: 0_i16,
            verify_only: false,
            topics: Vec::<AddPartitionsToTxnTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnTransaction {
    pub fn new<S1: AsRef<str>>(transactional_id: S1, producer_id: i64, producer_epoch: i16, verify_only: bool, topics: Vec<AddPartitionsToTxnTopic>) -> Self {
        Self {
            transactional_id: transactional_id.as_ref().to_string(),
            producer_id,
            producer_epoch,
            verify_only,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_transaction_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnTransaction::new(
            String::from(""),
            0_i64,
            0_i16,
            false,
            Vec::<AddPartitionsToTxnTopic>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnTransaction::default());
    }
}

impl Readable for AddPartitionsToTxnTransaction {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = String::read_ext(input, "transactional_id", true)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let verify_only = bool::read(input)?;
        let topics = read_array::<AddPartitionsToTxnTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnTransaction {
            transactional_id, producer_id, producer_epoch, verify_only, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnTransaction {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", true)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        self.verify_only.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AddPartitionsToTxnTopic, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddPartitionsToTxnTopic {
    /// The name of the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partition indexes to add to the transaction.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AddPartitionsToTxnTopic {
    fn default() -> Self {
        AddPartitionsToTxnTopic {
            name: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddPartitionsToTxnTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_partitions_to_txn_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddPartitionsToTxnTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, AddPartitionsToTxnTopic::default());
    }
}

impl Readable for AddPartitionsToTxnTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddPartitionsToTxnTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for AddPartitionsToTxnTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
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
        crate::test_utils::test_java_default::<AddPartitionsToTxnRequest>("AddPartitionsToTxnRequest", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AddPartitionsToTxnRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AddPartitionsToTxnRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AddPartitionsToTxnRequest", 4);
        }
    }
}
