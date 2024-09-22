// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ApiVersionsRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ApiVersionsRequest {
    /// The name of the client.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_software_name: String,
    /// The version of the client.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_software_version: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ApiVersionsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        18
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Request for ApiVersionsRequest { }

impl Default for ApiVersionsRequest {
    fn default() -> Self {
        ApiVersionsRequest {
            client_software_name: String::from(""),
            client_software_version: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ApiVersionsRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(client_software_name: S1, client_software_version: S2) -> Self {
        Self {
            client_software_name: client_software_name.as_ref().to_string(),
            client_software_version: client_software_version.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_api_versions_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ApiVersionsRequest::new(
            String::from(""),
            String::from(""),
        );
        assert_eq!(d, ApiVersionsRequest::default());
    }
}

impl Readable for ApiVersionsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let client_software_name = String::read_ext(input, "client_software_name", true)?;
        let client_software_version = String::read_ext(input, "client_software_version", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ApiVersionsRequest {
            client_software_name, client_software_version, _unknown_tagged_fields
        })
    }
}

impl Writable for ApiVersionsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.client_software_name.write_ext(output, "self.client_software_name", true)?;
        self.client_software_version.write_ext(output, "self.client_software_version", true)?;
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
        crate::test_utils::test_java_default::<ApiVersionsRequest>("ApiVersionsRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ApiVersionsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ApiVersionsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ApiVersionsRequest", 3);
        }
    }
}
