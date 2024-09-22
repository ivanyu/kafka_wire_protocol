// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// OffsetFetchRequest, version 8.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchRequest {
    /// Each group we would like to fetch offsets for
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<OffsetFetchRequestGroup>,
    /// Whether broker should hold on returning unstable offsets but set a retriable error code for the partitions.
    pub require_stable: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for OffsetFetchRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        9
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        8
    }
}

impl Request for OffsetFetchRequest { }

impl Default for OffsetFetchRequest {
    fn default() -> Self {
        OffsetFetchRequest {
            groups: Vec::<OffsetFetchRequestGroup>::new(),
            require_stable: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchRequest {
    pub fn new(groups: Vec<OffsetFetchRequestGroup>, require_stable: bool) -> Self {
        Self {
            groups,
            require_stable,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchRequest::new(
            Vec::<OffsetFetchRequestGroup>::new(),
            false,
        );
        assert_eq!(d, OffsetFetchRequest::default());
    }
}

impl Readable for OffsetFetchRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let groups = read_array::<OffsetFetchRequestGroup>(input, "groups", true)?;
        let require_stable = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchRequest {
            groups, require_stable, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.groups", &self.groups, true)?;
        self.require_stable.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetFetchRequestGroup, version 8.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchRequestGroup {
    /// The group ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topics: Option<Vec<OffsetFetchRequestTopics>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetFetchRequestGroup {
    fn default() -> Self {
        OffsetFetchRequestGroup {
            group_id: String::from(""),
            topics: Some(Vec::<OffsetFetchRequestTopics>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchRequestGroup {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Option<Vec<OffsetFetchRequestTopics>>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_request_group_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchRequestGroup::new(
            String::from(""),
            Some(Vec::<OffsetFetchRequestTopics>::new()),
        );
        assert_eq!(d, OffsetFetchRequestGroup::default());
    }
}

impl Readable for OffsetFetchRequestGroup {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_nullable_array::<OffsetFetchRequestTopics>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchRequestGroup {
            group_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchRequestGroup {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_nullable_array(output, "self.topics", self.topics.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// OffsetFetchRequestTopics, version 8.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct OffsetFetchRequestTopics {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partition indexes we would like to fetch offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_indexes: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for OffsetFetchRequestTopics {
    fn default() -> Self {
        OffsetFetchRequestTopics {
            name: String::from(""),
            partition_indexes: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl OffsetFetchRequestTopics {
    pub fn new<S1: AsRef<str>>(name: S1, partition_indexes: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_indexes,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_offset_fetch_request_topics_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = OffsetFetchRequestTopics::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, OffsetFetchRequestTopics::default());
    }
}

impl Readable for OffsetFetchRequestTopics {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partition_indexes = read_array::<i32>(input, "partition_indexes", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(OffsetFetchRequestTopics {
            name, partition_indexes, _unknown_tagged_fields
        })
    }
}

impl Writable for OffsetFetchRequestTopics {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partition_indexes", &self.partition_indexes, true)?;
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
        crate::test_utils::test_java_default::<OffsetFetchRequest>("OffsetFetchRequest", 8);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: OffsetFetchRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: OffsetFetchRequest) {
            crate::test_utils::test_java_arbitrary(&data, "OffsetFetchRequest", 8);
        }
    }
}
