// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FindCoordinatorRequest {
    /// The coordinator key.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub key: String,
}

impl ApiMessage for FindCoordinatorRequest {
    fn api_key(&self) -> i16 {
        10
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for FindCoordinatorRequest { }

impl Default for FindCoordinatorRequest {
    fn default() -> Self {
        FindCoordinatorRequest {
            key: String::from(""),
        }
    }
}

impl FindCoordinatorRequest {
    pub fn new<S1: AsRef<str>>(key: S1) -> Self {
        Self {
            key: key.as_ref().to_string(),
        }
    }
}

#[cfg(test)]
mod tests_find_coordinator_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FindCoordinatorRequest::new(
            String::from(""),
        );
        assert_eq!(d, FindCoordinatorRequest::default());
    }
}

impl Readable for FindCoordinatorRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let key = String::read_ext(input, "key", false)?;
        Ok(FindCoordinatorRequest {
            key
        })
    }
}

impl Writable for FindCoordinatorRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key.write_ext(output, "self.key", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<FindCoordinatorRequest>("FindCoordinatorRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: FindCoordinatorRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: FindCoordinatorRequest) {
            crate::test_utils::test_java_arbitrary(&data, "FindCoordinatorRequest", 0);
        }
    }
}
