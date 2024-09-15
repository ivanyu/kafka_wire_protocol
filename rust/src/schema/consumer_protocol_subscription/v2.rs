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
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub owned_partitions: Vec<TopicPartition>,
    /// 
    pub generation_id: i32,
}

impl ApiMessage for ConsumerProtocolSubscription {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Data for ConsumerProtocolSubscription { }

impl Default for ConsumerProtocolSubscription {
    fn default() -> Self {
        ConsumerProtocolSubscription {
            topics: Vec::<String>::new(),
            user_data: None,
            owned_partitions: Vec::<TopicPartition>::new(),
            generation_id: -1_i32,
        }
    }
}

impl ConsumerProtocolSubscription {
    pub fn new(topics: Vec<String>, user_data: Option<Vec<u8>>, owned_partitions: Vec<TopicPartition>, generation_id: i32) -> Self {
        Self {
            topics,
            user_data,
            owned_partitions,
            generation_id,
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
            Vec::<TopicPartition>::new(),
            -1_i32,
        );
        assert_eq!(d, ConsumerProtocolSubscription::default());
    }
}

impl Readable for ConsumerProtocolSubscription {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<String>(input, "topics", false)?;
        let user_data = read_nullable_bytes(input, "user_data", false)?;
        let owned_partitions = read_array::<TopicPartition>(input, "owned_partitions", false)?;
        let generation_id = i32::read(input)?;
        Ok(ConsumerProtocolSubscription {
            topics, user_data, owned_partitions, generation_id
        })
    }
}

impl Writable for ConsumerProtocolSubscription {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, false)?;
        write_nullable_bytes(output, "self.user_data", self.user_data.as_deref(), false)?;
        write_array(output, "self.owned_partitions", &self.owned_partitions, false)?;
        self.generation_id.write(output)?;
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
        crate::test_utils::test_java_default::<ConsumerProtocolSubscription>("ConsumerProtocolSubscription", 2);
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
            crate::test_utils::test_java_arbitrary(&data, "ConsumerProtocolSubscription", 2);
        }
    }
}
