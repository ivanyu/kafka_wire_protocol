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

/// UpdateRaftVoterRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateRaftVoterRequest {
    /// The cluster id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub cluster_id: Option<String>,
    /// The current leader epoch of the partition, -1 for unknown leader epoch.
    pub current_leader_epoch: i32,
    /// The replica id of the voter getting updated in the topic partition.
    pub voter_id: i32,
    /// The directory id of the voter getting updated in the topic partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub voter_directory_id: Uuid,
    /// The endpoint that can be used to communicate with the leader.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub listeners: Vec<Listener>,
    /// The range of versions of the protocol that the replica supports.
    pub kraft_version_feature: KRaftVersionFeature,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for UpdateRaftVoterRequest {
    fn api_key(&self) -> i16 {
        82
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for UpdateRaftVoterRequest { }

impl Default for UpdateRaftVoterRequest {
    fn default() -> Self {
        UpdateRaftVoterRequest {
            cluster_id: Some(String::from("")),
            current_leader_epoch: 0_i32,
            voter_id: 0_i32,
            voter_directory_id: Uuid::nil(),
            listeners: Vec::<Listener>::new(),
            kraft_version_feature: KRaftVersionFeature::default(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UpdateRaftVoterRequest {
    pub fn new<S1: AsRef<str>>(cluster_id: Option<S1>, current_leader_epoch: i32, voter_id: i32, voter_directory_id: Uuid, listeners: Vec<Listener>, kraft_version_feature: KRaftVersionFeature) -> Self {
        Self {
            cluster_id: cluster_id.map(|s| s.as_ref().to_string()),
            current_leader_epoch,
            voter_id,
            voter_directory_id,
            listeners,
            kraft_version_feature,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_update_raft_voter_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateRaftVoterRequest::new(
            Some(String::from("")),
            0_i32,
            0_i32,
            Uuid::nil(),
            Vec::<Listener>::new(),
            KRaftVersionFeature::default(),
        );
        assert_eq!(d, UpdateRaftVoterRequest::default());
    }
}

impl Readable for UpdateRaftVoterRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let cluster_id = Option::<String>::read_ext(input, "cluster_id", true)?;
        let current_leader_epoch = i32::read(input)?;
        let voter_id = i32::read(input)?;
        let voter_directory_id = Uuid::read(input)?;
        let listeners = read_array::<Listener>(input, "listeners", true)?;
        let kraft_version_feature = KRaftVersionFeature::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UpdateRaftVoterRequest {
            cluster_id, current_leader_epoch, voter_id, voter_directory_id, listeners, kraft_version_feature, _unknown_tagged_fields
        })
    }
}

impl Writable for UpdateRaftVoterRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
        self.current_leader_epoch.write(output)?;
        self.voter_id.write(output)?;
        self.voter_directory_id.write(output)?;
        write_array(output, "self.listeners", &self.listeners, true)?;
        self.kraft_version_feature.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Listener, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Listener {
    /// The name of the endpoint.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The port.
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

/// KRaftVersionFeature, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct KRaftVersionFeature {
    /// The minimum supported KRaft protocol version.
    pub min_supported_version: i16,
    /// The maximum supported KRaft protocol version.
    pub max_supported_version: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for KRaftVersionFeature {
    fn default() -> Self {
        KRaftVersionFeature {
            min_supported_version: 0_i16,
            max_supported_version: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl KRaftVersionFeature {
    pub fn new(min_supported_version: i16, max_supported_version: i16) -> Self {
        Self {
            min_supported_version,
            max_supported_version,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_kraft_version_feature_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = KRaftVersionFeature::new(
            0_i16,
            0_i16,
        );
        assert_eq!(d, KRaftVersionFeature::default());
    }
}

impl Readable for KRaftVersionFeature {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let min_supported_version = i16::read(input)?;
        let max_supported_version = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(KRaftVersionFeature {
            min_supported_version, max_supported_version, _unknown_tagged_fields
        })
    }
}

impl Writable for KRaftVersionFeature {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.min_supported_version.write(output)?;
        self.max_supported_version.write(output)?;
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
        crate::test_utils::test_java_default::<UpdateRaftVoterRequest>("UpdateRaftVoterRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UpdateRaftVoterRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UpdateRaftVoterRequest) {
            crate::test_utils::test_java_arbitrary(&data, "UpdateRaftVoterRequest", 0);
        }
    }
}
