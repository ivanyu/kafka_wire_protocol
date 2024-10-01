// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Data};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// LeaderChangeMessage, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct LeaderChangeMessage {
    /// The version of the leader change message
    pub version: i16,
    /// The ID of the newly elected leader
    pub leader_id: i32,
    /// The set of voters in the quorum for this epoch
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub voters: Vec<Voter>,
    /// The voters who voted for the leader at the time of election
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub granting_voters: Vec<Voter>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for LeaderChangeMessage {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Data for LeaderChangeMessage { }

impl Default for LeaderChangeMessage {
    fn default() -> Self {
        LeaderChangeMessage {
            version: 0_i16,
            leader_id: 0_i32,
            voters: Vec::<Voter>::new(),
            granting_voters: Vec::<Voter>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl LeaderChangeMessage {
    pub fn new(version: i16, leader_id: i32, voters: Vec<Voter>, granting_voters: Vec<Voter>) -> Self {
        Self {
            version,
            leader_id,
            voters,
            granting_voters,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_leader_change_message_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = LeaderChangeMessage::new(
            0_i16,
            0_i32,
            Vec::<Voter>::new(),
            Vec::<Voter>::new(),
        );
        assert_eq!(d, LeaderChangeMessage::default());
    }
}

impl Readable for LeaderChangeMessage {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let version = i16::read(input)?;
        let leader_id = i32::read(input)?;
        let voters = read_array::<Voter>(input, "voters", true)?;
        let granting_voters = read_array::<Voter>(input, "granting_voters", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(LeaderChangeMessage {
            version, leader_id, voters, granting_voters, _unknown_tagged_fields
        })
    }
}

impl Writable for LeaderChangeMessage {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.version.write(output)?;
        self.leader_id.write(output)?;
        write_array(output, "self.voters", &self.voters, true)?;
        write_array(output, "self.granting_voters", &self.granting_voters, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Voter, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Voter {
    /// 
    pub voter_id: i32,
    /// The directory id of the voter
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub voter_directory_id: Uuid,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Voter {
    fn default() -> Self {
        Voter {
            voter_id: 0_i32,
            voter_directory_id: Uuid::nil(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Voter {
    pub fn new(voter_id: i32, voter_directory_id: Uuid) -> Self {
        Self {
            voter_id,
            voter_directory_id,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_voter_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Voter::new(
            0_i32,
            Uuid::nil(),
        );
        assert_eq!(d, Voter::default());
    }
}

impl Readable for Voter {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let voter_id = i32::read(input)?;
        let voter_directory_id = Uuid::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Voter {
            voter_id, voter_directory_id, _unknown_tagged_fields
        })
    }
}

impl Writable for Voter {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.voter_id.write(output)?;
        self.voter_directory_id.write(output)?;
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
        crate::test_utils::test_java_default::<LeaderChangeMessage>("LeaderChangeMessage", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: LeaderChangeMessage) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: LeaderChangeMessage) {
            crate::test_utils::test_java_arbitrary(&data, "LeaderChangeMessage", 1);
        }
    }
}
