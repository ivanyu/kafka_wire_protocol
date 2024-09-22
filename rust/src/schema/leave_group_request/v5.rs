// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// LeaveGroupRequest, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaveGroupRequest {
    /// The ID of the group to leave.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// List of leaving member identities.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub members: Vec<MemberIdentity>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for LeaveGroupRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        13
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        5
    }
}

impl Request for LeaveGroupRequest { }

impl Default for LeaveGroupRequest {
    fn default() -> Self {
        LeaveGroupRequest {
            group_id: String::from(""),
            members: Vec::<MemberIdentity>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaveGroupRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, members: Vec<MemberIdentity>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            members,
            _unknown_tagged_fields: vec![],
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
        let group_id = String::read_ext(input, "group_id", true)?;
        let members = read_array::<MemberIdentity>(input, "members", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaveGroupRequest {
            group_id, members, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaveGroupRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_array(output, "self.members", &self.members, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// MemberIdentity, version 5.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct MemberIdentity {
    /// The member ID to remove from the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The group instance ID to remove from the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// The reason why the member left the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub reason: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for MemberIdentity {
    fn default() -> Self {
        MemberIdentity {
            member_id: String::from(""),
            group_instance_id: None,
            reason: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl MemberIdentity {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(member_id: S1, group_instance_id: Option<S2>, reason: Option<S3>) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            reason: reason.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
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
            None::<String>,
        );
        assert_eq!(d, MemberIdentity::default());
    }
}

impl Readable for MemberIdentity {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", true)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", true)?;
        let reason = Option::<String>::read_ext(input, "reason", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(MemberIdentity {
            member_id, group_instance_id, reason, _unknown_tagged_fields
        })
    }
}

impl Writable for MemberIdentity {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", true)?;
        self.reason.write_ext(output, "self.reason", true)?;
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
        crate::test_utils::test_java_default::<LeaveGroupRequest>("LeaveGroupRequest", 5);
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
            crate::test_utils::test_java_arbitrary(&data, "LeaveGroupRequest", 5);
        }
    }
}
