// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Header};
use crate::readable_writable::{Readable, Writable};

/// ResponseHeader, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ResponseHeader {
    /// The correlation ID of this response.
    pub correlation_id: i32,
}

impl ApiMessage for ResponseHeader {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Header for ResponseHeader { }

impl Default for ResponseHeader {
    fn default() -> Self {
        ResponseHeader {
            correlation_id: 0_i32,
        }
    }
}

impl ResponseHeader {
    pub fn new(correlation_id: i32) -> Self {
        Self {
            correlation_id,
        }
    }
}

#[cfg(test)]
mod tests_response_header_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ResponseHeader::new(
            0_i32,
        );
        assert_eq!(d, ResponseHeader::default());
    }
}

impl Readable for ResponseHeader {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let correlation_id = i32::read(input)?;
        Ok(ResponseHeader {
            correlation_id
        })
    }
}

impl Writable for ResponseHeader {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.correlation_id.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ResponseHeader>("ResponseHeader", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ResponseHeader) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ResponseHeader) {
            crate::test_utils::test_java_arbitrary(&data, "ResponseHeader", 0);
        }
    }
}
