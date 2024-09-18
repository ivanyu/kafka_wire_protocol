// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::markers::{ApiMessage, Header};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ResponseHeader, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ResponseHeader {
    /// The correlation ID of this response.
    pub correlation_id: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ResponseHeader {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Header for ResponseHeader { }

impl Default for ResponseHeader {
    fn default() -> Self {
        ResponseHeader {
            correlation_id: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ResponseHeader {
    pub fn new(correlation_id: i32) -> Self {
        Self {
            correlation_id,
            _unknown_tagged_fields: vec![],
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
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ResponseHeader {
            correlation_id, _unknown_tagged_fields
        })
    }
}

impl Writable for ResponseHeader {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.correlation_id.write(output)?;
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
        crate::test_utils::test_java_default::<ResponseHeader>("ResponseHeader", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "ResponseHeader", 1);
        }
    }
}
