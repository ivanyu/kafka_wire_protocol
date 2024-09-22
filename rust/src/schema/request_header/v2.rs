// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Header};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// RequestHeader, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RequestHeader {
    /// The API key of this request.
    pub request_api_key: i16,
    /// The API version of this request.
    pub request_api_version: i16,
    /// The correlation ID of this request.
    pub correlation_id: i32,
    /// The client ID string.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub client_id: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for RequestHeader {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        -1
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        2
    }
}

impl Header for RequestHeader { }

impl Default for RequestHeader {
    fn default() -> Self {
        RequestHeader {
            request_api_key: 0_i16,
            request_api_version: 0_i16,
            correlation_id: 0_i32,
            client_id: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl RequestHeader {
    pub fn new<S1: AsRef<str>>(request_api_key: i16, request_api_version: i16, correlation_id: i32, client_id: Option<S1>) -> Self {
        Self {
            request_api_key,
            request_api_version,
            correlation_id,
            client_id: client_id.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
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
            Some(String::from("")),
        );
        assert_eq!(d, RequestHeader::default());
    }
}

impl Readable for RequestHeader {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let request_api_key = i16::read(input)?;
        let request_api_version = i16::read(input)?;
        let correlation_id = i32::read(input)?;
        let client_id = Option::<String>::read_ext(input, "client_id", false)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(RequestHeader {
            request_api_key, request_api_version, correlation_id, client_id, _unknown_tagged_fields
        })
    }
}

impl Writable for RequestHeader {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.request_api_key.write(output)?;
        self.request_api_version.write(output)?;
        self.correlation_id.write(output)?;
        self.client_id.write_ext(output, "self.client_id", false)?;
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
        crate::test_utils::test_java_default::<RequestHeader>("RequestHeader", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "RequestHeader", 2);
        }
    }
}
