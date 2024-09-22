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

/// VotersRecord, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct VotersRecord {
    /// The version of the voters record
    pub version: i16,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub voters: Vec<Voter>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for VotersRecord {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        -1
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Data for VotersRecord { }

impl Default for VotersRecord {
    fn default() -> Self {
        VotersRecord {
            version: 0_i16,
            voters: Vec::<Voter>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl VotersRecord {
    pub fn new(version: i16, voters: Vec<Voter>) -> Self {
        Self {
            version,
            voters,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_voters_record_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = VotersRecord::new(
            0_i16,
            Vec::<Voter>::new(),
        );
        assert_eq!(d, VotersRecord::default());
    }
}

impl Readable for VotersRecord {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let version = i16::read(input)?;
        let voters = read_array::<Voter>(input, "voters", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(VotersRecord {
            version, voters, _unknown_tagged_fields
        })
    }
}

impl Writable for VotersRecord {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.version.write(output)?;
        write_array(output, "self.voters", &self.voters, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Voter, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Voter {
    /// The replica id of the voter in the topic partition
    pub voter_id: i32,
    /// The directory id of the voter in the topic partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub voter_directory_id: Uuid,
    /// The endpoint that can be used to communicate with the voter
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub endpoints: Vec<Endpoint>,
    /// The range of versions of the protocol that the replica supports
    pub kraft_version_feature: KRaftVersionFeature,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Voter {
    fn default() -> Self {
        Voter {
            voter_id: 0_i32,
            voter_directory_id: Uuid::nil(),
            endpoints: Vec::<Endpoint>::new(),
            kraft_version_feature: KRaftVersionFeature::default(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Voter {
    pub fn new(voter_id: i32, voter_directory_id: Uuid, endpoints: Vec<Endpoint>, kraft_version_feature: KRaftVersionFeature) -> Self {
        Self {
            voter_id,
            voter_directory_id,
            endpoints,
            kraft_version_feature,
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
            Vec::<Endpoint>::new(),
            KRaftVersionFeature::default(),
        );
        assert_eq!(d, Voter::default());
    }
}

impl Readable for Voter {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let voter_id = i32::read(input)?;
        let voter_directory_id = Uuid::read(input)?;
        let endpoints = read_array::<Endpoint>(input, "endpoints", true)?;
        let kraft_version_feature = KRaftVersionFeature::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Voter {
            voter_id, voter_directory_id, endpoints, kraft_version_feature, _unknown_tagged_fields
        })
    }
}

impl Writable for Voter {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.voter_id.write(output)?;
        self.voter_directory_id.write(output)?;
        write_array(output, "self.endpoints", &self.endpoints, true)?;
        self.kraft_version_feature.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Endpoint, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Endpoint {
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

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint {
            name: String::from(""),
            host: String::from(""),
            port: 0_u16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Endpoint {
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
mod tests_endpoint_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Endpoint::new(
            String::from(""),
            String::from(""),
            0_u16,
        );
        assert_eq!(d, Endpoint::default());
    }
}

impl Readable for Endpoint {
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
        Ok(Endpoint {
            name, host, port, _unknown_tagged_fields
        })
    }
}

impl Writable for Endpoint {
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
    /// The minimum supported KRaft protocol version
    pub min_supported_version: i16,
    /// The maximum supported KRaft protocol version
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
        crate::test_utils::test_java_default::<VotersRecord>("VotersRecord", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: VotersRecord) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: VotersRecord) {
            crate::test_utils::test_java_arbitrary(&data, "VotersRecord", 0);
        }
    }
}
