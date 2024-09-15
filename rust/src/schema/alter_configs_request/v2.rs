// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterConfigsRequest {
    /// The updates for each resource.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub resources: Vec<AlterConfigsResource>,
    /// True if we should validate the request, but not change the configurations.
    pub validate_only: bool,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterConfigsRequest {
    fn api_key(&self) -> i16 {
        33
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for AlterConfigsRequest { }

impl Default for AlterConfigsRequest {
    fn default() -> Self {
        AlterConfigsRequest {
            resources: Vec::<AlterConfigsResource>::new(),
            validate_only: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterConfigsRequest {
    pub fn new(resources: Vec<AlterConfigsResource>, validate_only: bool) -> Self {
        Self {
            resources,
            validate_only,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_configs_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterConfigsRequest::new(
            Vec::<AlterConfigsResource>::new(),
            false,
        );
        assert_eq!(d, AlterConfigsRequest::default());
    }
}

impl Readable for AlterConfigsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resources = read_array::<AlterConfigsResource>(input, "resources", true)?;
        let validate_only = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterConfigsRequest {
            resources, validate_only, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterConfigsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.resources", &self.resources, true)?;
        self.validate_only.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterConfigsResource {
    /// The resource type.
    pub resource_type: i8,
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
    /// The configurations.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub configs: Vec<AlterableConfig>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterConfigsResource {
    fn api_key(&self) -> i16 {
        33
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for AlterConfigsResource { }

impl Default for AlterConfigsResource {
    fn default() -> Self {
        AlterConfigsResource {
            resource_type: 0_i8,
            resource_name: String::from(""),
            configs: Vec::<AlterableConfig>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterConfigsResource {
    pub fn new<S1: AsRef<str>>(resource_type: i8, resource_name: S1, configs: Vec<AlterableConfig>) -> Self {
        Self {
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
            configs,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_configs_resource_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterConfigsResource::new(
            0_i8,
            String::from(""),
            Vec::<AlterableConfig>::new(),
        );
        assert_eq!(d, AlterConfigsResource::default());
    }
}

impl Readable for AlterConfigsResource {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", true)?;
        let configs = read_array::<AlterableConfig>(input, "configs", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterConfigsResource {
            resource_type, resource_name, configs, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterConfigsResource {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", true)?;
        write_array(output, "self.configs", &self.configs, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterableConfig {
    /// The configuration key name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The value to set for the configuration key.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub value: Option<String>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterableConfig {
    fn api_key(&self) -> i16 {
        33
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for AlterableConfig { }

impl Default for AlterableConfig {
    fn default() -> Self {
        AlterableConfig {
            name: String::from(""),
            value: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterableConfig {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, value: Option<S2>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value: value.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alterable_config_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterableConfig::new(
            String::from(""),
            Some(String::from("")),
        );
        assert_eq!(d, AlterableConfig::default());
    }
}

impl Readable for AlterableConfig {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let value = Option::<String>::read_ext(input, "value", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterableConfig {
            name, value, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterableConfig {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.value.write_ext(output, "self.value", true)?;
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
        crate::test_utils::test_java_default::<AlterConfigsRequest>("AlterConfigsRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterConfigsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterConfigsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AlterConfigsRequest", 2);
        }
    }
}
