// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// LeaveGroupResponse, version 3.
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
}

impl ApiMessage for LeaveGroupResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        13
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Response for LeaveGroupResponse { }

impl Default for LeaveGroupResponse {
    fn default() -> Self {
        LeaveGroupResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            members: Vec::<MemberResponse>::new(),
        }
    }
}

impl LeaveGroupResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, members: Vec<MemberResponse>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            members,
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
        let members = read_array::<MemberResponse>(input, "members", false)?;
        Ok(LeaveGroupResponse {
            throttle_time_ms, error_code, members
        })
    }
}

impl Writable for LeaveGroupResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.members", &self.members, false)?;
        Ok(())
    }
}

/// MemberResponse, version 3.
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
}

impl Default for MemberResponse {
    fn default() -> Self {
        MemberResponse {
            member_id: String::from(""),
            group_instance_id: Some(String::from("")),
            error_code: 0_i16,
        }
    }
}

impl MemberResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(member_id: S1, group_instance_id: Option<S2>, error_code: i16) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            error_code,
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
        let member_id = String::read_ext(input, "member_id", false)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", false)?;
        let error_code = i16::read(input)?;
        Ok(MemberResponse {
            member_id, group_instance_id, error_code
        })
    }
}

impl Writable for MemberResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", false)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", false)?;
        self.error_code.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<LeaveGroupResponse>("LeaveGroupResponse", 3);
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
            crate::test_utils::test_java_arbitrary(&data, "LeaveGroupResponse", 3);
        }
    }
}
