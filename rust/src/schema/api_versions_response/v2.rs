// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ApiVersionsResponse, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ApiVersionsResponse {
    /// The top-level error code.
    pub error_code: i16,
    /// The APIs supported by the broker.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub api_keys: Vec<ApiVersion>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

impl ApiMessage for ApiVersionsResponse {
    fn api_key(&self) -> i16 {
        18
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for ApiVersionsResponse { }

impl Default for ApiVersionsResponse {
    fn default() -> Self {
        ApiVersionsResponse {
            error_code: 0_i16,
            api_keys: Vec::<ApiVersion>::new(),
            throttle_time_ms: 0_i32,
        }
    }
}

impl ApiVersionsResponse {
    pub fn new(error_code: i16, api_keys: Vec<ApiVersion>, throttle_time_ms: i32) -> Self {
        Self {
            error_code,
            api_keys,
            throttle_time_ms,
        }
    }
}

#[cfg(test)]
mod tests_api_versions_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ApiVersionsResponse::new(
            0_i16,
            Vec::<ApiVersion>::new(),
            0_i32,
        );
        assert_eq!(d, ApiVersionsResponse::default());
    }
}

impl Readable for ApiVersionsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let api_keys = read_array::<ApiVersion>(input, "api_keys", false)?;
        let throttle_time_ms = i32::read(input)?;
        Ok(ApiVersionsResponse {
            error_code, api_keys, throttle_time_ms
        })
    }
}

impl Writable for ApiVersionsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.api_keys", &self.api_keys, false)?;
        self.throttle_time_ms.write(output)?;
        Ok(())
    }
}

/// ApiVersion, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ApiVersion {
    /// The API index.
    pub api_key: i16,
    /// The minimum supported version, inclusive.
    pub min_version: i16,
    /// The maximum supported version, inclusive.
    pub max_version: i16,
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion {
            api_key: 0_i16,
            min_version: 0_i16,
            max_version: 0_i16,
        }
    }
}

impl ApiVersion {
    pub fn new(api_key: i16, min_version: i16, max_version: i16) -> Self {
        Self {
            api_key,
            min_version,
            max_version,
        }
    }
}

#[cfg(test)]
mod tests_api_version_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ApiVersion::new(
            0_i16,
            0_i16,
            0_i16,
        );
        assert_eq!(d, ApiVersion::default());
    }
}

impl Readable for ApiVersion {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let api_key = i16::read(input)?;
        let min_version = i16::read(input)?;
        let max_version = i16::read(input)?;
        Ok(ApiVersion {
            api_key, min_version, max_version
        })
    }
}

impl Writable for ApiVersion {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.api_key.write(output)?;
        self.min_version.write(output)?;
        self.max_version.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ApiVersionsResponse>("ApiVersionsResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ApiVersionsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ApiVersionsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ApiVersionsResponse", 2);
        }
    }
}
