// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AllocateProducerIdsRequest {
    /// The ID of the requesting broker
    pub broker_id: i32,
    /// The epoch of the requesting broker
    pub broker_epoch: i64,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AllocateProducerIdsRequest {
    fn api_key(&self) -> i16 {
        67
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for AllocateProducerIdsRequest { }

impl Default for AllocateProducerIdsRequest {
    fn default() -> Self {
        AllocateProducerIdsRequest {
            broker_id: 0_i32,
            broker_epoch: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AllocateProducerIdsRequest {
    pub fn new(broker_id: i32, broker_epoch: i64) -> Self {
        Self {
            broker_id,
            broker_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_allocate_producer_ids_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AllocateProducerIdsRequest::new(
            0_i32,
            -1_i64,
        );
        assert_eq!(d, AllocateProducerIdsRequest::default());
    }
}

impl Readable for AllocateProducerIdsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AllocateProducerIdsRequest {
            broker_id, broker_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for AllocateProducerIdsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.broker_epoch.write(output)?;
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
        crate::test_utils::test_java_default::<AllocateProducerIdsRequest>("AllocateProducerIdsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AllocateProducerIdsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AllocateProducerIdsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AllocateProducerIdsRequest", 0);
        }
    }
}
