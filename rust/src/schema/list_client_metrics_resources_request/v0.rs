// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListClientMetricsResourcesRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListClientMetricsResourcesRequest {
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListClientMetricsResourcesRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        74
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ListClientMetricsResourcesRequest { }

impl Default for ListClientMetricsResourcesRequest {
    fn default() -> Self {
        ListClientMetricsResourcesRequest {
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListClientMetricsResourcesRequest {
    pub fn new() -> Self {
        Self {
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_client_metrics_resources_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListClientMetricsResourcesRequest::new(
        );
        assert_eq!(d, ListClientMetricsResourcesRequest::default());
    }
}

impl Readable for ListClientMetricsResourcesRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListClientMetricsResourcesRequest {
            _unknown_tagged_fields
        })
    }
}

impl Writable for ListClientMetricsResourcesRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
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
        crate::test_utils::test_java_default::<ListClientMetricsResourcesRequest>("ListClientMetricsResourcesRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListClientMetricsResourcesRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListClientMetricsResourcesRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ListClientMetricsResourcesRequest", 0);
        }
    }
}
