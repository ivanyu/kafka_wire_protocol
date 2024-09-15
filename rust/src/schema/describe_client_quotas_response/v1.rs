// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeClientQuotasResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or `0` if the quota description succeeded.
    pub error_code: i16,
    /// The error message, or `null` if the quota description succeeded.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// A result entry.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub entries: Option<Vec<EntryData>>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeClientQuotasResponse {
    fn api_key(&self) -> i16 {
        48
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for DescribeClientQuotasResponse { }

impl Default for DescribeClientQuotasResponse {
    fn default() -> Self {
        DescribeClientQuotasResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            entries: Some(Vec::<EntryData>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClientQuotasResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, entries: Option<Vec<EntryData>>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            entries,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_client_quotas_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeClientQuotasResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
            Some(Vec::<EntryData>::new()),
        );
        assert_eq!(d, DescribeClientQuotasResponse::default());
    }
}

impl Readable for DescribeClientQuotasResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let entries = read_nullable_array::<EntryData>(input, "entries", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeClientQuotasResponse {
            throttle_time_ms, error_code, error_message, entries, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeClientQuotasResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_nullable_array(output, "self.entries", self.entries.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EntryData {
    /// The quota entity description.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub entity: Vec<EntityData>,
    /// The quota values for the entity.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub values: Vec<ValueData>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EntryData {
    fn api_key(&self) -> i16 {
        48
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for EntryData { }

impl Default for EntryData {
    fn default() -> Self {
        EntryData {
            entity: Vec::<EntityData>::new(),
            values: Vec::<ValueData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl EntryData {
    pub fn new(entity: Vec<EntityData>, values: Vec<ValueData>) -> Self {
        Self {
            entity,
            values,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_entry_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EntryData::new(
            Vec::<EntityData>::new(),
            Vec::<ValueData>::new(),
        );
        assert_eq!(d, EntryData::default());
    }
}

impl Readable for EntryData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let entity = read_array::<EntityData>(input, "entity", true)?;
        let values = read_array::<ValueData>(input, "values", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(EntryData {
            entity, values, _unknown_tagged_fields
        })
    }
}

impl Writable for EntryData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.entity", &self.entity, true)?;
        write_array(output, "self.values", &self.values, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EntityData {
    /// The entity type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub entity_type: String,
    /// The entity name, or null if the default.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub entity_name: Option<String>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EntityData {
    fn api_key(&self) -> i16 {
        48
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for EntityData { }

impl Default for EntityData {
    fn default() -> Self {
        EntityData {
            entity_type: String::from(""),
            entity_name: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl EntityData {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(entity_type: S1, entity_name: Option<S2>) -> Self {
        Self {
            entity_type: entity_type.as_ref().to_string(),
            entity_name: entity_name.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_entity_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = EntityData::new(
            String::from(""),
            Some(String::from("")),
        );
        assert_eq!(d, EntityData::default());
    }
}

impl Readable for EntityData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let entity_type = String::read_ext(input, "entity_type", true)?;
        let entity_name = Option::<String>::read_ext(input, "entity_name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(EntityData {
            entity_type, entity_name, _unknown_tagged_fields
        })
    }
}

impl Writable for EntityData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.entity_type.write_ext(output, "self.entity_type", true)?;
        self.entity_name.write_ext(output, "self.entity_name", true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ValueData {
    /// The quota configuration key.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub key: String,
    /// The quota configuration value.
    pub value: f64,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ValueData {
    fn api_key(&self) -> i16 {
        48
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for ValueData { }

impl Default for ValueData {
    fn default() -> Self {
        ValueData {
            key: String::from(""),
            value: 0.0,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ValueData {
    pub fn new<S1: AsRef<str>>(key: S1, value: f64) -> Self {
        Self {
            key: key.as_ref().to_string(),
            value,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_value_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ValueData::new(
            String::from(""),
            0.0,
        );
        assert_eq!(d, ValueData::default());
    }
}

impl Readable for ValueData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let key = String::read_ext(input, "key", true)?;
        let value = f64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ValueData {
            key, value, _unknown_tagged_fields
        })
    }
}

impl Writable for ValueData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key.write_ext(output, "self.key", true)?;
        self.value.write(output)?;
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
        crate::test_utils::test_java_default::<DescribeClientQuotasResponse>("DescribeClientQuotasResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeClientQuotasResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeClientQuotasResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeClientQuotasResponse", 1);
        }
    }
}
