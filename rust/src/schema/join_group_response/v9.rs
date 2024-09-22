// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// JoinGroupResponse, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct JoinGroupResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The generation ID of the group.
    pub generation_id: i32,
    /// The group protocol name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub protocol_type: Option<String>,
    /// The group protocol selected by the coordinator.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub protocol_name: Option<String>,
    /// The leader of the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub leader: String,
    /// True if the leader must skip running the assignment.
    pub skip_assignment: bool,
    /// The member ID assigned by the group coordinator.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub members: Vec<JoinGroupResponseMember>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for JoinGroupResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        11
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        9
    }
}

impl Response for JoinGroupResponse { }

impl Default for JoinGroupResponse {
    fn default() -> Self {
        JoinGroupResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            generation_id: -1_i32,
            protocol_type: None,
            protocol_name: Some(String::from("")),
            leader: String::from(""),
            skip_assignment: false,
            member_id: String::from(""),
            members: Vec::<JoinGroupResponseMember>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl JoinGroupResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>>(throttle_time_ms: i32, error_code: i16, generation_id: i32, protocol_type: Option<S1>, protocol_name: Option<S2>, leader: S3, skip_assignment: bool, member_id: S4, members: Vec<JoinGroupResponseMember>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            generation_id,
            protocol_type: protocol_type.map(|s| s.as_ref().to_string()),
            protocol_name: protocol_name.map(|s| s.as_ref().to_string()),
            leader: leader.as_ref().to_string(),
            skip_assignment,
            member_id: member_id.as_ref().to_string(),
            members,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_join_group_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = JoinGroupResponse::new(
            0_i32,
            0_i16,
            -1_i32,
            None::<String>,
            Some(String::from("")),
            String::from(""),
            false,
            String::from(""),
            Vec::<JoinGroupResponseMember>::new(),
        );
        assert_eq!(d, JoinGroupResponse::default());
    }
}

impl Readable for JoinGroupResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let generation_id = i32::read(input)?;
        let protocol_type = Option::<String>::read_ext(input, "protocol_type", true)?;
        let protocol_name = Option::<String>::read_ext(input, "protocol_name", true)?;
        let leader = String::read_ext(input, "leader", true)?;
        let skip_assignment = bool::read(input)?;
        let member_id = String::read_ext(input, "member_id", true)?;
        let members = read_array::<JoinGroupResponseMember>(input, "members", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(JoinGroupResponse {
            throttle_time_ms, error_code, generation_id, protocol_type, protocol_name, leader, skip_assignment, member_id, members, _unknown_tagged_fields
        })
    }
}

impl Writable for JoinGroupResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.generation_id.write(output)?;
        self.protocol_type.write_ext(output, "self.protocol_type", true)?;
        self.protocol_name.write_ext(output, "self.protocol_name", true)?;
        self.leader.write_ext(output, "self.leader", true)?;
        self.skip_assignment.write(output)?;
        self.member_id.write_ext(output, "self.member_id", true)?;
        write_array(output, "self.members", &self.members, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// JoinGroupResponseMember, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct JoinGroupResponseMember {
    /// The group member ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The unique identifier of the consumer instance provided by end user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// The group member metadata.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub metadata: Vec<u8>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for JoinGroupResponseMember {
    fn default() -> Self {
        JoinGroupResponseMember {
            member_id: String::from(""),
            group_instance_id: None,
            metadata: Vec::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl JoinGroupResponseMember {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(member_id: S1, group_instance_id: Option<S2>, metadata: Vec<u8>) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            metadata,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_join_group_response_member_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = JoinGroupResponseMember::new(
            String::from(""),
            None::<String>,
            Vec::new(),
        );
        assert_eq!(d, JoinGroupResponseMember::default());
    }
}

impl Readable for JoinGroupResponseMember {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", true)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", true)?;
        let metadata = read_bytes(input, "metadata", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(JoinGroupResponseMember {
            member_id, group_instance_id, metadata, _unknown_tagged_fields
        })
    }
}

impl Writable for JoinGroupResponseMember {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", true)?;
        write_bytes(output, "self.metadata", &self.metadata, true)?;
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
        crate::test_utils::test_java_default::<JoinGroupResponse>("JoinGroupResponse", 9);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: JoinGroupResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: JoinGroupResponse) {
            crate::test_utils::test_java_arbitrary(&data, "JoinGroupResponse", 9);
        }
    }
}
