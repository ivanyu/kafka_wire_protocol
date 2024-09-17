// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_nullable_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeDelegationTokenRequest, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeDelegationTokenRequest {
    /// Each owner that we want to describe delegation tokens for, or null to describe all tokens.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub owners: Option<Vec<DescribeDelegationTokenOwner>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeDelegationTokenRequest {
    fn api_key(&self) -> i16 {
        41
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for DescribeDelegationTokenRequest { }

impl Default for DescribeDelegationTokenRequest {
    fn default() -> Self {
        DescribeDelegationTokenRequest {
            owners: Some(Vec::<DescribeDelegationTokenOwner>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeDelegationTokenRequest {
    pub fn new(owners: Option<Vec<DescribeDelegationTokenOwner>>) -> Self {
        Self {
            owners,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_delegation_token_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeDelegationTokenRequest::new(
            Some(Vec::<DescribeDelegationTokenOwner>::new()),
        );
        assert_eq!(d, DescribeDelegationTokenRequest::default());
    }
}

impl Readable for DescribeDelegationTokenRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let owners = read_nullable_array::<DescribeDelegationTokenOwner>(input, "owners", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeDelegationTokenRequest {
            owners, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeDelegationTokenRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_nullable_array(output, "self.owners", self.owners.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribeDelegationTokenOwner, version 3.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeDelegationTokenOwner {
    /// The owner principal type.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_type: String,
    /// The owner principal name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_name: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribeDelegationTokenOwner {
    fn default() -> Self {
        DescribeDelegationTokenOwner {
            principal_type: String::from(""),
            principal_name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeDelegationTokenOwner {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(principal_type: S1, principal_name: S2) -> Self {
        Self {
            principal_type: principal_type.as_ref().to_string(),
            principal_name: principal_name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_delegation_token_owner_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeDelegationTokenOwner::new(
            String::from(""),
            String::from(""),
        );
        assert_eq!(d, DescribeDelegationTokenOwner::default());
    }
}

impl Readable for DescribeDelegationTokenOwner {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let principal_type = String::read_ext(input, "principal_type", true)?;
        let principal_name = String::read_ext(input, "principal_name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeDelegationTokenOwner {
            principal_type, principal_name, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeDelegationTokenOwner {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.principal_type.write_ext(output, "self.principal_type", true)?;
        self.principal_name.write_ext(output, "self.principal_name", true)?;
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
        crate::test_utils::test_java_default::<DescribeDelegationTokenRequest>("DescribeDelegationTokenRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeDelegationTokenRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeDelegationTokenRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeDelegationTokenRequest", 3);
        }
    }
}
