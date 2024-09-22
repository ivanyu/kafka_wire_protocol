// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeClusterResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeClusterResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top-level error code, or 0 if there was no error
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The endpoint type that was described. 1=brokers, 2=controllers.
    pub endpoint_type: i8,
    /// The cluster ID that responding broker belongs to.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub cluster_id: String,
    /// The ID of the controller broker.
    pub controller_id: i32,
    /// Each broker in the response.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub brokers: Vec<DescribeClusterBroker>,
    /// 32-bit bitfield to represent authorized operations for this cluster.
    pub cluster_authorized_operations: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeClusterResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        60
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        1
    }
}

impl Response for DescribeClusterResponse { }

impl Default for DescribeClusterResponse {
    fn default() -> Self {
        DescribeClusterResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: None,
            endpoint_type: 1_i8,
            cluster_id: String::from(""),
            controller_id: -1_i32,
            brokers: Vec::<DescribeClusterBroker>::new(),
            cluster_authorized_operations: -2147483648_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClusterResponse {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, endpoint_type: i8, cluster_id: S2, controller_id: i32, brokers: Vec<DescribeClusterBroker>, cluster_authorized_operations: i32) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            endpoint_type,
            cluster_id: cluster_id.as_ref().to_string(),
            controller_id,
            brokers,
            cluster_authorized_operations,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_cluster_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeClusterResponse::new(
            0_i32,
            0_i16,
            None::<String>,
            1_i8,
            String::from(""),
            -1_i32,
            Vec::<DescribeClusterBroker>::new(),
            -2147483648_i32,
        );
        assert_eq!(d, DescribeClusterResponse::default());
    }
}

impl Readable for DescribeClusterResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let endpoint_type = i8::read(input)?;
        let cluster_id = String::read_ext(input, "cluster_id", true)?;
        let controller_id = i32::read(input)?;
        let brokers = read_array::<DescribeClusterBroker>(input, "brokers", true)?;
        let cluster_authorized_operations = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeClusterResponse {
            throttle_time_ms, error_code, error_message, endpoint_type, cluster_id, controller_id, brokers, cluster_authorized_operations, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeClusterResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        self.endpoint_type.write(output)?;
        self.cluster_id.write_ext(output, "self.cluster_id", true)?;
        self.controller_id.write(output)?;
        write_array(output, "self.brokers", &self.brokers, true)?;
        self.cluster_authorized_operations.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeClusterBroker, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeClusterBroker {
    /// The broker ID.
    pub broker_id: i32,
    /// The broker hostname.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The broker port.
    pub port: i32,
    /// The rack of the broker, or null if it has not been assigned to a rack.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeClusterBroker {
    fn default() -> Self {
        DescribeClusterBroker {
            broker_id: 0_i32,
            host: String::from(""),
            port: 0_i32,
            rack: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeClusterBroker {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(broker_id: i32, host: S1, port: i32, rack: Option<S2>) -> Self {
        Self {
            broker_id,
            host: host.as_ref().to_string(),
            port,
            rack: rack.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_cluster_broker_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeClusterBroker::new(
            0_i32,
            String::from(""),
            0_i32,
            None::<String>,
        );
        assert_eq!(d, DescribeClusterBroker::default());
    }
}

impl Readable for DescribeClusterBroker {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let host = String::read_ext(input, "host", true)?;
        let port = i32::read(input)?;
        let rack = Option::<String>::read_ext(input, "rack", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeClusterBroker {
            broker_id, host, port, rack, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeClusterBroker {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
        self.rack.write_ext(output, "self.rack", true)?;
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
        crate::test_utils::test_java_default::<DescribeClusterResponse>("DescribeClusterResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeClusterResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeClusterResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeClusterResponse", 1);
        }
    }
}
