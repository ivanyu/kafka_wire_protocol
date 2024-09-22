// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// GetTelemetrySubscriptionsResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct GetTelemetrySubscriptionsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// Assigned client instance id if ClientInstanceId was 0 in the request, else 0.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub client_instance_id: Uuid,
    /// Unique identifier for the current subscription set for this client instance.
    pub subscription_id: i32,
    /// Compression types that broker accepts for the PushTelemetryRequest.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub accepted_compression_types: Vec<i8>,
    /// Configured push interval, which is the lowest configured interval in the current subscription set.
    pub push_interval_ms: i32,
    /// The maximum bytes of binary data the broker accepts in PushTelemetryRequest.
    pub telemetry_max_bytes: i32,
    /// Flag to indicate monotonic/counter metrics are to be emitted as deltas or cumulative values
    pub delta_temporality: bool,
    /// Requested metrics prefix string match. Empty array: No metrics subscribed, Array[0] empty string: All metrics subscribed.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub requested_metrics: Vec<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for GetTelemetrySubscriptionsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        71
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Response for GetTelemetrySubscriptionsResponse { }

impl Default for GetTelemetrySubscriptionsResponse {
    fn default() -> Self {
        GetTelemetrySubscriptionsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            client_instance_id: Uuid::nil(),
            subscription_id: 0_i32,
            accepted_compression_types: Vec::<i8>::new(),
            push_interval_ms: 0_i32,
            telemetry_max_bytes: 0_i32,
            delta_temporality: false,
            requested_metrics: Vec::<String>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl GetTelemetrySubscriptionsResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, client_instance_id: Uuid, subscription_id: i32, accepted_compression_types: Vec<i8>, push_interval_ms: i32, telemetry_max_bytes: i32, delta_temporality: bool, requested_metrics: Vec<String>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            client_instance_id,
            subscription_id,
            accepted_compression_types,
            push_interval_ms,
            telemetry_max_bytes,
            delta_temporality,
            requested_metrics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_get_telemetry_subscriptions_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = GetTelemetrySubscriptionsResponse::new(
            0_i32,
            0_i16,
            Uuid::nil(),
            0_i32,
            Vec::<i8>::new(),
            0_i32,
            0_i32,
            false,
            Vec::<String>::new(),
        );
        assert_eq!(d, GetTelemetrySubscriptionsResponse::default());
    }
}

impl Readable for GetTelemetrySubscriptionsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let client_instance_id = Uuid::read(input)?;
        let subscription_id = i32::read(input)?;
        let accepted_compression_types = read_array::<i8>(input, "accepted_compression_types", true)?;
        let push_interval_ms = i32::read(input)?;
        let telemetry_max_bytes = i32::read(input)?;
        let delta_temporality = bool::read(input)?;
        let requested_metrics = read_array::<String>(input, "requested_metrics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(GetTelemetrySubscriptionsResponse {
            throttle_time_ms, error_code, client_instance_id, subscription_id, accepted_compression_types, push_interval_ms, telemetry_max_bytes, delta_temporality, requested_metrics, _unknown_tagged_fields
        })
    }
}

impl Writable for GetTelemetrySubscriptionsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.client_instance_id.write(output)?;
        self.subscription_id.write(output)?;
        write_array(output, "self.accepted_compression_types", &self.accepted_compression_types, true)?;
        self.push_interval_ms.write(output)?;
        self.telemetry_max_bytes.write(output)?;
        self.delta_temporality.write(output)?;
        write_array(output, "self.requested_metrics", &self.requested_metrics, true)?;
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
        crate::test_utils::test_java_default::<GetTelemetrySubscriptionsResponse>("GetTelemetrySubscriptionsResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: GetTelemetrySubscriptionsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: GetTelemetrySubscriptionsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "GetTelemetrySubscriptionsResponse", 0);
        }
    }
}
