// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_nullable_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DescribeUserScramCredentialsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeUserScramCredentialsRequest {
    /// The users to describe, or null/empty to describe all users.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub users: Option<Vec<UserName>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DescribeUserScramCredentialsRequest {
    fn api_key(&self) -> i16 {
        50
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for DescribeUserScramCredentialsRequest { }

impl Default for DescribeUserScramCredentialsRequest {
    fn default() -> Self {
        DescribeUserScramCredentialsRequest {
            users: Some(Vec::<UserName>::new()),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribeUserScramCredentialsRequest {
    pub fn new(users: Option<Vec<UserName>>) -> Self {
        Self {
            users,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_describe_user_scram_credentials_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeUserScramCredentialsRequest::new(
            Some(Vec::<UserName>::new()),
        );
        assert_eq!(d, DescribeUserScramCredentialsRequest::default());
    }
}

impl Readable for DescribeUserScramCredentialsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let users = read_nullable_array::<UserName>(input, "users", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribeUserScramCredentialsRequest {
            users, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribeUserScramCredentialsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_nullable_array(output, "self.users", self.users.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// UserName, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UserName {
    /// The user name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for UserName {
    fn default() -> Self {
        UserName {
            name: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl UserName {
    pub fn new<S1: AsRef<str>>(name: S1) -> Self {
        Self {
            name: name.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_user_name_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = UserName::new(
            String::from(""),
        );
        assert_eq!(d, UserName::default());
    }
}

impl Readable for UserName {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(UserName {
            name, _unknown_tagged_fields
        })
    }
}

impl Writable for UserName {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
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
        crate::test_utils::test_java_default::<DescribeUserScramCredentialsRequest>("DescribeUserScramCredentialsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeUserScramCredentialsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeUserScramCredentialsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeUserScramCredentialsRequest", 0);
        }
    }
}
