// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// SaslHandshakeResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SaslHandshakeResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The mechanisms enabled in the server.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub mechanisms: Vec<String>,
}

impl ApiMessage for SaslHandshakeResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        17
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        1
    }
}

impl Response for SaslHandshakeResponse { }

impl Default for SaslHandshakeResponse {
    fn default() -> Self {
        SaslHandshakeResponse {
            error_code: 0_i16,
            mechanisms: Vec::<String>::new(),
        }
    }
}

impl SaslHandshakeResponse {
    pub fn new(error_code: i16, mechanisms: Vec<String>) -> Self {
        Self {
            error_code,
            mechanisms,
        }
    }
}

#[cfg(test)]
mod tests_sasl_handshake_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SaslHandshakeResponse::new(
            0_i16,
            Vec::<String>::new(),
        );
        assert_eq!(d, SaslHandshakeResponse::default());
    }
}

impl Readable for SaslHandshakeResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let mechanisms = read_array::<String>(input, "mechanisms", false)?;
        Ok(SaslHandshakeResponse {
            error_code, mechanisms
        })
    }
}

impl Writable for SaslHandshakeResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.mechanisms", &self.mechanisms, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<SaslHandshakeResponse>("SaslHandshakeResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: SaslHandshakeResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: SaslHandshakeResponse) {
            crate::test_utils::test_java_arbitrary(&data, "SaslHandshakeResponse", 1);
        }
    }
}
