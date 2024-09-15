// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_nullable_bytes, write_nullable_bytes};
use crate::markers::{ApiMessage, Data};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_option_bytes};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ConsumerProtocolSubscription {
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<String>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_bytes()"))]
    #[cfg_attr(test, serde(with="serde_option_bytes"))]
    pub user_data: Option<Vec<u8>>,
}

impl ApiMessage for ConsumerProtocolSubscription {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Data for ConsumerProtocolSubscription { }

impl Default for ConsumerProtocolSubscription {
    fn default() -> Self {
        ConsumerProtocolSubscription {
            topics: Vec::<String>::new(),
            user_data: None,
        }
    }
}

impl ConsumerProtocolSubscription {
    pub fn new(topics: Vec<String>, user_data: Option<Vec<u8>>) -> Self {
        Self {
            topics,
            user_data,
        }
    }
}

#[cfg(test)]
mod tests_consumer_protocol_subscription_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ConsumerProtocolSubscription::new(
            Vec::<String>::new(),
            None::<Vec::<u8>>,
        );
        assert_eq!(d, ConsumerProtocolSubscription::default());
    }
}

impl Readable for ConsumerProtocolSubscription {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<String>(input, "topics", false)?;
        let user_data = read_nullable_bytes(input, "user_data", false)?;
        Ok(ConsumerProtocolSubscription {
            topics, user_data
        })
    }
}

impl Writable for ConsumerProtocolSubscription {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, false)?;
        write_nullable_bytes(output, "self.user_data", self.user_data.as_deref(), false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ConsumerProtocolSubscription>("ConsumerProtocolSubscription", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ConsumerProtocolSubscription) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ConsumerProtocolSubscription) {
            crate::test_utils::test_java_arbitrary(&data, "ConsumerProtocolSubscription", 0);
        }
    }
}
