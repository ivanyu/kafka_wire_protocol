// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListTransactionsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListTransactionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// 
    pub error_code: i16,
    /// Set of state filters provided in the request which were unknown to the transaction coordinator
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub unknown_state_filters: Vec<String>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub transaction_states: Vec<TransactionState>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListTransactionsResponse {
    fn api_key(&self) -> i16 {
        66
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ListTransactionsResponse { }

impl Default for ListTransactionsResponse {
    fn default() -> Self {
        ListTransactionsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            unknown_state_filters: Vec::<String>::new(),
            transaction_states: Vec::<TransactionState>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListTransactionsResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, unknown_state_filters: Vec<String>, transaction_states: Vec<TransactionState>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            unknown_state_filters,
            transaction_states,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_transactions_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListTransactionsResponse::new(
            0_i32,
            0_i16,
            Vec::<String>::new(),
            Vec::<TransactionState>::new(),
        );
        assert_eq!(d, ListTransactionsResponse::default());
    }
}

impl Readable for ListTransactionsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let unknown_state_filters = read_array::<String>(input, "unknown_state_filters", true)?;
        let transaction_states = read_array::<TransactionState>(input, "transaction_states", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListTransactionsResponse {
            throttle_time_ms, error_code, unknown_state_filters, transaction_states, _unknown_tagged_fields
        })
    }
}

impl Writable for ListTransactionsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.unknown_state_filters", &self.unknown_state_filters, true)?;
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
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transactional_id: String,
    /// 
    pub producer_id: i64,
    /// The current transaction state of the producer
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub transaction_state: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TransactionState {
    fn default() -> Self {
        TransactionState {
            transactional_id: String::from(""),
            producer_id: 0_i64,
            transaction_state: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TransactionState {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(transactional_id: S1, producer_id: i64, transaction_state: S2) -> Self {
        Self {
            transactional_id: transactional_id.as_ref().to_string(),
            producer_id,
            transaction_state: transaction_state.as_ref().to_string(),
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
            String::from(""),
            0_i64,
            String::from(""),
        );
        assert_eq!(d, TransactionState::default());
    }
}

impl Readable for TransactionState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = String::read_ext(input, "transactional_id", true)?;
        let producer_id = i64::read(input)?;
        let transaction_state = String::read_ext(input, "transaction_state", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TransactionState {
            transactional_id, producer_id, transaction_state, _unknown_tagged_fields
        })
    }
}

impl Writable for TransactionState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", true)?;
        self.producer_id.write(output)?;
        self.transaction_state.write_ext(output, "self.transaction_state", true)?;
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
        crate::test_utils::test_java_default::<ListTransactionsResponse>("ListTransactionsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListTransactionsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListTransactionsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ListTransactionsResponse", 0);
        }
    }
}
