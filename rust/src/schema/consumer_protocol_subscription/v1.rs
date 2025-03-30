// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_nullable_bytes, write_nullable_bytes};
use crate::markers::{ApiMessage, Data};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_option_bytes};

/// ConsumerProtocolSubscription, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ConsumerProtocolSubscription {
    /// The topics that the member wants to consume.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<String>,
    /// User data that will be passed back to the consumer.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_bytes()"))]
    #[cfg_attr(test, serde(with="serde_option_bytes"))]
    pub user_data: Option<Vec<u8>>,
    /// The partitions that the member owns.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub owned_partitions: Vec<TopicPartition>,
}

impl ApiMessage for ConsumerProtocolSubscription {
    fn api_key(&self) -> i16 {
        -1
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Data for ConsumerProtocolSubscription { }

impl Default for ConsumerProtocolSubscription {
    fn default() -> Self {
        ConsumerProtocolSubscription {
            topics: Vec::<String>::new(),
            user_data: None,
            owned_partitions: Vec::<TopicPartition>::new(),
        }
    }
}

impl ConsumerProtocolSubscription {
    pub fn new(topics: Vec<String>, user_data: Option<Vec<u8>>, owned_partitions: Vec<TopicPartition>) -> Self {
        Self {
            topics,
            user_data,
            owned_partitions,
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
        );
        assert_eq!(d, ConsumerProtocolSubscription::default());
    }
}

impl Readable for ConsumerProtocolSubscription {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topics = read_array::<String>(input, "topics", false)?;
        let user_data = read_nullable_bytes(input, "user_data", false)?;
        let owned_partitions = read_array::<TopicPartition>(input, "owned_partitions", false)?;
        Ok(ConsumerProtocolSubscription {
            topics, user_data, owned_partitions
        })
    }
}

impl Writable for ConsumerProtocolSubscription {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topics", &self.topics, false)?;
        write_nullable_bytes(output, "self.user_data", self.user_data.as_deref(), false)?;
        write_array(output, "self.owned_partitions", &self.owned_partitions, false)?;
        Ok(())
    }
}

/// TopicPartition, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicPartition {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// The partition ids.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
}

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
        crate::test_utils::test_java_default::<ConsumerProtocolSubscription>("ConsumerProtocolSubscription", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "ConsumerProtocolSubscription", 1);
        }
    }
}
