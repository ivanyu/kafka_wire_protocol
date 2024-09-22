// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ListGroupsResponse, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Each group in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<ListedGroup>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ListGroupsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        16
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Response for ListGroupsResponse { }

impl Default for ListGroupsResponse {
    fn default() -> Self {
        ListGroupsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            groups: Vec::<ListedGroup>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListGroupsResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, groups: Vec<ListedGroup>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            groups,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_list_groups_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListGroupsResponse::new(
            0_i32,
            0_i16,
            Vec::<ListedGroup>::new(),
        );
        assert_eq!(d, ListGroupsResponse::default());
    }
}

impl Readable for ListGroupsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let groups = read_array::<ListedGroup>(input, "groups", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListGroupsResponse {
            throttle_time_ms, error_code, groups, _unknown_tagged_fields
        })
    }
}

impl Writable for ListGroupsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.groups", &self.groups, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ListedGroup, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ListedGroup {
    /// The group ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The group protocol type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub protocol_type: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ListedGroup {
    fn default() -> Self {
        ListedGroup {
            group_id: String::from(""),
            protocol_type: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ListedGroup {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(group_id: S1, protocol_type: S2) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            protocol_type: protocol_type.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_listed_group_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ListedGroup::new(
            String::from(""),
            String::from(""),
        );
        assert_eq!(d, ListedGroup::default());
    }
}

impl Readable for ListedGroup {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let protocol_type = String::read_ext(input, "protocol_type", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ListedGroup {
            group_id, protocol_type, _unknown_tagged_fields
        })
    }
}

impl Writable for ListedGroup {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.protocol_type.write_ext(output, "self.protocol_type", true)?;
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
        crate::test_utils::test_java_default::<ListGroupsResponse>("ListGroupsResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ListGroupsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ListGroupsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ListGroupsResponse", 3);
        }
    }
}
