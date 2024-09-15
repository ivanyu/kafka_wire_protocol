// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateDelegationTokenRequest {
    /// A list of those who are allowed to renew this token before it expires.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub renewers: Vec<CreatableRenewers>,
    /// The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
    pub max_lifetime_ms: i64,
}

impl ApiMessage for CreateDelegationTokenRequest {
    fn api_key(&self) -> i16 {
        38
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for CreateDelegationTokenRequest { }

impl Default for CreateDelegationTokenRequest {
    fn default() -> Self {
        CreateDelegationTokenRequest {
            renewers: Vec::<CreatableRenewers>::new(),
            max_lifetime_ms: 0_i64,
        }
    }
}

impl CreateDelegationTokenRequest {
    pub fn new(renewers: Vec<CreatableRenewers>, max_lifetime_ms: i64) -> Self {
        Self {
            renewers,
            max_lifetime_ms,
        }
    }
}

#[cfg(test)]
mod tests_create_delegation_token_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateDelegationTokenRequest::new(
            Vec::<CreatableRenewers>::new(),
            0_i64,
        );
        assert_eq!(d, CreateDelegationTokenRequest::default());
    }
}

impl Readable for CreateDelegationTokenRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let renewers = read_array::<CreatableRenewers>(input, "renewers", false)?;
        let max_lifetime_ms = i64::read(input)?;
        Ok(CreateDelegationTokenRequest {
            renewers, max_lifetime_ms
        })
    }
}

impl Writable for CreateDelegationTokenRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.renewers", &self.renewers, false)?;
        self.max_lifetime_ms.write(output)?;
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
}

impl ApiMessage for CreatableRenewers {
    fn api_key(&self) -> i16 {
        38
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for CreatableRenewers { }

impl Default for CreatableRenewers {
    fn default() -> Self {
        CreatableRenewers {
            principal_type: String::from(""),
            principal_name: String::from(""),
        }
    }
}

impl CreatableRenewers {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(principal_type: S1, principal_name: S2) -> Self {
        Self {
            principal_type: principal_type.as_ref().to_string(),
            principal_name: principal_name.as_ref().to_string(),
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
        let principal_type = String::read_ext(input, "principal_type", false)?;
        let principal_name = String::read_ext(input, "principal_name", false)?;
        Ok(CreatableRenewers {
            principal_type, principal_name
        })
    }
}

impl Writable for CreatableRenewers {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.principal_type.write_ext(output, "self.principal_type", false)?;
        self.principal_name.write_ext(output, "self.principal_name", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<CreateDelegationTokenRequest>("CreateDelegationTokenRequest", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "CreateDelegationTokenRequest", 1);
        }
    }
}
