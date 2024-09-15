// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaveGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// List of leaving member responses.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub members: Vec<MemberResponse>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for LeaveGroupResponse {
    fn api_key(&self) -> i16 {
        13
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Response for LeaveGroupResponse { }

impl Default for LeaveGroupResponse {
    fn default() -> Self {
        LeaveGroupResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            members: Vec::<MemberResponse>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaveGroupResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, members: Vec<MemberResponse>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            members,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leave_group_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaveGroupResponse::new(
            0_i32,
            0_i16,
            Vec::<MemberResponse>::new(),
        );
        assert_eq!(d, LeaveGroupResponse::default());
    }
}

impl Readable for LeaveGroupResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let members = read_array::<MemberResponse>(input, "members", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaveGroupResponse {
            throttle_time_ms, error_code, members, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaveGroupResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.members", &self.members, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MemberResponse {
    /// The member ID to remove from the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The group instance ID to remove from the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for MemberResponse {
    fn api_key(&self) -> i16 {
        13
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Response for MemberResponse { }

impl Default for MemberResponse {
    fn default() -> Self {
        MemberResponse {
            member_id: String::from(""),
            group_instance_id: Some(String::from("")),
            error_code: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MemberResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(member_id: S1, group_instance_id: Option<S2>, error_code: i16) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            error_code,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_member_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MemberResponse::new(
            String::from(""),
            Some(String::from("")),
            0_i16,
        );
        assert_eq!(d, MemberResponse::default());
    }
}

impl Readable for MemberResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", true)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", true)?;
        let error_code = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MemberResponse {
            member_id, group_instance_id, error_code, _unknown_tagged_fields
        })
    }
}

impl Writable for MemberResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", true)?;
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
        crate::test_utils::test_java_default::<LeaveGroupResponse>("LeaveGroupResponse", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: LeaveGroupResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: LeaveGroupResponse) {
            crate::test_utils::test_java_arbitrary(&data, "LeaveGroupResponse", 4);
        }
    }
}
