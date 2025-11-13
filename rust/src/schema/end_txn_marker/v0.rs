// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Data};
use crate::readable_writable::{Readable, Writable};

/// EndTxnMarker, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EndTxnMarker {
    /// The coordinator epoch when appending the record
    pub coordinator_epoch: i32,
}

impl ApiMessage for EndTxnMarker {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Data for EndTxnMarker { }

impl Default for EndTxnMarker {
    fn default() -> Self {
        EndTxnMarker {
            coordinator_epoch: 0_i32,
        }
    }
}

impl EndTxnMarker {
    pub fn new(coordinator_epoch: i32) -> Self {
        Self {
            coordinator_epoch,
        }
    }
}

#[cfg(test)]
mod tests_end_txn_marker_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EndTxnMarker::new(
            0_i32,
        );
        assert_eq!(d, EndTxnMarker::default());
    }
}

impl Readable for EndTxnMarker {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let coordinator_epoch = i32::read(input)?;
        Ok(EndTxnMarker {
            coordinator_epoch
        })
    }
}

impl Writable for EndTxnMarker {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.coordinator_epoch.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<EndTxnMarker>("EndTxnMarker", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: EndTxnMarker) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: EndTxnMarker) {
            crate::test_utils::test_java_arbitrary(&data, "EndTxnMarker", 0);
        }
    }
}
