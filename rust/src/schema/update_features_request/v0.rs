// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// UpdateFeaturesRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UpdateFeaturesRequest {
    /// How long to wait in milliseconds before timing out the request.
    pub timeout_ms: i32,
    /// The list of updates to finalized features.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub feature_updates: Vec<FeatureUpdateKey>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for UpdateFeaturesRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        57
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for UpdateFeaturesRequest { }

impl Default for UpdateFeaturesRequest {
    fn default() -> Self {
        UpdateFeaturesRequest {
            timeout_ms: 60000_i32,
            feature_updates: Vec::<FeatureUpdateKey>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UpdateFeaturesRequest {
    pub fn new(timeout_ms: i32, feature_updates: Vec<FeatureUpdateKey>) -> Self {
        Self {
            timeout_ms,
            feature_updates,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_update_features_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UpdateFeaturesRequest::new(
            60000_i32,
            Vec::<FeatureUpdateKey>::new(),
        );
        assert_eq!(d, UpdateFeaturesRequest::default());
    }
}

impl Readable for UpdateFeaturesRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let timeout_ms = i32::read(input)?;
        let feature_updates = read_array::<FeatureUpdateKey>(input, "feature_updates", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UpdateFeaturesRequest {
            timeout_ms, feature_updates, _unknown_tagged_fields
        })
    }
}

impl Writable for UpdateFeaturesRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.timeout_ms.write(output)?;
        write_array(output, "self.feature_updates", &self.feature_updates, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// FeatureUpdateKey, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FeatureUpdateKey {
    /// The name of the finalized feature to be updated.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub feature: String,
    /// The new maximum version level for the finalized feature. A value >= 1 is valid. A value < 1, is special, and can be used to request the deletion of the finalized feature.
    pub max_version_level: i16,
    /// DEPRECATED in version 1 (see DowngradeType). When set to true, the finalized feature version level is allowed to be downgraded/deleted. The downgrade request will fail if the new maximum version level is a value that's not lower than the existing maximum finalized version level.
    pub allow_downgrade: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for FeatureUpdateKey {
    fn default() -> Self {
        FeatureUpdateKey {
            feature: String::from(""),
            max_version_level: 0_i16,
            allow_downgrade: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FeatureUpdateKey {
    pub fn new<S1: AsRef<str>>(feature: S1, max_version_level: i16, allow_downgrade: bool) -> Self {
        Self {
            feature: feature.as_ref().to_string(),
            max_version_level,
            allow_downgrade,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_feature_update_key_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FeatureUpdateKey::new(
            String::from(""),
            0_i16,
            false,
        );
        assert_eq!(d, FeatureUpdateKey::default());
    }
}

impl Readable for FeatureUpdateKey {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let feature = String::read_ext(input, "feature", true)?;
        let max_version_level = i16::read(input)?;
        let allow_downgrade = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FeatureUpdateKey {
            feature, max_version_level, allow_downgrade, _unknown_tagged_fields
        })
    }
}

impl Writable for FeatureUpdateKey {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.feature.write_ext(output, "self.feature", true)?;
        self.max_version_level.write(output)?;
        self.allow_downgrade.write(output)?;
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
        crate::test_utils::test_java_default::<UpdateFeaturesRequest>("UpdateFeaturesRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: UpdateFeaturesRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: UpdateFeaturesRequest) {
            crate::test_utils::test_java_arbitrary(&data, "UpdateFeaturesRequest", 0);
        }
    }
}
