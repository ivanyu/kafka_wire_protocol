// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// InitProducerIdResponse, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct InitProducerIdResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The current producer id.
    pub producer_id: i64,
    /// The current epoch associated with the producer id.
    pub producer_epoch: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for InitProducerIdResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        22
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Response for InitProducerIdResponse { }

impl Default for InitProducerIdResponse {
    fn default() -> Self {
        InitProducerIdResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            producer_id: -1_i64,
            producer_epoch: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl InitProducerIdResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, producer_id: i64, producer_epoch: i16) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            producer_id,
            producer_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_init_producer_id_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = InitProducerIdResponse::new(
            0_i32,
            0_i16,
            -1_i64,
            0_i16,
        );
        assert_eq!(d, InitProducerIdResponse::default());
    }
}

impl Readable for InitProducerIdResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(InitProducerIdResponse {
            throttle_time_ms, error_code, producer_id, producer_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for InitProducerIdResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
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
        crate::test_utils::test_java_default::<InitProducerIdResponse>("InitProducerIdResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: InitProducerIdResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: InitProducerIdResponse) {
            crate::test_utils::test_java_arbitrary(&data, "InitProducerIdResponse", 3);
        }
    }
}
