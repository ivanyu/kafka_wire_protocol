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
pub struct UnregisterBrokerRequest {
    /// The broker ID to unregister.
    pub broker_id: i32,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for UnregisterBrokerRequest {
    fn api_key(&self) -> i16 {
        64
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for UnregisterBrokerRequest { }

impl Default for UnregisterBrokerRequest {
    fn default() -> Self {
        UnregisterBrokerRequest {
            broker_id: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UnregisterBrokerRequest {
    pub fn new(broker_id: i32) -> Self {
        Self {
            broker_id,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_unregister_broker_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UnregisterBrokerRequest::new(
            0_i32,
        );
        assert_eq!(d, UnregisterBrokerRequest::default());
    }
}

impl Readable for UnregisterBrokerRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UnregisterBrokerRequest {
            broker_id, _unknown_tagged_fields
        })
    }
}

impl Writable for UnregisterBrokerRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
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
        crate::test_utils::test_java_default::<UnregisterBrokerRequest>("UnregisterBrokerRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UnregisterBrokerRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UnregisterBrokerRequest) {
            crate::test_utils::test_java_arbitrary(&data, "UnregisterBrokerRequest", 0);
        }
    }
}
