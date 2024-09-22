// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Header};
use crate::readable_writable::{Readable, Writable};

/// RequestHeader, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RequestHeader {
    /// The API key of this request.
    pub request_api_key: i16,
    /// The API version of this request.
    pub request_api_version: i16,
    /// The correlation ID of this request.
    pub correlation_id: i32,
}

impl ApiMessage for RequestHeader {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        -1
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Header for RequestHeader { }

impl Default for RequestHeader {
    fn default() -> Self {
        RequestHeader {
            request_api_key: 0_i16,
            request_api_version: 0_i16,
            correlation_id: 0_i32,
        }
    }
}

impl RequestHeader {
    pub fn new(request_api_key: i16, request_api_version: i16, correlation_id: i32) -> Self {
        Self {
            request_api_key,
            request_api_version,
            correlation_id,
        }
    }
}

#[cfg(test)]
mod tests_request_header_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = RequestHeader::new(
            0_i16,
            0_i16,
            0_i32,
        );
        assert_eq!(d, RequestHeader::default());
    }
}

impl Readable for RequestHeader {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let request_api_key = i16::read(input)?;
        let request_api_version = i16::read(input)?;
        let correlation_id = i32::read(input)?;
        Ok(RequestHeader {
            request_api_key, request_api_version, correlation_id
        })
    }
}

impl Writable for RequestHeader {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.request_api_key.write(output)?;
        self.request_api_version.write(output)?;
        self.correlation_id.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<RequestHeader>("RequestHeader", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: RequestHeader) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: RequestHeader) {
            crate::test_utils::test_java_arbitrary(&data, "RequestHeader", 0);
        }
    }
}
