// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RemoveRaftVoterRequest {
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub cluster_id: String,
    /// The replica id of the voter getting removed from the topic partition
    pub voter_id: i32,
    /// The directory id of the voter getting removed from the topic partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub voter_directory_id: Uuid,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for RemoveRaftVoterRequest {
    fn api_key(&self) -> i16 {
        81
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for RemoveRaftVoterRequest { }

impl Default for RemoveRaftVoterRequest {
    fn default() -> Self {
        RemoveRaftVoterRequest {
            cluster_id: String::from(""),
            voter_id: 0_i32,
            voter_directory_id: Uuid::nil(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl RemoveRaftVoterRequest {
    pub fn new<S1: AsRef<str>>(cluster_id: S1, voter_id: i32, voter_directory_id: Uuid) -> Self {
        Self {
            cluster_id: cluster_id.as_ref().to_string(),
            voter_id,
            voter_directory_id,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_remove_raft_voter_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = RemoveRaftVoterRequest::new(
            String::from(""),
            0_i32,
            Uuid::nil(),
        );
        assert_eq!(d, RemoveRaftVoterRequest::default());
    }
}

impl Readable for RemoveRaftVoterRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let cluster_id = String::read_ext(input, "cluster_id", true)?;
        let voter_id = i32::read(input)?;
        let voter_directory_id = Uuid::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(RemoveRaftVoterRequest {
            cluster_id, voter_id, voter_directory_id, _unknown_tagged_fields
        })
    }
}

impl Writable for RemoveRaftVoterRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
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
        crate::test_utils::test_java_default::<RemoveRaftVoterRequest>("RemoveRaftVoterRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: RemoveRaftVoterRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: RemoveRaftVoterRequest) {
            crate::test_utils::test_java_arbitrary(&data, "RemoveRaftVoterRequest", 0);
        }
    }
}
