// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// DescribeGroupsResponse, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each described group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<DescribedGroup>,
}

impl ApiMessage for DescribeGroupsResponse {
    fn api_key(&self) -> i16 {
        15
    }
    
    fn version(&self) -> i16 {
        4
    }
}

impl Response for DescribeGroupsResponse { }

impl Default for DescribeGroupsResponse {
    fn default() -> Self {
        DescribeGroupsResponse {
            throttle_time_ms: 0_i32,
            groups: Vec::<DescribedGroup>::new(),
        }
    }
}

impl DescribeGroupsResponse {
    pub fn new(throttle_time_ms: i32, groups: Vec<DescribedGroup>) -> Self {
        Self {
            throttle_time_ms,
            groups,
        }
    }
}

#[cfg(test)]
mod tests_describe_groups_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeGroupsResponse::new(
            0_i32,
            Vec::<DescribedGroup>::new(),
        );
        assert_eq!(d, DescribeGroupsResponse::default());
    }
}

impl Readable for DescribeGroupsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let groups = read_array::<DescribedGroup>(input, "groups", false)?;
        Ok(DescribeGroupsResponse {
            throttle_time_ms, groups
        })
    }
}

impl Writable for DescribeGroupsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.groups", &self.groups, false)?;
        Ok(())
    }
}

/// DescribedGroup, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribedGroup {
    /// The describe error, or 0 if there was no error.
    pub error_code: i16,
    /// The group ID string.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The group state string, or the empty string.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_state: String,
    /// The group protocol type, or the empty string.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub protocol_type: String,
    /// The group protocol data, or the empty string.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub protocol_data: String,
    /// The group members.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub members: Vec<DescribedGroupMember>,
    /// 32-bit bitfield to represent authorized operations for this group.
    pub authorized_operations: i32,
}

impl Default for DescribedGroup {
    fn default() -> Self {
        DescribedGroup {
            error_code: 0_i16,
            group_id: String::from(""),
            group_state: String::from(""),
            protocol_type: String::from(""),
            protocol_data: String::from(""),
            members: Vec::<DescribedGroupMember>::new(),
            authorized_operations: -2147483648_i32,
        }
    }
}

impl DescribedGroup {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>>(error_code: i16, group_id: S1, group_state: S2, protocol_type: S3, protocol_data: S4, members: Vec<DescribedGroupMember>, authorized_operations: i32) -> Self {
        Self {
            error_code,
            group_id: group_id.as_ref().to_string(),
            group_state: group_state.as_ref().to_string(),
            protocol_type: protocol_type.as_ref().to_string(),
            protocol_data: protocol_data.as_ref().to_string(),
            members,
            authorized_operations,
        }
    }
}

#[cfg(test)]
mod tests_described_group_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribedGroup::new(
            0_i16,
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            Vec::<DescribedGroupMember>::new(),
            -2147483648_i32,
        );
        assert_eq!(d, DescribedGroup::default());
    }
}

impl Readable for DescribedGroup {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let group_id = String::read_ext(input, "group_id", false)?;
        let group_state = String::read_ext(input, "group_state", false)?;
        let protocol_type = String::read_ext(input, "protocol_type", false)?;
        let protocol_data = String::read_ext(input, "protocol_data", false)?;
        let members = read_array::<DescribedGroupMember>(input, "members", false)?;
        let authorized_operations = i32::read(input)?;
        Ok(DescribedGroup {
            error_code, group_id, group_state, protocol_type, protocol_data, members, authorized_operations
        })
    }
}

impl Writable for DescribedGroup {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.group_id.write_ext(output, "self.group_id", false)?;
        self.group_state.write_ext(output, "self.group_state", false)?;
        self.protocol_type.write_ext(output, "self.protocol_type", false)?;
        self.protocol_data.write_ext(output, "self.protocol_data", false)?;
        write_array(output, "self.members", &self.members, false)?;
        self.authorized_operations.write(output)?;
        Ok(())
    }
}

/// DescribedGroupMember, version 4.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribedGroupMember {
    /// The member id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The unique identifier of the consumer instance provided by end user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// The client ID used in the member's latest join group request.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_id: String,
    /// The client host.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_host: String,
    /// The metadata corresponding to the current group protocol in use.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub member_metadata: Vec<u8>,
    /// The current assignment provided by the group leader.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub member_assignment: Vec<u8>,
}

impl Default for DescribedGroupMember {
    fn default() -> Self {
        DescribedGroupMember {
            member_id: String::from(""),
            group_instance_id: None,
            client_id: String::from(""),
            client_host: String::from(""),
            member_metadata: Vec::new(),
            member_assignment: Vec::new(),
        }
    }
}

impl DescribedGroupMember {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>>(member_id: S1, group_instance_id: Option<S2>, client_id: S3, client_host: S4, member_metadata: Vec<u8>, member_assignment: Vec<u8>) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            client_id: client_id.as_ref().to_string(),
            client_host: client_host.as_ref().to_string(),
            member_metadata,
            member_assignment,
        }
    }
}

#[cfg(test)]
mod tests_described_group_member_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribedGroupMember::new(
            String::from(""),
            None::<String>,
            String::from(""),
            String::from(""),
            Vec::new(),
            Vec::new(),
        );
        assert_eq!(d, DescribedGroupMember::default());
    }
}

impl Readable for DescribedGroupMember {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", false)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", false)?;
        let client_id = String::read_ext(input, "client_id", false)?;
        let client_host = String::read_ext(input, "client_host", false)?;
        let member_metadata = read_bytes(input, "member_metadata", false)?;
        let member_assignment = read_bytes(input, "member_assignment", false)?;
        Ok(DescribedGroupMember {
            member_id, group_instance_id, client_id, client_host, member_metadata, member_assignment
        })
    }
}

impl Writable for DescribedGroupMember {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", false)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", false)?;
        self.client_id.write_ext(output, "self.client_id", false)?;
        self.client_host.write_ext(output, "self.client_host", false)?;
        write_bytes(output, "self.member_metadata", &self.member_metadata, false)?;
        write_bytes(output, "self.member_assignment", &self.member_assignment, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DescribeGroupsResponse>("DescribeGroupsResponse", 4);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeGroupsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeGroupsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeGroupsResponse", 4);
        }
    }
}
