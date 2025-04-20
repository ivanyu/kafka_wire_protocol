// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// DeleteTopicsRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DeleteTopicsRequest {
    /// The names of the topics to delete.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topic_names: Vec<String>,
    /// The length of time in milliseconds to wait for the deletions to complete.
    pub timeout_ms: i32,
}

impl ApiMessage for DeleteTopicsRequest {
    fn api_key(&self) -> i16 {
        20
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for DeleteTopicsRequest { }

impl Default for DeleteTopicsRequest {
    fn default() -> Self {
        DeleteTopicsRequest {
            topic_names: Vec::<String>::new(),
            timeout_ms: 0_i32,
        }
    }
}

impl DeleteTopicsRequest {
    pub fn new(topic_names: Vec<String>, timeout_ms: i32) -> Self {
        Self {
            topic_names,
            timeout_ms,
        }
    }
}

#[cfg(test)]
mod tests_delete_topics_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DeleteTopicsRequest::new(
            Vec::<String>::new(),
            0_i32,
        );
        assert_eq!(d, DeleteTopicsRequest::default());
    }
}

impl Readable for DeleteTopicsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_names = read_array::<String>(input, "topic_names", false)?;
        let timeout_ms = i32::read(input)?;
        Ok(DeleteTopicsRequest {
            topic_names, timeout_ms
        })
    }
}

impl Writable for DeleteTopicsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topic_names", &self.topic_names, false)?;
        self.timeout_ms.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<DeleteTopicsRequest>("DeleteTopicsRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DeleteTopicsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DeleteTopicsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "DeleteTopicsRequest", 2);
        }
    }
}
