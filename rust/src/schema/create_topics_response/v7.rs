// This file was generated. Do not edit.

use std::io::{Cursor, Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateTopicsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Results for each topic we tried to create.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<CreatableTopicResult>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreateTopicsResponse {
    fn api_key(&self) -> i16 {
        19
    }
    
    fn version(&self) -> i16 {
        7
    }
}

impl Response for CreateTopicsResponse { }

impl Default for CreateTopicsResponse {
    fn default() -> Self {
        CreateTopicsResponse {
            throttle_time_ms: 0_i32,
            topics: Vec::<CreatableTopicResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreateTopicsResponse {
    pub fn new(throttle_time_ms: i32, topics: Vec<CreatableTopicResult>) -> Self {
        Self {
            throttle_time_ms,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_create_topics_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateTopicsResponse::new(
            0_i32,
            Vec::<CreatableTopicResult>::new(),
        );
        assert_eq!(d, CreateTopicsResponse::default());
    }
}

impl Readable for CreateTopicsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let topics = read_array::<CreatableTopicResult>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreateTopicsResponse {
            throttle_time_ms, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for CreateTopicsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatableTopicResult {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The unique topic ID
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Optional topic config error returned if configs are not returned in the response.
    pub topic_config_error_code: i16,
    /// Number of partitions of the topic.
    pub num_partitions: i32,
    /// Replication factor of the topic.
    pub replication_factor: i16,
    /// Configuration of the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub configs: Option<Vec<CreatableTopicConfigs>>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreatableTopicResult {
    fn api_key(&self) -> i16 {
        19
    }
    
    fn version(&self) -> i16 {
        7
    }
}

impl Response for CreatableTopicResult { }

impl Default for CreatableTopicResult {
    fn default() -> Self {
        CreatableTopicResult {
            name: String::from(""),
            topic_id: Uuid::nil(),
            error_code: 0_i16,
            error_message: Some(String::from("")),
            topic_config_error_code: 0_i16,
            num_partitions: -1_i32,
            replication_factor: -1_i16,
            configs: Some(Vec::<CreatableTopicConfigs>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatableTopicResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, topic_id: Uuid, error_code: i16, error_message: Option<S2>, topic_config_error_code: i16, num_partitions: i32, replication_factor: i16, configs: Option<Vec<CreatableTopicConfigs>>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            topic_id,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            topic_config_error_code,
            num_partitions,
            replication_factor,
            configs,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_creatable_topic_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatableTopicResult::new(
            String::from(""),
            Uuid::nil(),
            0_i16,
            Some(String::from("")),
            0_i16,
            -1_i32,
            -1_i16,
            Some(Vec::<CreatableTopicConfigs>::new()),
        );
        assert_eq!(d, CreatableTopicResult::default());
    }
}

impl Readable for CreatableTopicResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let topic_id = Uuid::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let mut topic_config_error_code = 0_i16;
        let num_partitions = i32::read(input)?;
        let replication_factor = i16::read(input)?;
        let configs = read_nullable_array::<CreatableTopicConfigs>(input, "configs", true)?;
        let tagged_fields_callback = |tag: i32, tag_data: &[u8]| {
            match tag {
                0 => {
                    let mut cur = Cursor::new(tag_data);
                    topic_config_error_code = i16::read(&mut cur)?;
                    Ok(true)
                },
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatableTopicResult {
            name, topic_id, error_code, error_message, topic_config_error_code, num_partitions, replication_factor, configs, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatableTopicResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.topic_id.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        self.num_partitions.write(output)?;
        self.replication_factor.write(output)?;
        write_nullable_array(output, "self.configs", self.configs.as_deref(), true)?;
        let mut known_tagged_fields = Vec::<RawTaggedField>::new();
        if self.topic_config_error_code != 0_i16 {
            let mut cur = Cursor::new(Vec::<u8>::new());
            self.topic_config_error_code.write(&mut cur)?;
            known_tagged_fields.push(RawTaggedField { tag: 0, data: cur.into_inner() });
        }
        write_tagged_fields(output, &known_tagged_fields, &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatableTopicConfigs {
    /// The configuration name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The configuration value.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub value: Option<String>,
    /// True if the configuration is read-only.
    pub read_only: bool,
    /// The configuration source.
    pub config_source: i8,
    /// True if this configuration is sensitive.
    pub is_sensitive: bool,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreatableTopicConfigs {
    fn api_key(&self) -> i16 {
        19
    }
    
    fn version(&self) -> i16 {
        7
    }
}

impl Response for CreatableTopicConfigs { }

impl Default for CreatableTopicConfigs {
    fn default() -> Self {
        CreatableTopicConfigs {
            name: String::from(""),
            value: Some(String::from("")),
            read_only: false,
            config_source: -1_i8,
            is_sensitive: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatableTopicConfigs {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, value: Option<S2>, read_only: bool, config_source: i8, is_sensitive: bool) -> Self {
        Self {
            name: name.as_ref().to_string(),
            value: value.map(|s| s.as_ref().to_string()),
            read_only,
            config_source,
            is_sensitive,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_creatable_topic_configs_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatableTopicConfigs::new(
            String::from(""),
            Some(String::from("")),
            false,
            -1_i8,
            false,
        );
        assert_eq!(d, CreatableTopicConfigs::default());
    }
}

impl Readable for CreatableTopicConfigs {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let value = Option::<String>::read_ext(input, "value", true)?;
        let read_only = bool::read(input)?;
        let config_source = i8::read(input)?;
        let is_sensitive = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatableTopicConfigs {
            name, value, read_only, config_source, is_sensitive, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatableTopicConfigs {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.value.write_ext(output, "self.value", true)?;
        self.read_only.write(output)?;
        self.config_source.write(output)?;
        self.is_sensitive.write(output)?;
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
        crate::test_utils::test_java_default::<CreateTopicsResponse>("CreateTopicsResponse", 7);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreateTopicsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreateTopicsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "CreateTopicsResponse", 7);
        }
    }
}
