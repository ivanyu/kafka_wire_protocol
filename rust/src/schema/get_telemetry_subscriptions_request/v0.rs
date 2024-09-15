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
pub struct GetTelemetrySubscriptionsRequest {
    /// Unique id for this client instance, must be set to 0 on the first request.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub client_instance_id: Uuid,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for GetTelemetrySubscriptionsRequest {
    fn api_key(&self) -> i16 {
        71
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for GetTelemetrySubscriptionsRequest { }

impl Default for GetTelemetrySubscriptionsRequest {
    fn default() -> Self {
        GetTelemetrySubscriptionsRequest {
            client_instance_id: Uuid::nil(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl GetTelemetrySubscriptionsRequest {
    pub fn new(client_instance_id: Uuid) -> Self {
        Self {
            client_instance_id,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_get_telemetry_subscriptions_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = GetTelemetrySubscriptionsRequest::new(
            Uuid::nil(),
        );
        assert_eq!(d, GetTelemetrySubscriptionsRequest::default());
    }
}

impl Readable for GetTelemetrySubscriptionsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let client_instance_id = Uuid::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(GetTelemetrySubscriptionsRequest {
            client_instance_id, _unknown_tagged_fields
        })
    }
}

impl Writable for GetTelemetrySubscriptionsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.client_instance_id.write(output)?;
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
        crate::test_utils::test_java_default::<GetTelemetrySubscriptionsRequest>("GetTelemetrySubscriptionsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: GetTelemetrySubscriptionsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: GetTelemetrySubscriptionsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "GetTelemetrySubscriptionsRequest", 0);
        }
    }
}
