// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, read_nullable_bytes, write_bytes, write_nullable_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes, serde_option_bytes};

/// EnvelopeRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EnvelopeRequest {
    /// The embedded request header and data.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub request_data: Vec<u8>,
    /// Value of the initial client principal when the request is redirected by a broker.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_bytes()"))]
    #[cfg_attr(test, serde(with="serde_option_bytes"))]
    pub request_principal: Option<Vec<u8>>,
    /// The original client's address in bytes.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub client_host_address: Vec<u8>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EnvelopeRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        58
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for EnvelopeRequest { }

impl Default for EnvelopeRequest {
    fn default() -> Self {
        EnvelopeRequest {
            request_data: Vec::new(),
            request_principal: Some(Vec::new()),
            client_host_address: Vec::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl EnvelopeRequest {
    pub fn new(request_data: Vec<u8>, request_principal: Option<Vec<u8>>, client_host_address: Vec<u8>) -> Self {
        Self {
            request_data,
            request_principal,
            client_host_address,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_envelope_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EnvelopeRequest::new(
            Vec::new(),
            Some(Vec::new()),
            Vec::new(),
        );
        assert_eq!(d, EnvelopeRequest::default());
    }
}

impl Readable for EnvelopeRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let request_data = read_bytes(input, "request_data", true)?;
        let request_principal = read_nullable_bytes(input, "request_principal", true)?;
        let client_host_address = read_bytes(input, "client_host_address", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(EnvelopeRequest {
            request_data, request_principal, client_host_address, _unknown_tagged_fields
        })
    }
}

impl Writable for EnvelopeRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_bytes(output, "self.request_data", &self.request_data, true)?;
        write_nullable_bytes(output, "self.request_principal", self.request_principal.as_deref(), true)?;
        write_bytes(output, "self.client_host_address", &self.client_host_address, true)?;
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
        crate::test_utils::test_java_default::<EnvelopeRequest>("EnvelopeRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: EnvelopeRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: EnvelopeRequest) {
            crate::test_utils::test_java_arbitrary(&data, "EnvelopeRequest", 0);
        }
    }
}
