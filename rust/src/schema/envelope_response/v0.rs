// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_nullable_bytes, write_nullable_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_option_bytes};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EnvelopeResponse {
    /// The embedded response header and data.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_bytes()"))]
    #[cfg_attr(test, serde(with="serde_option_bytes"))]
    pub response_data: Option<Vec<u8>>,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EnvelopeResponse {
    fn api_key(&self) -> i16 {
        58
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for EnvelopeResponse { }

impl Default for EnvelopeResponse {
    fn default() -> Self {
        EnvelopeResponse {
            response_data: None,
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl EnvelopeResponse {
    pub fn new(response_data: Option<Vec<u8>>, error_code: i16) -> Self {
        Self {
            response_data,
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_envelope_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EnvelopeResponse::new(
            None::<Vec::<u8>>,
            0_i16,
        );
        assert_eq!(d, EnvelopeResponse::default());
    }
}

impl Readable for EnvelopeResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let response_data = read_nullable_bytes(input, "response_data", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(EnvelopeResponse {
            response_data, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for EnvelopeResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_nullable_bytes(output, "self.response_data", self.response_data.as_deref(), true)?;
        self.error_code.write(output)?;
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
        crate::test_utils::test_java_default::<EnvelopeResponse>("EnvelopeResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: EnvelopeResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: EnvelopeResponse) {
            crate::test_utils::test_java_arbitrary(&data, "EnvelopeResponse", 0);
        }
    }
}
