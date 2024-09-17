// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// SyncGroupRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SyncGroupRequest {
    /// The unique group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The generation of the group.
    pub generation_id: i32,
    /// The member ID assigned by the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The unique identifier of the consumer instance provided by end user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// Each assignment.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub assignments: Vec<SyncGroupRequestAssignment>,
}

impl ApiMessage for SyncGroupRequest {
    fn api_key(&self) -> i16 {
        14
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for SyncGroupRequest { }

impl Default for SyncGroupRequest {
    fn default() -> Self {
        SyncGroupRequest {
            group_id: String::from(""),
            generation_id: 0_i32,
            member_id: String::from(""),
            group_instance_id: None,
            assignments: Vec::<SyncGroupRequestAssignment>::new(),
        }
    }
}

impl SyncGroupRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(group_id: S1, generation_id: i32, member_id: S2, group_instance_id: Option<S3>, assignments: Vec<SyncGroupRequestAssignment>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            generation_id,
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            assignments,
        }
    }
}

#[cfg(test)]
mod tests_sync_group_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SyncGroupRequest::new(
            String::from(""),
            0_i32,
            String::from(""),
            None::<String>,
            Vec::<SyncGroupRequestAssignment>::new(),
        );
        assert_eq!(d, SyncGroupRequest::default());
    }
}

impl Readable for SyncGroupRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", false)?;
        let generation_id = i32::read(input)?;
        let member_id = String::read_ext(input, "member_id", false)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", false)?;
        let assignments = read_array::<SyncGroupRequestAssignment>(input, "assignments", false)?;
        Ok(SyncGroupRequest {
            group_id, generation_id, member_id, group_instance_id, assignments
        })
    }
}

impl Writable for SyncGroupRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", false)?;
        self.generation_id.write(output)?;
        self.member_id.write_ext(output, "self.member_id", false)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", false)?;
        write_array(output, "self.assignments", &self.assignments, false)?;
        Ok(())
    }
}

/// SyncGroupRequestAssignment, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SyncGroupRequestAssignment {
    /// The ID of the member to assign.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The member assignment.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub assignment: Vec<u8>,
}

impl Default for SyncGroupRequestAssignment {
    fn default() -> Self {
        SyncGroupRequestAssignment {
            member_id: String::from(""),
            assignment: Vec::new(),
        }
    }
}

impl SyncGroupRequestAssignment {
    pub fn new<S1: AsRef<str>>(member_id: S1, assignment: Vec<u8>) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            assignment,
        }
    }
}

#[cfg(test)]
mod tests_sync_group_request_assignment_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SyncGroupRequestAssignment::new(
            String::from(""),
            Vec::new(),
        );
        assert_eq!(d, SyncGroupRequestAssignment::default());
    }
}

impl Readable for SyncGroupRequestAssignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", false)?;
        let assignment = read_bytes(input, "assignment", false)?;
        Ok(SyncGroupRequestAssignment {
            member_id, assignment
        })
    }
}

impl Writable for SyncGroupRequestAssignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", false)?;
        write_bytes(output, "self.assignment", &self.assignment, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<SyncGroupRequest>("SyncGroupRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: SyncGroupRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: SyncGroupRequest) {
            crate::test_utils::test_java_arbitrary(&data, "SyncGroupRequest", 3);
        }
    }
}
