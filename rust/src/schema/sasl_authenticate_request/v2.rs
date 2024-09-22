// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// SaslAuthenticateRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SaslAuthenticateRequest {
    /// The SASL authentication bytes from the client, as defined by the SASL mechanism.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub auth_bytes: Vec<u8>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for SaslAuthenticateRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        36
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        2
    }
}

impl Request for SaslAuthenticateRequest { }

impl Default for SaslAuthenticateRequest {
    fn default() -> Self {
        SaslAuthenticateRequest {
            auth_bytes: Vec::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl SaslAuthenticateRequest {
    pub fn new(auth_bytes: Vec<u8>) -> Self {
        Self {
            auth_bytes,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_sasl_authenticate_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SaslAuthenticateRequest::new(
            Vec::new(),
        );
        assert_eq!(d, SaslAuthenticateRequest::default());
    }
}

impl Readable for SaslAuthenticateRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let auth_bytes = read_bytes(input, "auth_bytes", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(SaslAuthenticateRequest {
            auth_bytes, _unknown_tagged_fields
        })
    }
}

impl Writable for SaslAuthenticateRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_bytes(output, "self.auth_bytes", &self.auth_bytes, true)?;
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
        crate::test_utils::test_java_default::<SaslAuthenticateRequest>("SaslAuthenticateRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: SaslAuthenticateRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: SaslAuthenticateRequest) {
            crate::test_utils::test_java_arbitrary(&data, "SaslAuthenticateRequest", 2);
        }
    }
}
