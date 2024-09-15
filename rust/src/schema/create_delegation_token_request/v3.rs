// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateDelegationTokenRequest {
    /// The principal type of the owner of the token. If it's null it defaults to the token request principal.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub owner_principal_type: Option<String>,
    /// The principal name of the owner of the token. If it's null it defaults to the token request principal.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub owner_principal_name: Option<String>,
    /// A list of those who are allowed to renew this token before it expires.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub renewers: Vec<CreatableRenewers>,
    /// The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
    pub max_lifetime_ms: i64,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreateDelegationTokenRequest {
    fn api_key(&self) -> i16 {
        38
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for CreateDelegationTokenRequest { }

impl Default for CreateDelegationTokenRequest {
    fn default() -> Self {
        CreateDelegationTokenRequest {
            owner_principal_type: Some(String::from("")),
            owner_principal_name: Some(String::from("")),
            renewers: Vec::<CreatableRenewers>::new(),
            max_lifetime_ms: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreateDelegationTokenRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(owner_principal_type: Option<S1>, owner_principal_name: Option<S2>, renewers: Vec<CreatableRenewers>, max_lifetime_ms: i64) -> Self {
        Self {
            owner_principal_type: owner_principal_type.map(|s| s.as_ref().to_string()),
            owner_principal_name: owner_principal_name.map(|s| s.as_ref().to_string()),
            renewers,
            max_lifetime_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_create_delegation_token_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateDelegationTokenRequest::new(
            Some(String::from("")),
            Some(String::from("")),
            Vec::<CreatableRenewers>::new(),
            0_i64,
        );
        assert_eq!(d, CreateDelegationTokenRequest::default());
    }
}

impl Readable for CreateDelegationTokenRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let owner_principal_type = Option::<String>::read_ext(input, "owner_principal_type", true)?;
        let owner_principal_name = Option::<String>::read_ext(input, "owner_principal_name", true)?;
        let renewers = read_array::<CreatableRenewers>(input, "renewers", true)?;
        let max_lifetime_ms = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreateDelegationTokenRequest {
            owner_principal_type, owner_principal_name, renewers, max_lifetime_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for CreateDelegationTokenRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.owner_principal_type.write_ext(output, "self.owner_principal_type", true)?;
        self.owner_principal_name.write_ext(output, "self.owner_principal_name", true)?;
        write_array(output, "self.renewers", &self.renewers, true)?;
        self.max_lifetime_ms.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreatableRenewers {
    /// The type of the Kafka principal.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_type: String,
    /// The name of the Kafka principal.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal_name: String,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreatableRenewers {
    fn api_key(&self) -> i16 {
        38
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for CreatableRenewers { }

impl Default for CreatableRenewers {
    fn default() -> Self {
        CreatableRenewers {
            principal_type: String::from(""),
            principal_name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreatableRenewers {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(principal_type: S1, principal_name: S2) -> Self {
        Self {
            principal_type: principal_type.as_ref().to_string(),
            principal_name: principal_name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_creatable_renewers_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreatableRenewers::new(
            String::from(""),
            String::from(""),
        );
        assert_eq!(d, CreatableRenewers::default());
    }
}

impl Readable for CreatableRenewers {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let principal_type = String::read_ext(input, "principal_type", true)?;
        let principal_name = String::read_ext(input, "principal_name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreatableRenewers {
            principal_type, principal_name, _unknown_tagged_fields
        })
    }
}

impl Writable for CreatableRenewers {
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
        crate::test_utils::test_java_default::<CreateDelegationTokenRequest>("CreateDelegationTokenRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreateDelegationTokenRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreateDelegationTokenRequest) {
            crate::test_utils::test_java_arbitrary(&data, "CreateDelegationTokenRequest", 3);
        }
    }
}
