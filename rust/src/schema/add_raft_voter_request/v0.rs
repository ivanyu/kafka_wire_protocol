// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AddRaftVoterRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AddRaftVoterRequest {
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub cluster_id: Option<String>,
    /// 
    pub timeout_ms: i32,
    /// The replica id of the voter getting added to the topic partition
    pub voter_id: i32,
    /// The directory id of the voter getting added to the topic partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub voter_directory_id: Uuid,
    /// The endpoints that can be used to communicate with the voter
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub listeners: Vec<Listener>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AddRaftVoterRequest {
    fn api_key(&self) -> i16 {
        80
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for AddRaftVoterRequest { }

impl Default for AddRaftVoterRequest {
    fn default() -> Self {
        AddRaftVoterRequest {
            cluster_id: Some(String::from("")),
            timeout_ms: 0_i32,
            voter_id: 0_i32,
            voter_directory_id: Uuid::nil(),
            listeners: Vec::<Listener>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AddRaftVoterRequest {
    pub fn new<S1: AsRef<str>>(cluster_id: Option<S1>, timeout_ms: i32, voter_id: i32, voter_directory_id: Uuid, listeners: Vec<Listener>) -> Self {
        Self {
            cluster_id: cluster_id.map(|s| s.as_ref().to_string()),
            timeout_ms,
            voter_id,
            voter_directory_id,
            listeners,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_add_raft_voter_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AddRaftVoterRequest::new(
            Some(String::from("")),
            0_i32,
            0_i32,
            Uuid::nil(),
            Vec::<Listener>::new(),
        );
        assert_eq!(d, AddRaftVoterRequest::default());
    }
}

impl Readable for AddRaftVoterRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let cluster_id = Option::<String>::read_ext(input, "cluster_id", true)?;
        let timeout_ms = i32::read(input)?;
        let voter_id = i32::read(input)?;
        let voter_directory_id = Uuid::read(input)?;
        let listeners = read_array::<Listener>(input, "listeners", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AddRaftVoterRequest {
            cluster_id, timeout_ms, voter_id, voter_directory_id, listeners, _unknown_tagged_fields
        })
    }
}

impl Writable for AddRaftVoterRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
        self.timeout_ms.write(output)?;
        self.voter_id.write(output)?;
        self.voter_directory_id.write(output)?;
        write_array(output, "self.listeners", &self.listeners, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Listener, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Listener {
    /// The name of the endpoint
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The hostname
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The port
    pub port: u16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Listener {
    fn default() -> Self {
        Listener {
            name: String::from(""),
            host: String::from(""),
            port: 0_u16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Listener {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, host: S2, port: u16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            host: host.as_ref().to_string(),
            port,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_listener_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Listener::new(
            String::from(""),
            String::from(""),
            0_u16,
        );
        assert_eq!(d, Listener::default());
    }
}

impl Readable for Listener {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let host = String::read_ext(input, "host", true)?;
        let port = u16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Listener {
            name, host, port, _unknown_tagged_fields
        })
    }
}

impl Writable for Listener {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
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
        crate::test_utils::test_java_default::<AddRaftVoterRequest>("AddRaftVoterRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AddRaftVoterRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AddRaftVoterRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AddRaftVoterRequest", 0);
        }
    }
}
