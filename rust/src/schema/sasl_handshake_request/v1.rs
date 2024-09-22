// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// SaslHandshakeRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SaslHandshakeRequest {
    /// The SASL mechanism chosen by the client.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub mechanism: String,
}

impl ApiMessage for SaslHandshakeRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        17
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        1
    }
}

impl Request for SaslHandshakeRequest { }

impl Default for SaslHandshakeRequest {
    fn default() -> Self {
        SaslHandshakeRequest {
            mechanism: String::from(""),
        }
    }
}

impl SaslHandshakeRequest {
    pub fn new<S1: AsRef<str>>(mechanism: S1) -> Self {
        Self {
            mechanism: mechanism.as_ref().to_string(),
        }
    }
}

#[cfg(test)]
mod tests_sasl_handshake_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SaslHandshakeRequest::new(
            String::from(""),
        );
        assert_eq!(d, SaslHandshakeRequest::default());
    }
}

impl Readable for SaslHandshakeRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let mechanism = String::read_ext(input, "mechanism", false)?;
        Ok(SaslHandshakeRequest {
            mechanism
        })
    }
}

impl Writable for SaslHandshakeRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.mechanism.write_ext(output, "self.mechanism", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<SaslHandshakeRequest>("SaslHandshakeRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: SaslHandshakeRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: SaslHandshakeRequest) {
            crate::test_utils::test_java_arbitrary(&data, "SaslHandshakeRequest", 1);
        }
    }
}
