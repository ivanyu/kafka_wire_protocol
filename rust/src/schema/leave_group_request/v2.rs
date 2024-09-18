// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// LeaveGroupRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaveGroupRequest {
    /// The ID of the group to leave.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The member ID to remove from the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
}

impl ApiMessage for LeaveGroupRequest {
    fn api_key(&self) -> i16 {
        13
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for LeaveGroupRequest { }

impl Default for LeaveGroupRequest {
    fn default() -> Self {
        LeaveGroupRequest {
            group_id: String::from(""),
            member_id: String::from(""),
        }
    }
}

impl LeaveGroupRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(group_id: S1, member_id: S2) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            member_id: member_id.as_ref().to_string(),
        }
    }
}

#[cfg(test)]
mod tests_leave_group_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaveGroupRequest::new(
            String::from(""),
            String::from(""),
        );
        assert_eq!(d, LeaveGroupRequest::default());
    }
}

impl Readable for LeaveGroupRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", false)?;
        let member_id = String::read_ext(input, "member_id", false)?;
        Ok(LeaveGroupRequest {
            group_id, member_id
        })
    }
}

impl Writable for LeaveGroupRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", false)?;
        self.member_id.write_ext(output, "self.member_id", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<LeaveGroupRequest>("LeaveGroupRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: LeaveGroupRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: LeaveGroupRequest) {
            crate::test_utils::test_java_arbitrary(&data, "LeaveGroupRequest", 2);
        }
    }
}
