// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// InitializeShareGroupStateRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct InitializeShareGroupStateRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The data for the topics.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<InitializeStateData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for InitializeShareGroupStateRequest {
    fn api_key(&self) -> i16 {
        83
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for InitializeShareGroupStateRequest { }

impl Default for InitializeShareGroupStateRequest {
    fn default() -> Self {
        InitializeShareGroupStateRequest {
            group_id: String::from(""),
            topics: Vec::<InitializeStateData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl InitializeShareGroupStateRequest {
    pub fn new<S1: AsRef<str>>(group_id: S1, topics: Vec<InitializeStateData>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_initialize_share_group_state_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = InitializeShareGroupStateRequest::new(
            String::from(""),
            Vec::<InitializeStateData>::new(),
        );
        assert_eq!(d, InitializeShareGroupStateRequest::default());
    }
}

impl Readable for InitializeShareGroupStateRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let topics = read_array::<InitializeStateData>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(InitializeShareGroupStateRequest {
            group_id, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for InitializeShareGroupStateRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// InitializeStateData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct InitializeStateData {
    /// The topic identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The data for the partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionData>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for InitializeStateData {
    fn default() -> Self {
        InitializeStateData {
            topic_id: Uuid::nil(),
            partitions: Vec::<PartitionData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl InitializeStateData {
    pub fn new(topic_id: Uuid, partitions: Vec<PartitionData>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_initialize_state_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = InitializeStateData::new(
            Uuid::nil(),
            Vec::<PartitionData>::new(),
        );
        assert_eq!(d, InitializeStateData::default());
    }
}

impl Readable for InitializeStateData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<PartitionData>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(InitializeStateData {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for InitializeStateData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// PartitionData, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index.
    pub partition: i32,
    /// The state epoch for this share-partition.
    pub state_epoch: i32,
    /// The share-partition start offset, or -1 if the start offset is not being initialized.
    pub start_offset: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition: 0_i32,
            state_epoch: 0_i32,
            start_offset: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new(partition: i32, state_epoch: i32, start_offset: i64) -> Self {
        Self {
            partition,
            state_epoch,
            start_offset,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_partition_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionData::new(
            0_i32,
            0_i32,
            0_i64,
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let state_epoch = i32::read(input)?;
        let start_offset = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition, state_epoch, start_offset, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.state_epoch.write(output)?;
        self.start_offset.write(output)?;
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
        crate::test_utils::test_java_default::<InitializeShareGroupStateRequest>("InitializeShareGroupStateRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: InitializeShareGroupStateRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: InitializeShareGroupStateRequest) {
            crate::test_utils::test_java_arbitrary(&data, "InitializeShareGroupStateRequest", 0);
        }
    }
}
