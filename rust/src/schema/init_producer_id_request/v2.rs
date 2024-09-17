// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// InitProducerIdRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct InitProducerIdRequest {
    /// The transactional id, or null if the producer is not transactional.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub transactional_id: Option<String>,
    /// The time in ms to wait before aborting idle transactions sent by this producer. This is only relevant if a TransactionalId has been defined.
    pub transaction_timeout_ms: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for InitProducerIdRequest {
    fn api_key(&self) -> i16 {
        22
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for InitProducerIdRequest { }

impl Default for InitProducerIdRequest {
    fn default() -> Self {
        InitProducerIdRequest {
            transactional_id: Some(String::from("")),
            transaction_timeout_ms: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl InitProducerIdRequest {
    pub fn new<S1: AsRef<str>>(transactional_id: Option<S1>, transaction_timeout_ms: i32) -> Self {
        Self {
            transactional_id: transactional_id.map(|s| s.as_ref().to_string()),
            transaction_timeout_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_init_producer_id_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = InitProducerIdRequest::new(
            Some(String::from("")),
            0_i32,
        );
        assert_eq!(d, InitProducerIdRequest::default());
    }
}

impl Readable for InitProducerIdRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let transactional_id = Option::<String>::read_ext(input, "transactional_id", true)?;
        let transaction_timeout_ms = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(InitProducerIdRequest {
            transactional_id, transaction_timeout_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for InitProducerIdRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.transactional_id.write_ext(output, "self.transactional_id", true)?;
        self.transaction_timeout_ms.write(output)?;
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
        crate::test_utils::test_java_default::<InitProducerIdRequest>("InitProducerIdRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: InitProducerIdRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: InitProducerIdRequest) {
            crate::test_utils::test_java_arbitrary(&data, "InitProducerIdRequest", 2);
        }
    }
}
