// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_nullable_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// MetadataRequest, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataRequest {
    /// The topics to fetch metadata for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topics: Option<Vec<MetadataRequestTopic>>,
    /// If this is true, the broker may auto-create topics that we requested which do not already exist, if it is configured to do so.
    pub allow_auto_topic_creation: bool,
    /// Whether to include cluster authorized operations.
    pub include_cluster_authorized_operations: bool,
    /// Whether to include topic authorized operations.
    pub include_topic_authorized_operations: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for MetadataRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        3
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        9
    }
}

impl Request for MetadataRequest { }

impl Default for MetadataRequest {
    fn default() -> Self {
        MetadataRequest {
            topics: Some(Vec::<MetadataRequestTopic>::new()),
            allow_auto_topic_creation: true,
            include_cluster_authorized_operations: false,
            include_topic_authorized_operations: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MetadataRequest {
    pub fn new(topics: Option<Vec<MetadataRequestTopic>>, allow_auto_topic_creation: bool, include_cluster_authorized_operations: bool, include_topic_authorized_operations: bool) -> Self {
        Self {
            topics,
            allow_auto_topic_creation,
            include_cluster_authorized_operations,
            include_topic_authorized_operations,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_metadata_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MetadataRequest::new(
            Some(Vec::<MetadataRequestTopic>::new()),
            true,
            false,
            false,
        );
        assert_eq!(d, MetadataRequest::default());
    }
}

impl Readable for MetadataRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_nullable_array::<MetadataRequestTopic>(input, "topics", true)?;
        let allow_auto_topic_creation = bool::read(input)?;
        let include_cluster_authorized_operations = bool::read(input)?;
        let include_topic_authorized_operations = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MetadataRequest {
            topics, allow_auto_topic_creation, include_cluster_authorized_operations, include_topic_authorized_operations, _unknown_tagged_fields
        })
    }
}

impl Writable for MetadataRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_nullable_array(output, "self.topics", self.topics.as_deref(), true)?;
        self.allow_auto_topic_creation.write(output)?;
        self.include_cluster_authorized_operations.write(output)?;
        self.include_topic_authorized_operations.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// MetadataRequestTopic, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for MetadataRequestTopic {
    fn default() -> Self {
        MetadataRequestTopic {
            name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MetadataRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1) -> Self {
        Self {
            name: name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_metadata_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MetadataRequestTopic::new(
            String::from(""),
        );
        assert_eq!(d, MetadataRequestTopic::default());
    }
}

impl Readable for MetadataRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MetadataRequestTopic {
            name, _unknown_tagged_fields
        })
    }
}

impl Writable for MetadataRequestTopic {
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
        crate::test_utils::test_java_default::<MetadataRequest>("MetadataRequest", 9);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: MetadataRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: MetadataRequest) {
            crate::test_utils::test_java_arbitrary(&data, "MetadataRequest", 9);
        }
    }
}
