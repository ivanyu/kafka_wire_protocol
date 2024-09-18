// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// SaslAuthenticateResponse, version 1.
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
}

impl ApiMessage for SaslAuthenticateResponse {
    fn api_key(&self) -> i16 {
        36
    }
    
    fn version(&self) -> i16 {
        1
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
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        let auth_bytes = read_bytes(input, "auth_bytes", false)?;
        let session_lifetime_ms = i64::read(input)?;
        Ok(SaslAuthenticateResponse {
            error_code, error_message, auth_bytes, session_lifetime_ms
        })
    }
}

impl Writable for SaslAuthenticateResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        write_bytes(output, "self.auth_bytes", &self.auth_bytes, false)?;
        self.session_lifetime_ms.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<SaslAuthenticateResponse>("SaslAuthenticateResponse", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "SaslAuthenticateResponse", 1);
        }
    }
}
