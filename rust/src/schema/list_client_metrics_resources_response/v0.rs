// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListClientMetricsResourcesResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListClientMetricsResourcesResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Each client metrics resource in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub client_metrics_resources: Vec<ClientMetricsResource>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListClientMetricsResourcesResponse {
    fn api_key(&self) -> i16 {
        74
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ListClientMetricsResourcesResponse { }

impl Default for ListClientMetricsResourcesResponse {
    fn default() -> Self {
        ListClientMetricsResourcesResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            client_metrics_resources: Vec::<ClientMetricsResource>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListClientMetricsResourcesResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, client_metrics_resources: Vec<ClientMetricsResource>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            client_metrics_resources,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_client_metrics_resources_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListClientMetricsResourcesResponse::new(
            0_i32,
            0_i16,
            Vec::<ClientMetricsResource>::new(),
        );
        assert_eq!(d, ListClientMetricsResourcesResponse::default());
    }
}

impl Readable for ListClientMetricsResourcesResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let client_metrics_resources = read_array::<ClientMetricsResource>(input, "client_metrics_resources", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListClientMetricsResourcesResponse {
            throttle_time_ms, error_code, client_metrics_resources, _unknown_tagged_fields
        })
    }
}

impl Writable for ListClientMetricsResourcesResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.client_metrics_resources", &self.client_metrics_resources, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ClientMetricsResource, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ClientMetricsResource {
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ClientMetricsResource {
    fn default() -> Self {
        ClientMetricsResource {
            name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ClientMetricsResource {
    pub fn new<S1: AsRef<str>>(name: S1) -> Self {
        Self {
            name: name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_client_metrics_resource_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ClientMetricsResource::new(
            String::from(""),
        );
        assert_eq!(d, ClientMetricsResource::default());
    }
}

impl Readable for ClientMetricsResource {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ClientMetricsResource {
            name, _unknown_tagged_fields
        })
    }
}

impl Writable for ClientMetricsResource {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
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
        crate::test_utils::test_java_default::<ListClientMetricsResourcesResponse>("ListClientMetricsResourcesResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListClientMetricsResourcesResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListClientMetricsResourcesResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ListClientMetricsResourcesResponse", 0);
        }
    }
}
