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
pub struct AlterClientQuotasRequest {
    /// The quota configuration entries to alter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub entries: Vec<EntryData>,
    /// Whether the alteration should be validated, but not performed.
    pub validate_only: bool,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterClientQuotasRequest {
    fn api_key(&self) -> i16 {
        49
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for AlterClientQuotasRequest { }

impl Default for AlterClientQuotasRequest {
    fn default() -> Self {
        AlterClientQuotasRequest {
            entries: Vec::<EntryData>::new(),
            validate_only: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterClientQuotasRequest {
    pub fn new(entries: Vec<EntryData>, validate_only: bool) -> Self {
        Self {
            entries,
            validate_only,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_client_quotas_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterClientQuotasRequest::new(
            Vec::<EntryData>::new(),
            false,
        );
        assert_eq!(d, AlterClientQuotasRequest::default());
    }
}

impl Readable for AlterClientQuotasRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let entries = read_array::<EntryData>(input, "entries", true)?;
        let validate_only = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterClientQuotasRequest {
            entries, validate_only, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterClientQuotasRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.entries", &self.entries, true)?;
        self.validate_only.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EntryData {
    /// The quota entity to alter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub entity: Vec<EntityData>,
    /// An individual quota configuration entry to alter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub ops: Vec<OpData>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EntryData {
    fn api_key(&self) -> i16 {
        49
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for EntryData { }

impl Default for EntryData {
    fn default() -> Self {
        EntryData {
            entity: Vec::<EntityData>::new(),
            ops: Vec::<OpData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl EntryData {
    pub fn new(entity: Vec<EntityData>, ops: Vec<OpData>) -> Self {
        Self {
            entity,
            ops,
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
            Vec::<OpData>::new(),
        );
        assert_eq!(d, EntryData::default());
    }
}

impl Readable for EntryData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let entity = read_array::<EntityData>(input, "entity", true)?;
        let ops = read_array::<OpData>(input, "ops", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(EntryData {
            entity, ops, _unknown_tagged_fields
        })
    }
}

impl Writable for EntryData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.entity", &self.entity, true)?;
        write_array(output, "self.ops", &self.ops, true)?;
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
    /// The name of the entity, or null if the default.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub entity_name: Option<String>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for EntityData {
    fn api_key(&self) -> i16 {
        49
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for EntityData { }

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
pub struct OpData {
    /// The quota configuration key.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub key: String,
    /// The value to set, otherwise ignored if the value is to be removed.
    pub value: f64,
    /// Whether the quota configuration value should be removed, otherwise set.
    pub remove: bool,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OpData {
    fn api_key(&self) -> i16 {
        49
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for OpData { }

impl Default for OpData {
    fn default() -> Self {
        OpData {
            key: String::from(""),
            value: 0.0,
            remove: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OpData {
    pub fn new<S1: AsRef<str>>(key: S1, value: f64, remove: bool) -> Self {
        Self {
            key: key.as_ref().to_string(),
            value,
            remove,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_op_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OpData::new(
            String::from(""),
            0.0,
            false,
        );
        assert_eq!(d, OpData::default());
    }
}

impl Readable for OpData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let key = String::read_ext(input, "key", true)?;
        let value = f64::read(input)?;
        let remove = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OpData {
            key, value, remove, _unknown_tagged_fields
        })
    }
}

impl Writable for OpData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key.write_ext(output, "self.key", true)?;
        self.value.write(output)?;
        self.remove.write(output)?;
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
        crate::test_utils::test_java_default::<AlterClientQuotasRequest>("AlterClientQuotasRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterClientQuotasRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterClientQuotasRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AlterClientQuotasRequest", 1);
        }
    }
}
