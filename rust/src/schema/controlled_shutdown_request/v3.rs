// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ControlledShutdownRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ControlledShutdownRequest {
    /// The id of the broker for which controlled shutdown has been requested.
    pub broker_id: i32,
    /// The broker epoch.
    pub broker_epoch: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ControlledShutdownRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        7
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Request for ControlledShutdownRequest { }

impl Default for ControlledShutdownRequest {
    fn default() -> Self {
        ControlledShutdownRequest {
            broker_id: 0_i32,
            broker_epoch: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ControlledShutdownRequest {
    pub fn new(broker_id: i32, broker_epoch: i64) -> Self {
        Self {
            broker_id,
            broker_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_controlled_shutdown_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ControlledShutdownRequest::new(
            0_i32,
            -1_i64,
        );
        assert_eq!(d, ControlledShutdownRequest::default());
    }
}

impl Readable for ControlledShutdownRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ControlledShutdownRequest {
            broker_id, broker_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for ControlledShutdownRequest {
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
        crate::test_utils::test_java_default::<ControlledShutdownRequest>("ControlledShutdownRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ControlledShutdownRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ControlledShutdownRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ControlledShutdownRequest", 3);
        }
    }
}
