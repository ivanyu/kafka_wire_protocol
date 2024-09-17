// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// BrokerHeartbeatRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BrokerHeartbeatRequest {
    /// The broker ID.
    pub broker_id: i32,
    /// The broker epoch.
    pub broker_epoch: i64,
    /// The highest metadata offset which the broker has reached.
    pub current_metadata_offset: i64,
    /// True if the broker wants to be fenced, false otherwise.
    pub want_fence: bool,
    /// True if the broker wants to be shut down, false otherwise.
    pub want_shut_down: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for BrokerHeartbeatRequest {
    fn api_key(&self) -> i16 {
        63
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for BrokerHeartbeatRequest { }

impl Default for BrokerHeartbeatRequest {
    fn default() -> Self {
        BrokerHeartbeatRequest {
            broker_id: 0_i32,
            broker_epoch: -1_i64,
            current_metadata_offset: 0_i64,
            want_fence: false,
            want_shut_down: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl BrokerHeartbeatRequest {
    pub fn new(broker_id: i32, broker_epoch: i64, current_metadata_offset: i64, want_fence: bool, want_shut_down: bool) -> Self {
        Self {
            broker_id,
            broker_epoch,
            current_metadata_offset,
            want_fence,
            want_shut_down,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_broker_heartbeat_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = BrokerHeartbeatRequest::new(
            0_i32,
            -1_i64,
            0_i64,
            false,
            false,
        );
        assert_eq!(d, BrokerHeartbeatRequest::default());
    }
}

impl Readable for BrokerHeartbeatRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let current_metadata_offset = i64::read(input)?;
        let want_fence = bool::read(input)?;
        let want_shut_down = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(BrokerHeartbeatRequest {
            broker_id, broker_epoch, current_metadata_offset, want_fence, want_shut_down, _unknown_tagged_fields
        })
    }
}

impl Writable for BrokerHeartbeatRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.broker_epoch.write(output)?;
        self.current_metadata_offset.write(output)?;
        self.want_fence.write(output)?;
        self.want_shut_down.write(output)?;
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
        crate::test_utils::test_java_default::<BrokerHeartbeatRequest>("BrokerHeartbeatRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: BrokerHeartbeatRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: BrokerHeartbeatRequest) {
            crate::test_utils::test_java_arbitrary(&data, "BrokerHeartbeatRequest", 0);
        }
    }
}
