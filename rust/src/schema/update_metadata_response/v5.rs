// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};

/// UpdateMetadataResponse, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateMetadataResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
}

impl ApiMessage for UpdateMetadataResponse {
    fn api_key(&self) -> i16 {
        6
    }
    
    fn version(&self) -> i16 {
        5
    }
}

impl Response for UpdateMetadataResponse { }

impl Default for UpdateMetadataResponse {
    fn default() -> Self {
        UpdateMetadataResponse {
            error_code: 0_i16,
        }
    }
}

impl UpdateMetadataResponse {
    pub fn new(error_code: i16) -> Self {
        Self {
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_update_metadata_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateMetadataResponse::new(
            0_i16,
        );
        assert_eq!(d, UpdateMetadataResponse::default());
    }
}

impl Readable for UpdateMetadataResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        Ok(UpdateMetadataResponse {
            error_code
        })
    }
}

impl Writable for UpdateMetadataResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<UpdateMetadataResponse>("UpdateMetadataResponse", 5);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UpdateMetadataResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UpdateMetadataResponse) {
            crate::test_utils::test_java_arbitrary(&data, "UpdateMetadataResponse", 5);
        }
    }
}
