// This file was generated. Do not edit.

use std::io::{Cursor, Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// UpdateRaftVoterResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateRaftVoterResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Details of the current Raft cluster leader.
    pub current_leader: CurrentLeader,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for UpdateRaftVoterResponse {
    fn api_key(&self) -> i16 {
        82
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for UpdateRaftVoterResponse { }

impl Default for UpdateRaftVoterResponse {
    fn default() -> Self {
        UpdateRaftVoterResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            current_leader: CurrentLeader::default(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UpdateRaftVoterResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, current_leader: CurrentLeader) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            current_leader,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_update_raft_voter_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateRaftVoterResponse::new(
            0_i32,
            0_i16,
            CurrentLeader::default(),
        );
        assert_eq!(d, UpdateRaftVoterResponse::default());
    }
}

impl Readable for UpdateRaftVoterResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let mut current_leader = CurrentLeader::default();
        let tagged_fields_callback = |tag: i32, tag_data: &[u8]| {
            match tag {
                0 => {
                    let mut cur = Cursor::new(tag_data);
                    current_leader = CurrentLeader::read(&mut cur)?;
                    Ok(true)
                },
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UpdateRaftVoterResponse {
            throttle_time_ms, error_code, current_leader, _unknown_tagged_fields
        })
    }
}

impl Writable for UpdateRaftVoterResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        let mut known_tagged_fields = Vec::<RawTaggedField>::new();
        if self.current_leader != CurrentLeader::default() {
            let mut cur = Cursor::new(Vec::<u8>::new());
            self.current_leader.write(&mut cur)?;
            known_tagged_fields.push(RawTaggedField { tag: 0, data: cur.into_inner() });
        }
        write_tagged_fields(output, &known_tagged_fields, &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// CurrentLeader, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CurrentLeader {
    /// The replica id of the current leader or -1 if the leader is unknown.
    pub leader_id: i32,
    /// The latest known leader epoch.
    pub leader_epoch: i32,
    /// The node's hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The node's port.
    pub port: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for CurrentLeader {
    fn default() -> Self {
        CurrentLeader {
            leader_id: -1_i32,
            leader_epoch: -1_i32,
            host: String::from(""),
            port: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CurrentLeader {
    pub fn new<S1: AsRef<str>>(leader_id: i32, leader_epoch: i32, host: S1, port: i32) -> Self {
        Self {
            leader_id,
            leader_epoch,
            host: host.as_ref().to_string(),
            port,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_current_leader_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CurrentLeader::new(
            -1_i32,
            -1_i32,
            String::from(""),
            0_i32,
        );
        assert_eq!(d, CurrentLeader::default());
    }
}

impl Readable for CurrentLeader {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let leader_id = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let host = String::read_ext(input, "host", true)?;
        let port = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CurrentLeader {
            leader_id, leader_epoch, host, port, _unknown_tagged_fields
        })
    }
}

impl Writable for CurrentLeader {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.leader_id.write(output)?;
        self.leader_epoch.write(output)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
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
        crate::test_utils::test_java_default::<UpdateRaftVoterResponse>("UpdateRaftVoterResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UpdateRaftVoterResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UpdateRaftVoterResponse) {
            crate::test_utils::test_java_arbitrary(&data, "UpdateRaftVoterResponse", 0);
        }
    }
}
