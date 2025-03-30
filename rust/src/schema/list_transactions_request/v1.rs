// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListTransactionsRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListTransactionsRequest {
    /// The transaction states to filter by: if empty, all transactions are returned; if non-empty, then only transactions matching one of the filtered states will be returned.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub state_filters: Vec<String>,
    /// The producerIds to filter by: if empty, all transactions will be returned; if non-empty, only transactions which match one of the filtered producerIds will be returned.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub producer_id_filters: Vec<i64>,
    /// Duration (in millis) to filter by: if < 0, all transactions will be returned; otherwise, only transactions running longer than this duration will be returned.
    pub duration_filter: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListTransactionsRequest {
    fn api_key(&self) -> i16 {
        66
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for ListTransactionsRequest { }

impl Default for ListTransactionsRequest {
    fn default() -> Self {
        ListTransactionsRequest {
            state_filters: Vec::<String>::new(),
            producer_id_filters: Vec::<i64>::new(),
            duration_filter: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListTransactionsRequest {
    pub fn new(state_filters: Vec<String>, producer_id_filters: Vec<i64>, duration_filter: i64) -> Self {
        Self {
            state_filters,
            producer_id_filters,
            duration_filter,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_transactions_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListTransactionsRequest::new(
            Vec::<String>::new(),
            Vec::<i64>::new(),
            -1_i64,
        );
        assert_eq!(d, ListTransactionsRequest::default());
    }
}

impl Readable for ListTransactionsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let state_filters = read_array::<String>(input, "state_filters", true)?;
        let producer_id_filters = read_array::<i64>(input, "producer_id_filters", true)?;
        let duration_filter = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListTransactionsRequest {
            state_filters, producer_id_filters, duration_filter, _unknown_tagged_fields
        })
    }
}

impl Writable for ListTransactionsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.state_filters", &self.state_filters, true)?;
        write_array(output, "self.producer_id_filters", &self.producer_id_filters, true)?;
        self.duration_filter.write(output)?;
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
        crate::test_utils::test_java_default::<ListTransactionsRequest>("ListTransactionsRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListTransactionsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListTransactionsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ListTransactionsRequest", 1);
        }
    }
}
