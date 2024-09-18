// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// PushTelemetryRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PushTelemetryRequest {
    /// Unique id for this client instance.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub client_instance_id: Uuid,
    /// Unique identifier for the current subscription.
    pub subscription_id: i32,
    /// Client is terminating the connection.
    pub terminating: bool,
    /// Compression codec used to compress the metrics.
    pub compression_type: i8,
    /// Metrics encoded in OpenTelemetry MetricsData v1 protobuf format.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub metrics: Vec<u8>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for PushTelemetryRequest {
    fn api_key(&self) -> i16 {
        72
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for PushTelemetryRequest { }

impl Default for PushTelemetryRequest {
    fn default() -> Self {
        PushTelemetryRequest {
            client_instance_id: Uuid::nil(),
            subscription_id: 0_i32,
            terminating: false,
            compression_type: 0_i8,
            metrics: Vec::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PushTelemetryRequest {
    pub fn new(client_instance_id: Uuid, subscription_id: i32, terminating: bool, compression_type: i8, metrics: Vec<u8>) -> Self {
        Self {
            client_instance_id,
            subscription_id,
            terminating,
            compression_type,
            metrics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_push_telemetry_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PushTelemetryRequest::new(
            Uuid::nil(),
            0_i32,
            false,
            0_i8,
            Vec::new(),
        );
        assert_eq!(d, PushTelemetryRequest::default());
    }
}

impl Readable for PushTelemetryRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let client_instance_id = Uuid::read(input)?;
        let subscription_id = i32::read(input)?;
        let terminating = bool::read(input)?;
        let compression_type = i8::read(input)?;
        let metrics = read_bytes(input, "metrics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PushTelemetryRequest {
            client_instance_id, subscription_id, terminating, compression_type, metrics, _unknown_tagged_fields
        })
    }
}

impl Writable for PushTelemetryRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.client_instance_id.write(output)?;
        self.subscription_id.write(output)?;
        self.terminating.write(output)?;
        self.compression_type.write(output)?;
        write_bytes(output, "self.metrics", &self.metrics, true)?;
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
        crate::test_utils::test_java_default::<PushTelemetryRequest>("PushTelemetryRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: PushTelemetryRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: PushTelemetryRequest) {
            crate::test_utils::test_java_arbitrary(&data, "PushTelemetryRequest", 0);
        }
    }
}
