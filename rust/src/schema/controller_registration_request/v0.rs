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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ControllerRegistrationRequest {
    /// The ID of the controller to register.
    pub controller_id: i32,
    /// The controller incarnation ID, which is unique to each process run.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub incarnation_id: Uuid,
    /// Set if the required configurations for ZK migration are present.
    pub zk_migration_ready: bool,
    /// The listeners of this controller
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub listeners: Vec<Listener>,
    /// The features on this controller
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub features: Vec<Feature>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ControllerRegistrationRequest {
    fn api_key(&self) -> i16 {
        70
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ControllerRegistrationRequest { }

impl Default for ControllerRegistrationRequest {
    fn default() -> Self {
        ControllerRegistrationRequest {
            controller_id: 0_i32,
            incarnation_id: Uuid::nil(),
            zk_migration_ready: false,
            listeners: Vec::<Listener>::new(),
            features: Vec::<Feature>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ControllerRegistrationRequest {
    pub fn new(controller_id: i32, incarnation_id: Uuid, zk_migration_ready: bool, listeners: Vec<Listener>, features: Vec<Feature>) -> Self {
        Self {
            controller_id,
            incarnation_id,
            zk_migration_ready,
            listeners,
            features,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_controller_registration_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ControllerRegistrationRequest::new(
            0_i32,
            Uuid::nil(),
            false,
            Vec::<Listener>::new(),
            Vec::<Feature>::new(),
        );
        assert_eq!(d, ControllerRegistrationRequest::default());
    }
}

impl Readable for ControllerRegistrationRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let controller_id = i32::read(input)?;
        let incarnation_id = Uuid::read(input)?;
        let zk_migration_ready = bool::read(input)?;
        let listeners = read_array::<Listener>(input, "listeners", true)?;
        let features = read_array::<Feature>(input, "features", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ControllerRegistrationRequest {
            controller_id, incarnation_id, zk_migration_ready, listeners, features, _unknown_tagged_fields
        })
    }
}

impl Writable for ControllerRegistrationRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.controller_id.write(output)?;
        self.incarnation_id.write(output)?;
        self.zk_migration_ready.write(output)?;
        write_array(output, "self.listeners", &self.listeners, true)?;
        write_array(output, "self.features", &self.features, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

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
    /// The security protocol.
    pub security_protocol: i16,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for Listener {
    fn api_key(&self) -> i16 {
        70
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for Listener { }

impl Default for Listener {
    fn default() -> Self {
        Listener {
            name: String::from(""),
            host: String::from(""),
            port: 0_u16,
            security_protocol: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Listener {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(name: S1, host: S2, port: u16, security_protocol: i16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            host: host.as_ref().to_string(),
            port,
            security_protocol,
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
            0_i16,
        );
        assert_eq!(d, Listener::default());
    }
}

impl Readable for Listener {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let host = String::read_ext(input, "host", true)?;
        let port = u16::read(input)?;
        let security_protocol = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Listener {
            name, host, port, security_protocol, _unknown_tagged_fields
        })
    }
}

impl Writable for Listener {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
        self.security_protocol.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Feature {
    /// The feature name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The minimum supported feature level.
    pub min_supported_version: i16,
    /// The maximum supported feature level.
    pub max_supported_version: i16,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for Feature {
    fn api_key(&self) -> i16 {
        70
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for Feature { }

impl Default for Feature {
    fn default() -> Self {
        Feature {
            name: String::from(""),
            min_supported_version: 0_i16,
            max_supported_version: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Feature {
    pub fn new<S1: AsRef<str>>(name: S1, min_supported_version: i16, max_supported_version: i16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            min_supported_version,
            max_supported_version,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_feature_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Feature::new(
            String::from(""),
            0_i16,
            0_i16,
        );
        assert_eq!(d, Feature::default());
    }
}

impl Readable for Feature {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let min_supported_version = i16::read(input)?;
        let max_supported_version = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Feature {
            name, min_supported_version, max_supported_version, _unknown_tagged_fields
        })
    }
}

impl Writable for Feature {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
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
        crate::test_utils::test_java_default::<ControllerRegistrationRequest>("ControllerRegistrationRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ControllerRegistrationRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ControllerRegistrationRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ControllerRegistrationRequest", 0);
        }
    }
}
