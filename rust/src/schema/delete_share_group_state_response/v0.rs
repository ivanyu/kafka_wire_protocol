// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteShareGroupStateResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteShareGroupStateResponse {
    /// The delete results.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<DeleteStateResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteShareGroupStateResponse {
    fn api_key(&self) -> i16 {
        86
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for DeleteShareGroupStateResponse { }

impl Default for DeleteShareGroupStateResponse {
    fn default() -> Self {
        DeleteShareGroupStateResponse {
            results: Vec::<DeleteStateResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteShareGroupStateResponse {
    pub fn new(results: Vec<DeleteStateResult>) -> Self {
        Self {
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_share_group_state_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteShareGroupStateResponse::new(
            Vec::<DeleteStateResult>::new(),
        );
        assert_eq!(d, DeleteShareGroupStateResponse::default());
    }
}

impl Readable for DeleteShareGroupStateResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let results = read_array::<DeleteStateResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteShareGroupStateResponse {
            results, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteShareGroupStateResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeleteStateResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteStateResult {
    /// The topic identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The results for the partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeleteStateResult {
    fn default() -> Self {
        DeleteStateResult {
            topic_id: Uuid::nil(),
            partitions: Vec::<PartitionResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteStateResult {
    pub fn new(topic_id: Uuid, partitions: Vec<PartitionResult>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_state_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteStateResult::new(
            Uuid::nil(),
            Vec::<PartitionResult>::new(),
        );
        assert_eq!(d, DeleteStateResult::default());
    }
}

impl Readable for DeleteStateResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<PartitionResult>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteStateResult {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteStateResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionResult, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionResult {
    /// The partition index.
    pub partition: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionResult {
    fn default() -> Self {
        PartitionResult {
            partition: 0_i32,
            error_code: 0_i16,
            error_message: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionResult {
    pub fn new<S1: AsRef<str>>(partition: i32, error_code: i16, error_message: Option<S1>) -> Self {
        Self {
            partition,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_partition_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionResult::new(
            0_i32,
            0_i16,
            None::<String>,
        );
        assert_eq!(d, PartitionResult::default());
    }
}

impl Readable for PartitionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionResult {
            partition, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
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
        crate::test_utils::test_java_default::<DeleteShareGroupStateResponse>("DeleteShareGroupStateResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteShareGroupStateResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteShareGroupStateResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteShareGroupStateResponse", 0);
        }
    }
}
