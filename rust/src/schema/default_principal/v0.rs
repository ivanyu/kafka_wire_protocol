// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Data};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DefaultPrincipal, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DefaultPrincipal {
    /// The principal type
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub type_: String,
    /// The principal name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// Whether the principal was authenticated by a delegation token on the forwarding broker.
    pub token_authenticated: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for DefaultPrincipal {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Data for DefaultPrincipal { }

impl Default for DefaultPrincipal {
    fn default() -> Self {
        DefaultPrincipal {
            type_: String::from(""),
            name: String::from(""),
            token_authenticated: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DefaultPrincipal {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(type_: S1, name: S2, token_authenticated: bool) -> Self {
        Self {
            type_: type_.as_ref().to_string(),
            name: name.as_ref().to_string(),
            token_authenticated,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_default_principal_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DefaultPrincipal::new(
            String::from(""),
            String::from(""),
            false,
        );
        assert_eq!(d, DefaultPrincipal::default());
    }
}

impl Readable for DefaultPrincipal {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let type_ = String::read_ext(input, "type_", true)?;
        let name = String::read_ext(input, "name", true)?;
        let token_authenticated = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DefaultPrincipal {
            type_, name, token_authenticated, _unknown_tagged_fields
        })
    }
}

impl Writable for DefaultPrincipal {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.type_.write_ext(output, "self.type_", true)?;
        self.name.write_ext(output, "self.name", true)?;
        self.token_authenticated.write(output)?;
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
        crate::test_utils::test_java_default::<DefaultPrincipal>("DefaultPrincipal", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DefaultPrincipal) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DefaultPrincipal) {
            crate::test_utils::test_java_arbitrary(&data, "DefaultPrincipal", 0);
        }
    }
}
