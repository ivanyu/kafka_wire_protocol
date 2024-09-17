// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ElectLeadersRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ElectLeadersRequest {
    /// Type of elections to conduct for the partition. A value of '0' elects the preferred replica. A value of '1' elects the first live replica if there are no in-sync replica.
    pub election_type: i8,
    /// The topic partitions to elect leaders.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub topic_partitions: Option<Vec<TopicPartitions>>,
    /// The time in ms to wait for the election to complete.
    pub timeout_ms: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ElectLeadersRequest {
    fn api_key(&self) -> i16 {
        43
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for ElectLeadersRequest { }

impl Default for ElectLeadersRequest {
    fn default() -> Self {
        ElectLeadersRequest {
            election_type: 0_i8,
            topic_partitions: Some(Vec::<TopicPartitions>::new()),
            timeout_ms: 60000_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ElectLeadersRequest {
    pub fn new(election_type: i8, topic_partitions: Option<Vec<TopicPartitions>>, timeout_ms: i32) -> Self {
        Self {
            election_type,
            topic_partitions,
            timeout_ms,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_elect_leaders_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ElectLeadersRequest::new(
            0_i8,
            Some(Vec::<TopicPartitions>::new()),
            60000_i32,
        );
        assert_eq!(d, ElectLeadersRequest::default());
    }
}

impl Readable for ElectLeadersRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let election_type = i8::read(input)?;
        let topic_partitions = read_nullable_array::<TopicPartitions>(input, "topic_partitions", true)?;
        let timeout_ms = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ElectLeadersRequest {
            election_type, topic_partitions, timeout_ms, _unknown_tagged_fields
        })
    }
}

impl Writable for ElectLeadersRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.election_type.write(output)?;
        write_nullable_array(output, "self.topic_partitions", self.topic_partitions.as_deref(), true)?;
        self.timeout_ms.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicPartitions, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicPartitions {
    /// The name of a topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// The partitions of this topic whose leader should be elected.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicPartitions {
    fn default() -> Self {
        TopicPartitions {
            topic: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicPartitions {
    pub fn new<S1: AsRef<str>>(topic: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_partitions_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicPartitions::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, TopicPartitions::default());
    }
}

impl Readable for TopicPartitions {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicPartitions {
            topic, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicPartitions {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
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
        crate::test_utils::test_java_default::<ElectLeadersRequest>("ElectLeadersRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ElectLeadersRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ElectLeadersRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ElectLeadersRequest", 2);
        }
    }
}
