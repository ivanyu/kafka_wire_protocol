// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListConfigResourcesResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListConfigResourcesResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Each config resource in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub config_resources: Vec<ConfigResource>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListConfigResourcesResponse {
    fn api_key(&self) -> i16 {
        74
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for ListConfigResourcesResponse { }

impl Default for ListConfigResourcesResponse {
    fn default() -> Self {
        ListConfigResourcesResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            config_resources: Vec::<ConfigResource>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListConfigResourcesResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, config_resources: Vec<ConfigResource>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            config_resources,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_config_resources_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListConfigResourcesResponse::new(
            0_i32,
            0_i16,
            Vec::<ConfigResource>::new(),
        );
        assert_eq!(d, ListConfigResourcesResponse::default());
    }
}

impl Readable for ListConfigResourcesResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let config_resources = read_array::<ConfigResource>(input, "config_resources", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListConfigResourcesResponse {
            throttle_time_ms, error_code, config_resources, _unknown_tagged_fields
        })
    }
}

impl Writable for ListConfigResourcesResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.config_resources", &self.config_resources, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ConfigResource, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ConfigResource {
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
    /// The resource type.
    pub resource_type: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ConfigResource {
    fn default() -> Self {
        ConfigResource {
            resource_name: String::from(""),
            resource_type: 16_i8,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ConfigResource {
    pub fn new<S1: AsRef<str>>(resource_name: S1, resource_type: i8) -> Self {
        Self {
            resource_name: resource_name.as_ref().to_string(),
            resource_type,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_config_resource_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ConfigResource::new(
            String::from(""),
            16_i8,
        );
        assert_eq!(d, ConfigResource::default());
    }
}

impl Readable for ConfigResource {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resource_name = String::read_ext(input, "resource_name", true)?;
        let resource_type = i8::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ConfigResource {
            resource_name, resource_type, _unknown_tagged_fields
        })
    }
}

impl Writable for ConfigResource {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.resource_name.write_ext(output, "self.resource_name", true)?;
        self.resource_type.write(output)?;
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
        crate::test_utils::test_java_default::<ListConfigResourcesResponse>("ListConfigResourcesResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListConfigResourcesResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListConfigResourcesResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ListConfigResourcesResponse", 1);
        }
    }
}
