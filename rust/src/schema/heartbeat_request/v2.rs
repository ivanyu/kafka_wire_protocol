// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct HeartbeatRequest {
    /// The group id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The generation of the group.
    pub generation_id: i32,
    /// The member ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
}

impl ApiMessage for HeartbeatRequest {
    fn api_key(&self) -> i16 {
        12
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for HeartbeatRequest { }

impl Default for HeartbeatRequest {
    fn default() -> Self {
        HeartbeatRequest {
            group_id: String::from(""),
            generation_id: 0_i32,
            member_id: String::from(""),
        }
    }
}

impl HeartbeatRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(group_id: S1, generation_id: i32, member_id: S2) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            generation_id,
            member_id: member_id.as_ref().to_string(),
        }
    }
}

#[cfg(test)]
mod tests_heartbeat_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = HeartbeatRequest::new(
            String::from(""),
            0_i32,
            String::from(""),
        );
        assert_eq!(d, HeartbeatRequest::default());
    }
}

impl Readable for HeartbeatRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", false)?;
        let generation_id = i32::read(input)?;
        let member_id = String::read_ext(input, "member_id", false)?;
        Ok(HeartbeatRequest {
            group_id, generation_id, member_id
        })
    }
}

impl Writable for HeartbeatRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", false)?;
        self.generation_id.write(output)?;
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
        crate::test_utils::test_java_default::<HeartbeatRequest>("HeartbeatRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: HeartbeatRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: HeartbeatRequest) {
            crate::test_utils::test_java_arbitrary(&data, "HeartbeatRequest", 2);
        }
    }
}
