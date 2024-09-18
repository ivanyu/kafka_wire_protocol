// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AllocateProducerIdsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AllocateProducerIdsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top level response error code
    pub error_code: i16,
    /// The first producer ID in this range, inclusive
    pub producer_id_start: i64,
    /// The number of producer IDs in this range
    pub producer_id_len: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AllocateProducerIdsResponse {
    fn api_key(&self) -> i16 {
        67
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for AllocateProducerIdsResponse { }

impl Default for AllocateProducerIdsResponse {
    fn default() -> Self {
        AllocateProducerIdsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            producer_id_start: 0_i64,
            producer_id_len: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AllocateProducerIdsResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, producer_id_start: i64, producer_id_len: i32) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            producer_id_start,
            producer_id_len,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_allocate_producer_ids_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AllocateProducerIdsResponse::new(
            0_i32,
            0_i16,
            0_i64,
            0_i32,
        );
        assert_eq!(d, AllocateProducerIdsResponse::default());
    }
}

impl Readable for AllocateProducerIdsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let producer_id_start = i64::read(input)?;
        let producer_id_len = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AllocateProducerIdsResponse {
            throttle_time_ms, error_code, producer_id_start, producer_id_len, _unknown_tagged_fields
        })
    }
}

impl Writable for AllocateProducerIdsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.producer_id_start.write(output)?;
        self.producer_id_len.write(output)?;
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
        crate::test_utils::test_java_default::<AllocateProducerIdsResponse>("AllocateProducerIdsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AllocateProducerIdsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AllocateProducerIdsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AllocateProducerIdsResponse", 0);
        }
    }
}
