// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_nullable_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

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
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for MetadataRequest {
    fn api_key(&self) -> i16 {
        3
    }
    
    fn version(&self) -> i16 {
        10
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataRequestTopic {
    /// The topic id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub name: Option<String>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for MetadataRequestTopic {
    fn api_key(&self) -> i16 {
        3
    }
    
    fn version(&self) -> i16 {
        10
    }
}

impl Request for MetadataRequestTopic { }

impl Default for MetadataRequestTopic {
    fn default() -> Self {
        MetadataRequestTopic {
            topic_id: Uuid::nil(),
            name: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MetadataRequestTopic {
    pub fn new<S1: AsRef<str>>(topic_id: Uuid, name: Option<S1>) -> Self {
        Self {
            topic_id,
            name: name.map(|s| s.as_ref().to_string()),
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
            Uuid::nil(),
            Some(String::from("")),
        );
        assert_eq!(d, MetadataRequestTopic::default());
    }
}

impl Readable for MetadataRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let name = Option::<String>::read_ext(input, "name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MetadataRequestTopic {
            topic_id, name, _unknown_tagged_fields
        })
    }
}

impl Writable for MetadataRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
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
        crate::test_utils::test_java_default::<MetadataRequest>("MetadataRequest", 10);
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
            crate::test_utils::test_java_arbitrary(&data, "MetadataRequest", 10);
        }
    }
}
