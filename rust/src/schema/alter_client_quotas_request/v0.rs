// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AlterClientQuotasRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterClientQuotasRequest {
    /// The quota configuration entries to alter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub entries: Vec<EntryData>,
    /// Whether the alteration should be validated, but not performed.
    pub validate_only: bool,
}

impl ApiMessage for AlterClientQuotasRequest {
    fn api_key(&self) -> i16 {
        49
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for AlterClientQuotasRequest { }

impl Default for AlterClientQuotasRequest {
    fn default() -> Self {
        AlterClientQuotasRequest {
            entries: Vec::<EntryData>::new(),
            validate_only: false,
        }
    }
}

impl AlterClientQuotasRequest {
    pub fn new(entries: Vec<EntryData>, validate_only: bool) -> Self {
        Self {
            entries,
            validate_only,
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
        let entries = read_array::<EntryData>(input, "entries", false)?;
        let validate_only = bool::read(input)?;
        Ok(AlterClientQuotasRequest {
            entries, validate_only
        })
    }
}

impl Writable for AlterClientQuotasRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.entries", &self.entries, false)?;
        self.validate_only.write(output)?;
        Ok(())
    }
}

/// EntryData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EntryData {
    /// The quota entity to alter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub entity: Vec<EntityData>,
    /// An individual quota configuration entry to alter.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub ops: Vec<OpData>,
}

impl Default for EntryData {
    fn default() -> Self {
        EntryData {
            entity: Vec::<EntityData>::new(),
            ops: Vec::<OpData>::new(),
        }
    }
}

impl EntryData {
    pub fn new(entity: Vec<EntityData>, ops: Vec<OpData>) -> Self {
        Self {
            entity,
            ops,
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
        let entity = read_array::<EntityData>(input, "entity", false)?;
        let ops = read_array::<OpData>(input, "ops", false)?;
        Ok(EntryData {
            entity, ops
        })
    }
}

impl Writable for EntryData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.entity", &self.entity, false)?;
        write_array(output, "self.ops", &self.ops, false)?;
        Ok(())
    }
}

/// EntityData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct EntityData {
    /// The entity type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub entity_type: String,
    /// The name of the entity, or null if the default.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub entity_name: Option<String>,
}

impl Default for EntityData {
    fn default() -> Self {
        EntityData {
            entity_type: String::from(""),
            entity_name: Some(String::from("")),
        }
    }
}

impl EntityData {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(entity_type: S1, entity_name: Option<S2>) -> Self {
        Self {
            entity_type: entity_type.as_ref().to_string(),
            entity_name: entity_name.map(|s| s.as_ref().to_string()),
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
        let entity_type = String::read_ext(input, "entity_type", false)?;
        let entity_name = Option::<String>::read_ext(input, "entity_name", false)?;
        Ok(EntityData {
            entity_type, entity_name
        })
    }
}

impl Writable for EntityData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.entity_type.write_ext(output, "self.entity_type", false)?;
        self.entity_name.write_ext(output, "self.entity_name", false)?;
        Ok(())
    }
}

/// OpData, version 0.
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
}

impl Default for OpData {
    fn default() -> Self {
        OpData {
            key: String::from(""),
            value: 0.0,
            remove: false,
        }
    }
}

impl OpData {
    pub fn new<S1: AsRef<str>>(key: S1, value: f64, remove: bool) -> Self {
        Self {
            key: key.as_ref().to_string(),
            value,
            remove,
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
        let key = String::read_ext(input, "key", false)?;
        let value = f64::read(input)?;
        let remove = bool::read(input)?;
        Ok(OpData {
            key, value, remove
        })
    }
}

impl Writable for OpData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key.write_ext(output, "self.key", false)?;
        self.value.write(output)?;
        self.remove.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<AlterClientQuotasRequest>("AlterClientQuotasRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "AlterClientQuotasRequest", 0);
        }
    }
}
