// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SaslAuthenticateResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The SASL authentication bytes from the server, as defined by the SASL mechanism.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub auth_bytes: Vec<u8>,
    /// Number of milliseconds after which only re-authentication over the existing connection to create a new session can occur.
    pub session_lifetime_ms: i64,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for SaslAuthenticateResponse {
    fn api_key(&self) -> i16 {
        36
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for SaslAuthenticateResponse { }

impl Default for SaslAuthenticateResponse {
    fn default() -> Self {
        SaslAuthenticateResponse {
            error_code: 0_i16,
            error_message: Some(String::from("")),
            auth_bytes: Vec::new(),
            session_lifetime_ms: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl SaslAuthenticateResponse {
    pub fn new<S1: AsRef<str>>(error_code: i16, error_message: Option<S1>, auth_bytes: Vec<u8>, session_lifetime_ms: i64) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            auth_bytes,
            session_lifetime_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_sasl_authenticate_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SaslAuthenticateResponse::new(
            0_i16,
            Some(String::from("")),
            Vec::new(),
            0_i64,
        );
        assert_eq!(d, SaslAuthenticateResponse::default());
    }
}

impl Readable for SaslAuthenticateResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let auth_bytes = read_bytes(input, "auth_bytes", true)?;
        let session_lifetime_ms = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(SaslAuthenticateResponse {
            error_code, error_message, auth_bytes, session_lifetime_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for SaslAuthenticateResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_bytes(output, "self.auth_bytes", &self.auth_bytes, true)?;
        self.session_lifetime_ms.write(output)?;
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
        crate::test_utils::test_java_default::<SaslAuthenticateResponse>("SaslAuthenticateResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: SaslAuthenticateResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: SaslAuthenticateResponse) {
            crate::test_utils::test_java_arbitrary(&data, "SaslAuthenticateResponse", 2);
        }
    }
}
