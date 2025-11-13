// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteShareGroupOffsetsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteShareGroupOffsetsRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The topics to delete offsets for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<DeleteShareGroupOffsetsRequestTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteShareGroupOffsetsRequest {
    fn api_key(&self) -> i16 {
        92
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DeleteShareGroupOffsetsRequest { }

impl Default for DeleteShareGroupOffsetsRequest {
    fn default() -> Self {
        DeleteShareGroupOffsetsRequest {
            group_id: String::from(""),
            topics: Vec::<DeleteShareGroupOffsetsRequestTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteShareGroupOffsetsRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<DeleteShareGroupOffsetsRequestTopic>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_share_group_offsets_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteShareGroupOffsetsRequest::new(
            String::from(""),
            Vec::<DeleteShareGroupOffsetsRequestTopic>::new(),
        );
        assert_eq!(d, DeleteShareGroupOffsetsRequest::default());
    }
}

impl Readable for DeleteShareGroupOffsetsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_array::<DeleteShareGroupOffsetsRequestTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteShareGroupOffsetsRequest {
            group_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteShareGroupOffsetsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteShareGroupOffsetsRequestTopic, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteShareGroupOffsetsRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteShareGroupOffsetsRequestTopic {
    fn default() -> Self {
        DeleteShareGroupOffsetsRequestTopic {
            topic_name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteShareGroupOffsetsRequestTopic {
    pub fn new<S1: AsRef<str>>(topic_name: S1) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_share_group_offsets_request_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteShareGroupOffsetsRequestTopic::new(
            String::from(""),
        );
        assert_eq!(d, DeleteShareGroupOffsetsRequestTopic::default());
    }
}

impl Readable for DeleteShareGroupOffsetsRequestTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteShareGroupOffsetsRequestTopic {
            topic_name, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteShareGroupOffsetsRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", true)?;
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
        crate::test_utils::test_java_default::<DeleteShareGroupOffsetsRequest>("DeleteShareGroupOffsetsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteShareGroupOffsetsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteShareGroupOffsetsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteShareGroupOffsetsRequest", 0);
        }
    }
}
