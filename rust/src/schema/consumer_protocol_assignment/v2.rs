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
pub struct ConsumerProtocolAssignment {
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub assigned_partitions: Vec<TopicPartition>,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_bytes()"))]
    #[cfg_attr(test, serde(with="serde_option_bytes"))]
    pub user_data: Option<Vec<u8>>,
}

impl ApiMessage for ConsumerProtocolAssignment {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Data for ConsumerProtocolAssignment { }

impl Default for ConsumerProtocolAssignment {
    fn default() -> Self {
        ConsumerProtocolAssignment {
            assigned_partitions: Vec::<TopicPartition>::new(),
            user_data: None,
        }
    }
}

impl ConsumerProtocolAssignment {
    pub fn new(assigned_partitions: Vec<TopicPartition>, user_data: Option<Vec<u8>>) -> Self {
        Self {
            assigned_partitions,
            user_data,
        }
    }
}

#[cfg(test)]
mod tests_consumer_protocol_assignment_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ConsumerProtocolAssignment::new(
            Vec::<TopicPartition>::new(),
            None::<Vec::<u8>>,
        );
        assert_eq!(d, ConsumerProtocolAssignment::default());
    }
}

impl Readable for ConsumerProtocolAssignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let assigned_partitions = read_array::<TopicPartition>(input, "assigned_partitions", false)?;
        let user_data = read_nullable_bytes(input, "user_data", false)?;
        Ok(ConsumerProtocolAssignment {
            assigned_partitions, user_data
        })
    }
}

impl Writable for ConsumerProtocolAssignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.assigned_partitions", &self.assigned_partitions, false)?;
        write_nullable_bytes(output, "self.user_data", self.user_data.as_deref(), false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicPartition {
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
}

impl ApiMessage for TopicPartition {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Data for TopicPartition { }

impl Default for TopicPartition {
    fn default() -> Self {
        TopicPartition {
            topic: String::from(""),
            partitions: Vec::<i32>::new(),
        }
    }
}

impl TopicPartition {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
        }
    }
}

#[cfg(test)]
mod tests_topic_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicPartition::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, TopicPartition::default());
    }
}

impl Readable for TopicPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", false)?;
        let partitions = read_array::<i32>(input, "partitions", false)?;
        Ok(TopicPartition {
            topic, partitions
        })
    }
}

impl Writable for TopicPartition {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ConsumerProtocolAssignment>("ConsumerProtocolAssignment", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ConsumerProtocolAssignment) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ConsumerProtocolAssignment) {
            crate::test_utils::test_java_arbitrary(&data, "ConsumerProtocolAssignment", 2);
        }
    }
}
