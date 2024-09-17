// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// LeaveGroupRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaveGroupRequest {
    /// The ID of the group to leave.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// List of leaving member identities.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub members: Vec<MemberIdentity>,
}

impl ApiMessage for LeaveGroupRequest {
    fn api_key(&self) -> i16 {
        13
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for LeaveGroupRequest { }

impl Default for LeaveGroupRequest {
    fn default() -> Self {
        LeaveGroupRequest {
            group_id: String::from(""),
            members: Vec::<MemberIdentity>::new(),
        }
    }
}

impl LeaveGroupRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, members: Vec<MemberIdentity>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            members,
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
            Vec::<MemberIdentity>::new(),
        );
        assert_eq!(d, LeaveGroupRequest::default());
    }
}

impl Readable for LeaveGroupRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", false)?;
        let members = read_array::<MemberIdentity>(input, "members", false)?;
        Ok(LeaveGroupRequest {
            group_id, members
        })
    }
}

impl Writable for LeaveGroupRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", false)?;
        write_array(output, "self.members", &self.members, false)?;
        Ok(())
    }
}

/// MemberIdentity, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MemberIdentity {
    /// The member ID to remove from the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The group instance ID to remove from the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
}

impl Default for MemberIdentity {
    fn default() -> Self {
        MemberIdentity {
            member_id: String::from(""),
            group_instance_id: None,
        }
    }
}

impl MemberIdentity {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(member_id: S1, group_instance_id: Option<S2>) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
        }
    }
}

#[cfg(test)]
mod tests_member_identity_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = MemberIdentity::new(
            String::from(""),
            None::<String>,
        );
        assert_eq!(d, MemberIdentity::default());
    }
}

impl Readable for MemberIdentity {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", false)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", false)?;
        Ok(MemberIdentity {
            member_id, group_instance_id
        })
    }
}

impl Writable for MemberIdentity {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", false)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<LeaveGroupRequest>("LeaveGroupRequest", 3);
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
            crate::test_utils::test_java_arbitrary(&data, "LeaveGroupRequest", 3);
        }
    }
}
