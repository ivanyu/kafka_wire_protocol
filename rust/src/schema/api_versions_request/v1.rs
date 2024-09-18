// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};

/// ApiVersionsRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ApiVersionsRequest {
}

impl ApiMessage for ApiVersionsRequest {
    fn api_key(&self) -> i16 {
        18
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for ApiVersionsRequest { }

impl Default for ApiVersionsRequest {
    fn default() -> Self {
        ApiVersionsRequest {
        }
    }
}

impl ApiVersionsRequest {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests_api_versions_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ApiVersionsRequest::new(
        );
        assert_eq!(d, ApiVersionsRequest::default());
    }
}

impl Readable for ApiVersionsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        Ok(ApiVersionsRequest {
            
        })
    }
}

impl Writable for ApiVersionsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ApiVersionsRequest>("ApiVersionsRequest", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "ApiVersionsRequest", 1);
        }
    }
}
