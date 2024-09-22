// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AlterPartitionReassignmentsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterPartitionReassignmentsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top-level error code, or 0 if there was no error.
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The responses to topics to reassign.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub responses: Vec<ReassignableTopicResponse>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterPartitionReassignmentsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        45
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for AlterPartitionReassignmentsResponse { }

impl Default for AlterPartitionReassignmentsResponse {
    fn default() -> Self {
        AlterPartitionReassignmentsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            responses: Vec::<ReassignableTopicResponse>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterPartitionReassignmentsResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, responses: Vec<ReassignableTopicResponse>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            responses,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_partition_reassignments_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterPartitionReassignmentsResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
            Vec::<ReassignableTopicResponse>::new(),
        );
        assert_eq!(d, AlterPartitionReassignmentsResponse::default());
    }
}

impl Readable for AlterPartitionReassignmentsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let responses = read_array::<ReassignableTopicResponse>(input, "responses", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterPartitionReassignmentsResponse {
            throttle_time_ms, error_code, error_message, responses, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterPartitionReassignmentsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.responses", &self.responses, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ReassignableTopicResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReassignableTopicResponse {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The responses to partitions to reassign
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<ReassignablePartitionResponse>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ReassignableTopicResponse {
    fn default() -> Self {
        ReassignableTopicResponse {
            name: String::from(""),
            partitions: Vec::<ReassignablePartitionResponse>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReassignableTopicResponse {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<ReassignablePartitionResponse>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_reassignable_topic_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReassignableTopicResponse::new(
            String::from(""),
            Vec::<ReassignablePartitionResponse>::new(),
        );
        assert_eq!(d, ReassignableTopicResponse::default());
    }
}

impl Readable for ReassignableTopicResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<ReassignablePartitionResponse>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReassignableTopicResponse {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for ReassignableTopicResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ReassignablePartitionResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReassignablePartitionResponse {
    /// The partition index.
    pub partition_index: i32,
    /// The error code for this partition, or 0 if there was no error.
    pub error_code: i16,
    /// The error message for this partition, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ReassignablePartitionResponse {
    fn default() -> Self {
        ReassignablePartitionResponse {
            partition_index: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ReassignablePartitionResponse {
    pub fn new<S1: AsRef<str>>(partition_index: i32, error_code: i16, error_message: Option<S1>) -> Self {
        Self {
            partition_index,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_reassignable_partition_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReassignablePartitionResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, ReassignablePartitionResponse::default());
    }
}

impl Readable for ReassignablePartitionResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ReassignablePartitionResponse {
            partition_index, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for ReassignablePartitionResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
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
        crate::test_utils::test_java_default::<AlterPartitionReassignmentsResponse>("AlterPartitionReassignmentsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterPartitionReassignmentsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterPartitionReassignmentsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "AlterPartitionReassignmentsResponse", 0);
        }
    }
}
