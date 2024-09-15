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
pub struct DescribeClientQuotasRequest {
    /// Filter components to apply to quota entities.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub components: Vec<ComponentData>,
    /// Whether the match is strict, i.e. should exclude entities with unspecified entity types.
    pub strict: bool,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeClientQuotasRequest {
    fn api_key(&self) -> i16 {
        48
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for DescribeClientQuotasRequest { }

impl Default for DescribeClientQuotasRequest {
    fn default() -> Self {
        DescribeClientQuotasRequest {
            components: Vec::<ComponentData>::new(),
            strict: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClientQuotasRequest {
    pub fn new(components: Vec<ComponentData>, strict: bool) -> Self {
        Self {
            components,
            strict,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_client_quotas_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeClientQuotasRequest::new(
            Vec::<ComponentData>::new(),
            false,
        );
        assert_eq!(d, DescribeClientQuotasRequest::default());
    }
}

impl Readable for DescribeClientQuotasRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let components = read_array::<ComponentData>(input, "components", true)?;
        let strict = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeClientQuotasRequest {
            components, strict, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeClientQuotasRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.components", &self.components, true)?;
        self.strict.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ComponentData {
    /// The entity type that the filter component applies to.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub entity_type: String,
    /// How to match the entity {0 = exact name, 1 = default name, 2 = any specified name}.
    pub match_type: i8,
    /// The string to match against, or null if unused for the match type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub match_: Option<String>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ComponentData {
    fn api_key(&self) -> i16 {
        48
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for ComponentData { }

impl Default for ComponentData {
    fn default() -> Self {
        ComponentData {
            entity_type: String::from(""),
            match_type: 0_i8,
            match_: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ComponentData {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(entity_type: S1, match_type: i8, match_: Option<S2>) -> Self {
        Self {
            entity_type: entity_type.as_ref().to_string(),
            match_type,
            match_: match_.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_component_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ComponentData::new(
            String::from(""),
            0_i8,
            Some(String::from("")),
        );
        assert_eq!(d, ComponentData::default());
    }
}

impl Readable for ComponentData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let entity_type = String::read_ext(input, "entity_type", true)?;
        let match_type = i8::read(input)?;
        let match_ = Option::<String>::read_ext(input, "match_", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ComponentData {
            entity_type, match_type, match_, _unknown_tagged_fields
        })
    }
}

impl Writable for ComponentData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.entity_type.write_ext(output, "self.entity_type", true)?;
        self.match_type.write(output)?;
        self.match_.write_ext(output, "self.match_", true)?;
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
        crate::test_utils::test_java_default::<DescribeClientQuotasRequest>("DescribeClientQuotasRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeClientQuotasRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeClientQuotasRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeClientQuotasRequest", 1);
        }
    }
}
