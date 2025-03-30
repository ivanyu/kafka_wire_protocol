// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteGroupsResponse, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The deletion results.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<DeletableGroupResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DeleteGroupsResponse {
    fn api_key(&self) -> i16 {
        42
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Response for DeleteGroupsResponse { }

impl Default for DeleteGroupsResponse {
    fn default() -> Self {
        DeleteGroupsResponse {
            throttle_time_ms: 0_i32,
            results: Vec::<DeletableGroupResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeleteGroupsResponse {
    pub fn new(throttle_time_ms: i32, results: Vec<DeletableGroupResult>) -> Self {
        Self {
            throttle_time_ms,
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_delete_groups_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteGroupsResponse::new(
            0_i32,
            Vec::<DeletableGroupResult>::new(),
        );
        assert_eq!(d, DeleteGroupsResponse::default());
    }
}

impl Readable for DeleteGroupsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let results = read_array::<DeletableGroupResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeleteGroupsResponse {
            throttle_time_ms, results, _unknown_tagged_fields
        })
    }
}

impl Writable for DeleteGroupsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DeletableGroupResult, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeletableGroupResult {
    /// The group id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DeletableGroupResult {
    fn default() -> Self {
        DeletableGroupResult {
            group_id: String::from(""),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DeletableGroupResult {
    pub fn new<S1: AsRef<str>>(group_id: S1, error_code: i16) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_deletable_group_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeletableGroupResult::new(
            String::from(""),
            0_i16,
        );
        assert_eq!(d, DeletableGroupResult::default());
    }
}

impl Readable for DeletableGroupResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DeletableGroupResult {
            group_id, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for DeletableGroupResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.error_code.write(output)?;
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
        crate::test_utils::test_java_default::<DeleteGroupsResponse>("DeleteGroupsResponse", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteGroupsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteGroupsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteGroupsResponse", 2);
        }
    }
}
