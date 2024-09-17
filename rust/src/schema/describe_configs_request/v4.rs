// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeConfigsRequest, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeConfigsRequest {
    /// The resources whose configurations we want to describe.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub resources: Vec<DescribeConfigsResource>,
    /// True if we should include all synonyms.
    pub include_synonyms: bool,
    /// True if we should include configuration documentation.
    pub include_documentation: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeConfigsRequest {
    fn api_key(&self) -> i16 {
        32
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Request for DescribeConfigsRequest { }

impl Default for DescribeConfigsRequest {
    fn default() -> Self {
        DescribeConfigsRequest {
            resources: Vec::<DescribeConfigsResource>::new(),
            include_synonyms: false,
            include_documentation: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeConfigsRequest {
    pub fn new(resources: Vec<DescribeConfigsResource>, include_synonyms: bool, include_documentation: bool) -> Self {
        Self {
            resources,
            include_synonyms,
            include_documentation,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_configs_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeConfigsRequest::new(
            Vec::<DescribeConfigsResource>::new(),
            false,
            false,
        );
        assert_eq!(d, DescribeConfigsRequest::default());
    }
}

impl Readable for DescribeConfigsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resources = read_array::<DescribeConfigsResource>(input, "resources", true)?;
        let include_synonyms = bool::read(input)?;
        let include_documentation = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeConfigsRequest {
            resources, include_synonyms, include_documentation, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeConfigsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.resources", &self.resources, true)?;
        self.include_synonyms.write(output)?;
        self.include_documentation.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeConfigsResource, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeConfigsResource {
    /// The resource type.
    pub resource_type: i8,
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
    /// The configuration keys to list, or null to list all configuration keys.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub configuration_keys: Option<Vec<String>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeConfigsResource {
    fn default() -> Self {
        DescribeConfigsResource {
            resource_type: 0_i8,
            resource_name: String::from(""),
            configuration_keys: Some(Vec::<String>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeConfigsResource {
    pub fn new<S1: AsRef<str>>(resource_type: i8, resource_name: S1, configuration_keys: Option<Vec<String>>) -> Self {
        Self {
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
            configuration_keys,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_configs_resource_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeConfigsResource::new(
            0_i8,
            String::from(""),
            Some(Vec::<String>::new()),
        );
        assert_eq!(d, DescribeConfigsResource::default());
    }
}

impl Readable for DescribeConfigsResource {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", true)?;
        let configuration_keys = read_nullable_array::<String>(input, "configuration_keys", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeConfigsResource {
            resource_type, resource_name, configuration_keys, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeConfigsResource {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", true)?;
        write_nullable_array(output, "self.configuration_keys", self.configuration_keys.as_deref(), true)?;
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
        crate::test_utils::test_java_default::<DescribeConfigsRequest>("DescribeConfigsRequest", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeConfigsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeConfigsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeConfigsRequest", 4);
        }
    }
}
