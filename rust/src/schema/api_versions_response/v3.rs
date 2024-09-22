// This file was generated. Do not edit.

use std::io::{Cursor, Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ApiVersionsResponse, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ApiVersionsResponse {
    /// The top-level error code.
    pub error_code: i16,
    /// The APIs supported by the broker.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub api_keys: Vec<ApiVersion>,
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Features supported by the broker.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub supported_features: Vec<SupportedFeatureKey>,
    /// The monotonically increasing epoch for the finalized features information. Valid values are >= 0. A value of -1 is special and represents unknown epoch.
    pub finalized_features_epoch: i64,
    /// List of cluster-wide finalized features. The information is valid only if FinalizedFeaturesEpoch >= 0.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub finalized_features: Vec<FinalizedFeatureKey>,
    /// Set by a KRaft controller if the required configurations for ZK migration are present
    pub zk_migration_ready: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ApiVersionsResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        18
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        3
    }
}

impl Response for ApiVersionsResponse { }

impl Default for ApiVersionsResponse {
    fn default() -> Self {
        ApiVersionsResponse {
            error_code: 0_i16,
            api_keys: Vec::<ApiVersion>::new(),
            throttle_time_ms: 0_i32,
            supported_features: Vec::<SupportedFeatureKey>::new(),
            finalized_features_epoch: -1_i64,
            finalized_features: Vec::<FinalizedFeatureKey>::new(),
            zk_migration_ready: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ApiVersionsResponse {
    pub fn new(error_code: i16, api_keys: Vec<ApiVersion>, throttle_time_ms: i32, supported_features: Vec<SupportedFeatureKey>, finalized_features_epoch: i64, finalized_features: Vec<FinalizedFeatureKey>, zk_migration_ready: bool) -> Self {
        Self {
            error_code,
            api_keys,
            throttle_time_ms,
            supported_features,
            finalized_features_epoch,
            finalized_features,
            zk_migration_ready,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_api_versions_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ApiVersionsResponse::new(
            0_i16,
            Vec::<ApiVersion>::new(),
            0_i32,
            Vec::<SupportedFeatureKey>::new(),
            -1_i64,
            Vec::<FinalizedFeatureKey>::new(),
            false,
        );
        assert_eq!(d, ApiVersionsResponse::default());
    }
}

impl Readable for ApiVersionsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let api_keys = read_array::<ApiVersion>(input, "api_keys", true)?;
        let throttle_time_ms = i32::read(input)?;
        let mut supported_features = Vec::<SupportedFeatureKey>::new();
        let mut finalized_features_epoch = -1_i64;
        let mut finalized_features = Vec::<FinalizedFeatureKey>::new();
        let mut zk_migration_ready = false;
        let tagged_fields_callback = |tag: i32, tag_data: &[u8]| {
            match tag {
                0 => {
                    let mut cur = Cursor::new(tag_data);
                    supported_features = read_array::<SupportedFeatureKey>(&mut cur, "supported_features", true)?;
                    Ok(true)
                },
                1 => {
                    let mut cur = Cursor::new(tag_data);
                    finalized_features_epoch = i64::read(&mut cur)?;
                    Ok(true)
                },
                2 => {
                    let mut cur = Cursor::new(tag_data);
                    finalized_features = read_array::<FinalizedFeatureKey>(&mut cur, "finalized_features", true)?;
                    Ok(true)
                },
                3 => {
                    let mut cur = Cursor::new(tag_data);
                    zk_migration_ready = bool::read(&mut cur)?;
                    Ok(true)
                },
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ApiVersionsResponse {
            error_code, api_keys, throttle_time_ms, supported_features, finalized_features_epoch, finalized_features, zk_migration_ready, _unknown_tagged_fields
        })
    }
}

impl Writable for ApiVersionsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.api_keys", &self.api_keys, true)?;
        self.throttle_time_ms.write(output)?;
        let mut known_tagged_fields = Vec::<RawTaggedField>::new();
        if !self.supported_features.is_empty() {
            let mut cur = Cursor::new(Vec::<u8>::new());
            write_array(&mut cur, "self.supported_features", &self.supported_features, true)?;
            known_tagged_fields.push(RawTaggedField { tag: 0, data: cur.into_inner() });
        }
        if self.finalized_features_epoch != -1_i64 {
            let mut cur = Cursor::new(Vec::<u8>::new());
            self.finalized_features_epoch.write(&mut cur)?;
            known_tagged_fields.push(RawTaggedField { tag: 1, data: cur.into_inner() });
        }
        if !self.finalized_features.is_empty() {
            let mut cur = Cursor::new(Vec::<u8>::new());
            write_array(&mut cur, "self.finalized_features", &self.finalized_features, true)?;
            known_tagged_fields.push(RawTaggedField { tag: 2, data: cur.into_inner() });
        }
        if self.zk_migration_ready {
            let mut cur = Cursor::new(Vec::<u8>::new());
            self.zk_migration_ready.write(&mut cur)?;
            known_tagged_fields.push(RawTaggedField { tag: 3, data: cur.into_inner() });
        }
        write_tagged_fields(output, &known_tagged_fields, &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ApiVersion, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ApiVersion {
    /// The API index.
    pub api_key: i16,
    /// The minimum supported version, inclusive.
    pub min_version: i16,
    /// The maximum supported version, inclusive.
    pub max_version: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion {
            api_key: 0_i16,
            min_version: 0_i16,
            max_version: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ApiVersion {
    pub fn new(api_key: i16, min_version: i16, max_version: i16) -> Self {
        Self {
            api_key,
            min_version,
            max_version,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_api_version_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ApiVersion::new(
            0_i16,
            0_i16,
            0_i16,
        );
        assert_eq!(d, ApiVersion::default());
    }
}

impl Readable for ApiVersion {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let api_key = i16::read(input)?;
        let min_version = i16::read(input)?;
        let max_version = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ApiVersion {
            api_key, min_version, max_version, _unknown_tagged_fields
        })
    }
}

impl Writable for ApiVersion {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.api_key.write(output)?;
        self.min_version.write(output)?;
        self.max_version.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// SupportedFeatureKey, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SupportedFeatureKey {
    /// The name of the feature.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The minimum supported version for the feature.
    pub min_version: i16,
    /// The maximum supported version for the feature.
    pub max_version: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for SupportedFeatureKey {
    fn default() -> Self {
        SupportedFeatureKey {
            name: String::from(""),
            min_version: 0_i16,
            max_version: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl SupportedFeatureKey {
    pub fn new<S1: AsRef<str>>(name: S1, min_version: i16, max_version: i16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            min_version,
            max_version,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_supported_feature_key_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SupportedFeatureKey::new(
            String::from(""),
            0_i16,
            0_i16,
        );
        assert_eq!(d, SupportedFeatureKey::default());
    }
}

impl Readable for SupportedFeatureKey {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let min_version = i16::read(input)?;
        let max_version = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(SupportedFeatureKey {
            name, min_version, max_version, _unknown_tagged_fields
        })
    }
}

impl Writable for SupportedFeatureKey {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.min_version.write(output)?;
        self.max_version.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// FinalizedFeatureKey, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FinalizedFeatureKey {
    /// The name of the feature.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The cluster-wide finalized max version level for the feature.
    pub max_version_level: i16,
    /// The cluster-wide finalized min version level for the feature.
    pub min_version_level: i16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for FinalizedFeatureKey {
    fn default() -> Self {
        FinalizedFeatureKey {
            name: String::from(""),
            max_version_level: 0_i16,
            min_version_level: 0_i16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FinalizedFeatureKey {
    pub fn new<S1: AsRef<str>>(name: S1, max_version_level: i16, min_version_level: i16) -> Self {
        Self {
            name: name.as_ref().to_string(),
            max_version_level,
            min_version_level,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_finalized_feature_key_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FinalizedFeatureKey::new(
            String::from(""),
            0_i16,
            0_i16,
        );
        assert_eq!(d, FinalizedFeatureKey::default());
    }
}

impl Readable for FinalizedFeatureKey {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let max_version_level = i16::read(input)?;
        let min_version_level = i16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FinalizedFeatureKey {
            name, max_version_level, min_version_level, _unknown_tagged_fields
        })
    }
}

impl Writable for FinalizedFeatureKey {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.max_version_level.write(output)?;
        self.min_version_level.write(output)?;
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
        crate::test_utils::test_java_default::<ApiVersionsResponse>("ApiVersionsResponse", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ApiVersionsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ApiVersionsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ApiVersionsResponse", 3);
        }
    }
}
