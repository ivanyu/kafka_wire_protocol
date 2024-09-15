// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ControlledShutdownRequest {
    /// The id of the broker for which controlled shutdown has been requested.
    pub broker_id: i32,
    /// The broker epoch.
    pub broker_epoch: i64,
}

impl ApiMessage for ControlledShutdownRequest {
    fn api_key(&self) -> i16 {
        7
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for ControlledShutdownRequest { }

impl Default for ControlledShutdownRequest {
    fn default() -> Self {
        ControlledShutdownRequest {
            broker_id: 0_i32,
            broker_epoch: -1_i64,
        }
    }
}

impl ControlledShutdownRequest {
    pub fn new(broker_id: i32, broker_epoch: i64) -> Self {
        Self {
            broker_id,
            broker_epoch,
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
        Ok(ControlledShutdownRequest {
            broker_id, broker_epoch
        })
    }
}

impl Writable for ControlledShutdownRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.broker_epoch.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ControlledShutdownRequest>("ControlledShutdownRequest", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "ControlledShutdownRequest", 2);
        }
    }
}
