// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaRequest {
    /// The controller id.
    pub controller_id: i32,
    /// The controller epoch.
    pub controller_epoch: i32,
    /// Whether these partitions should be deleted.
    pub delete_partitions: bool,
    /// The partitions to stop.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub ungrouped_partitions: Vec<StopReplicaPartitionV0>,
}

impl ApiMessage for StopReplicaRequest {
    fn api_key(&self) -> i16 {
        5
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for StopReplicaRequest { }

impl Default for StopReplicaRequest {
    fn default() -> Self {
        StopReplicaRequest {
            controller_id: 0_i32,
            controller_epoch: 0_i32,
            delete_partitions: false,
            ungrouped_partitions: Vec::<StopReplicaPartitionV0>::new(),
        }
    }
}

impl StopReplicaRequest {
    pub fn new(controller_id: i32, controller_epoch: i32, delete_partitions: bool, ungrouped_partitions: Vec<StopReplicaPartitionV0>) -> Self {
        Self {
            controller_id,
            controller_epoch,
            delete_partitions,
            ungrouped_partitions,
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaRequest::new(
            0_i32,
            0_i32,
            false,
            Vec::<StopReplicaPartitionV0>::new(),
        );
        assert_eq!(d, StopReplicaRequest::default());
    }
}

impl Readable for StopReplicaRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let controller_id = i32::read(input)?;
        let controller_epoch = i32::read(input)?;
        let delete_partitions = bool::read(input)?;
        let ungrouped_partitions = read_array::<StopReplicaPartitionV0>(input, "ungrouped_partitions", false)?;
        Ok(StopReplicaRequest {
            controller_id, controller_epoch, delete_partitions, ungrouped_partitions
        })
    }
}

impl Writable for StopReplicaRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.controller_id.write(output)?;
        self.controller_epoch.write(output)?;
        self.delete_partitions.write(output)?;
        write_array(output, "self.ungrouped_partitions", &self.ungrouped_partitions, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaPartitionV0 {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partition index.
    pub partition_index: i32,
}

impl ApiMessage for StopReplicaPartitionV0 {
    fn api_key(&self) -> i16 {
        5
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for StopReplicaPartitionV0 { }

impl Default for StopReplicaPartitionV0 {
    fn default() -> Self {
        StopReplicaPartitionV0 {
            topic_name: String::from(""),
            partition_index: 0_i32,
        }
    }
}

impl StopReplicaPartitionV0 {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_partition_v0_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaPartitionV0::new(
            String::from(""),
            0_i32,
        );
        assert_eq!(d, StopReplicaPartitionV0::default());
    }
}

impl Readable for StopReplicaPartitionV0 {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", false)?;
        let partition_index = i32::read(input)?;
        Ok(StopReplicaPartitionV0 {
            topic_name, partition_index
        })
    }
}

impl Writable for StopReplicaPartitionV0 {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", false)?;
        self.partition_index.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<StopReplicaRequest>("StopReplicaRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: StopReplicaRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: StopReplicaRequest) {
            crate::test_utils::test_java_arbitrary(&data, "StopReplicaRequest", 0);
        }
    }
}
