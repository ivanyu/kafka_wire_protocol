// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// AlterUserScramCredentialsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterUserScramCredentialsRequest {
    /// The SCRAM credentials to remove.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub deletions: Vec<ScramCredentialDeletion>,
    /// The SCRAM credentials to update/insert.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub upsertions: Vec<ScramCredentialUpsertion>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterUserScramCredentialsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        51
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for AlterUserScramCredentialsRequest { }

impl Default for AlterUserScramCredentialsRequest {
    fn default() -> Self {
        AlterUserScramCredentialsRequest {
            deletions: Vec::<ScramCredentialDeletion>::new(),
            upsertions: Vec::<ScramCredentialUpsertion>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterUserScramCredentialsRequest {
    pub fn new(deletions: Vec<ScramCredentialDeletion>, upsertions: Vec<ScramCredentialUpsertion>) -> Self {
        Self {
            deletions,
            upsertions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_user_scram_credentials_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterUserScramCredentialsRequest::new(
            Vec::<ScramCredentialDeletion>::new(),
            Vec::<ScramCredentialUpsertion>::new(),
        );
        assert_eq!(d, AlterUserScramCredentialsRequest::default());
    }
}

impl Readable for AlterUserScramCredentialsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let deletions = read_array::<ScramCredentialDeletion>(input, "deletions", true)?;
        let upsertions = read_array::<ScramCredentialUpsertion>(input, "upsertions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterUserScramCredentialsRequest {
            deletions, upsertions, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterUserScramCredentialsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.deletions", &self.deletions, true)?;
        write_array(output, "self.upsertions", &self.upsertions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ScramCredentialDeletion, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ScramCredentialDeletion {
    /// The user name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The SCRAM mechanism.
    pub mechanism: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ScramCredentialDeletion {
    fn default() -> Self {
        ScramCredentialDeletion {
            name: String::from(""),
            mechanism: 0_i8,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ScramCredentialDeletion {
    pub fn new<S1: AsRef<str>>(name: S1, mechanism: i8) -> Self {
        Self {
            name: name.as_ref().to_string(),
            mechanism,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_scram_credential_deletion_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ScramCredentialDeletion::new(
            String::from(""),
            0_i8,
        );
        assert_eq!(d, ScramCredentialDeletion::default());
    }
}

impl Readable for ScramCredentialDeletion {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let mechanism = i8::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ScramCredentialDeletion {
            name, mechanism, _unknown_tagged_fields
        })
    }
}

impl Writable for ScramCredentialDeletion {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.mechanism.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// ScramCredentialUpsertion, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ScramCredentialUpsertion {
    /// The user name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The SCRAM mechanism.
    pub mechanism: i8,
    /// The number of iterations.
    pub iterations: i32,
    /// A random salt generated by the client.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub salt: Vec<u8>,
    /// The salted password.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub salted_password: Vec<u8>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for ScramCredentialUpsertion {
    fn default() -> Self {
        ScramCredentialUpsertion {
            name: String::from(""),
            mechanism: 0_i8,
            iterations: 0_i32,
            salt: Vec::new(),
            salted_password: Vec::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ScramCredentialUpsertion {
    pub fn new<S1: AsRef<str>>(name: S1, mechanism: i8, iterations: i32, salt: Vec<u8>, salted_password: Vec<u8>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            mechanism,
            iterations,
            salt,
            salted_password,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_scram_credential_upsertion_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ScramCredentialUpsertion::new(
            String::from(""),
            0_i8,
            0_i32,
            Vec::new(),
            Vec::new(),
        );
        assert_eq!(d, ScramCredentialUpsertion::default());
    }
}

impl Readable for ScramCredentialUpsertion {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let mechanism = i8::read(input)?;
        let iterations = i32::read(input)?;
        let salt = read_bytes(input, "salt", true)?;
        let salted_password = read_bytes(input, "salted_password", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ScramCredentialUpsertion {
            name, mechanism, iterations, salt, salted_password, _unknown_tagged_fields
        })
    }
}

impl Writable for ScramCredentialUpsertion {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.mechanism.write(output)?;
        self.iterations.write(output)?;
        write_bytes(output, "self.salt", &self.salt, true)?;
        write_bytes(output, "self.salted_password", &self.salted_password, true)?;
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
        crate::test_utils::test_java_default::<AlterUserScramCredentialsRequest>("AlterUserScramCredentialsRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterUserScramCredentialsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterUserScramCredentialsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AlterUserScramCredentialsRequest", 0);
        }
    }
}
