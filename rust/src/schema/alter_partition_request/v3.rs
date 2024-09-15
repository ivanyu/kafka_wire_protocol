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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterPartitionRequest {
    /// The ID of the requesting broker
    pub broker_id: i32,
    /// The epoch of the requesting broker
    pub broker_epoch: i64,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicData>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterPartitionRequest {
    fn api_key(&self) -> i16 {
        56
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for AlterPartitionRequest { }

impl Default for AlterPartitionRequest {
    fn default() -> Self {
        AlterPartitionRequest {
            broker_id: 0_i32,
            broker_epoch: -1_i64,
            topics: Vec::<TopicData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterPartitionRequest {
    pub fn new(broker_id: i32, broker_epoch: i64, topics: Vec<TopicData>) -> Self {
        Self {
            broker_id,
            broker_epoch,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_partition_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterPartitionRequest::new(
            0_i32,
            -1_i64,
            Vec::<TopicData>::new(),
        );
        assert_eq!(d, AlterPartitionRequest::default());
    }
}

impl Readable for AlterPartitionRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let topics = read_array::<TopicData>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterPartitionRequest {
            broker_id, broker_epoch, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterPartitionRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.broker_epoch.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicData {
    /// The ID of the topic to alter ISRs for
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionData>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for TopicData {
    fn api_key(&self) -> i16 {
        56
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for TopicData { }

impl Default for TopicData {
    fn default() -> Self {
        TopicData {
            topic_id: Uuid::nil(),
            partitions: Vec::<PartitionData>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicData {
    pub fn new(topic_id: Uuid, partitions: Vec<PartitionData>) -> Self {
        Self {
            topic_id,
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_data_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicData::new(
            Uuid::nil(),
            Vec::<PartitionData>::new(),
        );
        assert_eq!(d, TopicData::default());
    }
}

impl Readable for TopicData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let partitions = read_array::<PartitionData>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicData {
            topic_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionData {
    /// The partition index
    pub partition_index: i32,
    /// The leader epoch of this partition
    pub leader_epoch: i32,
    /// 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub new_isr_with_epochs: Vec<BrokerState>,
    /// 1 if the partition is recovering from an unclean leader election; 0 otherwise.
    pub leader_recovery_state: i8,
    /// The expected epoch of the partition which is being updated. For legacy cluster this is the ZkVersion in the LeaderAndIsr request.
    pub partition_epoch: i32,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for PartitionData {
    fn api_key(&self) -> i16 {
        56
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for PartitionData { }

impl Default for PartitionData {
    fn default() -> Self {
        PartitionData {
            partition_index: 0_i32,
            leader_epoch: 0_i32,
            new_isr_with_epochs: Vec::<BrokerState>::new(),
            leader_recovery_state: 0_i8,
            partition_epoch: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionData {
    pub fn new(partition_index: i32, leader_epoch: i32, new_isr_with_epochs: Vec<BrokerState>, leader_recovery_state: i8, partition_epoch: i32) -> Self {
        Self {
            partition_index,
            leader_epoch,
            new_isr_with_epochs,
            leader_recovery_state,
            partition_epoch,
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
            Vec::<BrokerState>::new(),
            0_i8,
            0_i32,
        );
        assert_eq!(d, PartitionData::default());
    }
}

impl Readable for PartitionData {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_index = i32::read(input)?;
        let leader_epoch = i32::read(input)?;
        let new_isr_with_epochs = read_array::<BrokerState>(input, "new_isr_with_epochs", true)?;
        let leader_recovery_state = i8::read(input)?;
        let partition_epoch = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionData {
            partition_index, leader_epoch, new_isr_with_epochs, leader_recovery_state, partition_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionData {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_index.write(output)?;
        self.leader_epoch.write(output)?;
        write_array(output, "self.new_isr_with_epochs", &self.new_isr_with_epochs, true)?;
        self.leader_recovery_state.write(output)?;
        self.partition_epoch.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct BrokerState {
    /// The ID of the broker.
    pub broker_id: i32,
    /// The epoch of the broker. It will be -1 if the epoch check is not supported.
    pub broker_epoch: i64,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for BrokerState {
    fn api_key(&self) -> i16 {
        56
    }
    
    fn version(&self) -> i16 {
        3
    }
}

impl Request for BrokerState { }

impl Default for BrokerState {
    fn default() -> Self {
        BrokerState {
            broker_id: 0_i32,
            broker_epoch: -1_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl BrokerState {
    pub fn new(broker_id: i32, broker_epoch: i64) -> Self {
        Self {
            broker_id,
            broker_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_broker_state_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = BrokerState::new(
            0_i32,
            -1_i64,
        );
        assert_eq!(d, BrokerState::default());
    }
}

impl Readable for BrokerState {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let broker_id = i32::read(input)?;
        let broker_epoch = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(BrokerState {
            broker_id, broker_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for BrokerState {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.broker_id.write(output)?;
        self.broker_epoch.write(output)?;
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
        crate::test_utils::test_java_default::<AlterPartitionRequest>("AlterPartitionRequest", 3);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterPartitionRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterPartitionRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AlterPartitionRequest", 3);
        }
    }
}
