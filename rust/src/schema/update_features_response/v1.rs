// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// UpdateFeaturesResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateFeaturesResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top-level error code, or `0` if there was no top-level error.
    pub error_code: i16,
    /// The top-level error message, or `null` if there was no top-level error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Results for each feature update.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub results: Vec<UpdatableFeatureResult>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for UpdateFeaturesResponse {
    fn api_key(&self) -> i16 {
        57
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for UpdateFeaturesResponse { }

impl Default for UpdateFeaturesResponse {
    fn default() -> Self {
        UpdateFeaturesResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            results: Vec::<UpdatableFeatureResult>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UpdateFeaturesResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, results: Vec<UpdatableFeatureResult>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            results,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_update_features_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateFeaturesResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
            Vec::<UpdatableFeatureResult>::new(),
        );
        assert_eq!(d, UpdateFeaturesResponse::default());
    }
}

impl Readable for UpdateFeaturesResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let results = read_array::<UpdatableFeatureResult>(input, "results", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UpdateFeaturesResponse {
            throttle_time_ms, error_code, error_message, results, _unknown_tagged_fields
        })
    }
}

impl Writable for UpdateFeaturesResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        write_array(output, "self.results", &self.results, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// UpdatableFeatureResult, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdatableFeatureResult {
    /// The name of the finalized feature.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub feature: String,
    /// The feature update error code or `0` if the feature update succeeded.
    pub error_code: i16,
    /// The feature update error, or `null` if the feature update succeeded.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for UpdatableFeatureResult {
    fn default() -> Self {
        UpdatableFeatureResult {
            feature: String::from(""),
            error_code: 0_i16,
            error_message: Some(String::from("")),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UpdatableFeatureResult {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(feature: S1, error_code: i16, error_message: Option<S2>) -> Self {
        Self {
            feature: feature.as_ref().to_string(),
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_updatable_feature_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdatableFeatureResult::new(
            String::from(""),
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, UpdatableFeatureResult::default());
    }
}

impl Readable for UpdatableFeatureResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let feature = String::read_ext(input, "feature", true)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UpdatableFeatureResult {
            feature, error_code, error_message, _unknown_tagged_fields
        })
    }
}

impl Writable for UpdatableFeatureResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.feature.write_ext(output, "self.feature", true)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
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
        crate::test_utils::test_java_default::<UpdateFeaturesResponse>("UpdateFeaturesResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UpdateFeaturesResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UpdateFeaturesResponse) {
            crate::test_utils::test_java_arbitrary(&data, "UpdateFeaturesResponse", 1);
        }
    }
}
