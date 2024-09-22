// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_nullable_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// MetadataRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataRequest {
    /// The topics to fetch metadata for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topics: Option<Vec<MetadataRequestTopic>>,
}

impl ApiMessage for MetadataRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        3
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        1
    }
}

impl Request for MetadataRequest { }

impl Default for MetadataRequest {
    fn default() -> Self {
        MetadataRequest {
            topics: Some(Vec::<MetadataRequestTopic>::new()),
        }
    }
}

impl MetadataRequest {
    pub fn new(topics: Option<Vec<MetadataRequestTopic>>) -> Self {
        Self {
            topics,
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
        );
        assert_eq!(d, MetadataRequest::default());
    }
}

impl Readable for MetadataRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_nullable_array::<MetadataRequestTopic>(input, "topics", false)?;
        Ok(MetadataRequest {
            topics
        })
    }
}

impl Writable for MetadataRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_nullable_array(output, "self.topics", self.topics.as_deref(), false)?;
        Ok(())
    }
}

/// MetadataRequestTopic, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MetadataRequestTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
}

impl Default for MetadataRequestTopic {
    fn default() -> Self {
        MetadataRequestTopic {
            name: String::from(""),
        }
    }
}

impl MetadataRequestTopic {
    pub fn new<S1: AsRef<str>>(name: S1) -> Self {
        Self {
            name: name.as_ref().to_string(),
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
        let name = String::read_ext(input, "name", false)?;
        Ok(MetadataRequestTopic {
            name
        })
    }
}

impl Writable for MetadataRequestTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<MetadataRequest>("MetadataRequest", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "MetadataRequest", 1);
        }
    }
}
