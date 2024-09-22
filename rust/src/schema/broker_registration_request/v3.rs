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

/// BrokerRegistrationRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BrokerRegistrationRequest {
    /// The broker ID.
    pub broker_id: i32,
    /// The cluster id of the broker process.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub cluster_id: String,
    /// The incarnation id of the broker process.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub incarnation_id: Uuid,
    /// The listeners of this broker
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub listeners: Vec<Listener>,
    /// The features on this broker
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub features: Vec<Feature>,
    /// The rack which this broker is in.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack: Option<String>,
    /// If the required configurations for ZK migration are present, this value is set to true
    pub is_migrating_zk_broker: bool,
    /// Log directories configured in this broker which are available.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec_elem::<Uuid>(proptest_strategies::uuid())"))]
    pub log_dirs: Vec<Uuid>,
    /// The epoch before a clean shutdown.
    pub previous_broker_epoch: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for BrokerRegistrationRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        62
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Request for BrokerRegistrationRequest { }

impl Default for BrokerRegistrationRequest {
    fn default() -> Self {
        BrokerRegistrationRequest {
            broker_id: 0_i32,
            cluster_id: String::from(""),
            incarnation_id: Uuid::nil(),
            listeners: Vec::<Listener>::new(),
            features: Vec::<Feature>::new(),
            rack: Some(String::from("")),
            is_migrating_zk_broker: false,
            log_dirs: Vec::<Uuid>::new(),
            previous_broker_epoch: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl BrokerRegistrationRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(broker_id: i32, cluster_id: S1, incarnation_id: Uuid, listeners: Vec<Listener>, features: Vec<Feature>, rack: Option<S2>, is_migrating_zk_broker: bool, log_dirs: Vec<Uuid>, previous_broker_epoch: i64) -> Self {
        Self {
            broker_id,
            cluster_id: cluster_id.as_ref().to_string(),
            incarnation_id,
            listeners,
            features,
            rack: rack.map(|s| s.as_ref().to_string()),
            is_migrating_zk_broker,
            log_dirs,
            previous_broker_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_broker_registration_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = BrokerRegistrationRequest::new(
            0_i32,
            String::from(""),
            Uuid::nil(),
            Vec::<Listener>::new(),
            Vec::<Feature>::new(),
            Some(String::from("")),
            false,
            Vec::<Uuid>::new(),
            -1_i64,
        );
        assert_eq!(d, BrokerRegistrationRequest::default());
    }
}

impl Readable for BrokerRegistrationRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let cluster_id = String::read_ext(input, "cluster_id", true)?;
        let incarnation_id = Uuid::read(input)?;
        let listeners = read_array::<Listener>(input, "listeners", true)?;
        let features = read_array::<Feature>(input, "features", true)?;
        let rack = Option::<String>::read_ext(input, "rack", true)?;
        let is_migrating_zk_broker = bool::read(input)?;
        let log_dirs = read_array::<Uuid>(input, "log_dirs", true)?;
        let previous_broker_epoch = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(BrokerRegistrationRequest {
            broker_id, cluster_id, incarnation_id, listeners, features, rack, is_migrating_zk_broker, log_dirs, previous_broker_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for BrokerRegistrationRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
        self.incarnation_id.write(output)?;
        write_array(output, "self.listeners", &self.listeners, true)?;
        write_array(output, "self.features", &self.features, true)?;
        self.rack.write_ext(output, "self.rack", true)?;
        self.is_migrating_zk_broker.write(output)?;
        write_array(output, "self.log_dirs", &self.log_dirs, true)?;
        self.previous_broker_epoch.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Listener, version 3.
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

/// Feature, version 3.
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
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

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
        crate::test_utils::test_java_default::<BrokerRegistrationRequest>("BrokerRegistrationRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: BrokerRegistrationRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: BrokerRegistrationRequest) {
            crate::test_utils::test_java_arbitrary(&data, "BrokerRegistrationRequest", 3);
        }
    }
}
