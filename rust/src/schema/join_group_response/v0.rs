// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// JoinGroupResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct JoinGroupResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The generation ID of the group.
    pub generation_id: i32,
    /// The group protocol selected by the coordinator.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub protocol_name: String,
    /// The leader of the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub leader: String,
    /// The member ID assigned by the group coordinator.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The group members.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub members: Vec<JoinGroupResponseMember>,
}

impl ApiMessage for JoinGroupResponse {
    fn api_key(&self) -> i16 {
        11
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for JoinGroupResponse { }

impl Default for JoinGroupResponse {
    fn default() -> Self {
        JoinGroupResponse {
            error_code: 0_i16,
            generation_id: -1_i32,
            protocol_name: String::from(""),
            leader: String::from(""),
            member_id: String::from(""),
            members: Vec::<JoinGroupResponseMember>::new(),
        }
    }
}

impl JoinGroupResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(error_code: i16, generation_id: i32, protocol_name: S1, leader: S2, member_id: S3, members: Vec<JoinGroupResponseMember>) -> Self {
        Self {
            error_code,
            generation_id,
            protocol_name: protocol_name.as_ref().to_string(),
            leader: leader.as_ref().to_string(),
            member_id: member_id.as_ref().to_string(),
            members,
        }
    }
}

#[cfg(test)]
mod tests_join_group_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = JoinGroupResponse::new(
            0_i16,
            -1_i32,
            String::from(""),
            String::from(""),
            String::from(""),
            Vec::<JoinGroupResponseMember>::new(),
        );
        assert_eq!(d, JoinGroupResponse::default());
    }
}

impl Readable for JoinGroupResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let generation_id = i32::read(input)?;
        let protocol_name = String::read_ext(input, "protocol_name", false)?;
        let leader = String::read_ext(input, "leader", false)?;
        let member_id = String::read_ext(input, "member_id", false)?;
        let members = read_array::<JoinGroupResponseMember>(input, "members", false)?;
        Ok(JoinGroupResponse {
            error_code, generation_id, protocol_name, leader, member_id, members
        })
    }
}

impl Writable for JoinGroupResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.generation_id.write(output)?;
        self.protocol_name.write_ext(output, "self.protocol_name", false)?;
        self.leader.write_ext(output, "self.leader", false)?;
        self.member_id.write_ext(output, "self.member_id", false)?;
        write_array(output, "self.members", &self.members, false)?;
        Ok(())
    }
}

/// JoinGroupResponseMember, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct JoinGroupResponseMember {
    /// The group member ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The group member metadata.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub metadata: Vec<u8>,
}

impl Default for JoinGroupResponseMember {
    fn default() -> Self {
        JoinGroupResponseMember {
            member_id: String::from(""),
            metadata: Vec::new(),
        }
    }
}

impl JoinGroupResponseMember {
    pub fn new<S1: AsRef<str>>(member_id: S1, metadata: Vec<u8>) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            metadata,
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
            Vec::new(),
        );
        assert_eq!(d, JoinGroupResponseMember::default());
    }
}

impl Readable for JoinGroupResponseMember {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", false)?;
        let metadata = read_bytes(input, "metadata", false)?;
        Ok(JoinGroupResponseMember {
            member_id, metadata
        })
    }
}

impl Writable for JoinGroupResponseMember {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", false)?;
        write_bytes(output, "self.metadata", &self.metadata, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<JoinGroupResponse>("JoinGroupResponse", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "JoinGroupResponse", 0);
        }
    }
}
