// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeTransactionsRequest {
    /// Array of transactionalIds to include in describe results. If empty, then no results will be returned.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub transactional_ids: Vec<String>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeTransactionsRequest {
    fn api_key(&self) -> i16 {
        65
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DescribeTransactionsRequest { }

impl Default for DescribeTransactionsRequest {
    fn default() -> Self {
        DescribeTransactionsRequest {
            transactional_ids: Vec::<String>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeTransactionsRequest {
    pub fn new(transactional_ids: Vec<String>) -> Self {
        Self {
            transactional_ids,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_transactions_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeTransactionsRequest::new(
            Vec::<String>::new(),
        );
        assert_eq!(d, DescribeTransactionsRequest::default());
    }
}

impl Readable for DescribeTransactionsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_ids = read_array::<String>(input, "transactional_ids", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeTransactionsRequest {
            transactional_ids, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeTransactionsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.transactional_ids", &self.transactional_ids, true)?;
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
        crate::test_utils::test_java_default::<DescribeTransactionsRequest>("DescribeTransactionsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeTransactionsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeTransactionsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeTransactionsRequest", 0);
        }
    }
}
