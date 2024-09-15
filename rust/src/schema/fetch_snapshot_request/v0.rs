// This file was generated. Do not edit.

use std::io::{Cursor, Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct FetchSnapshotRequest {
    /// The clusterId if known, this is used to validate metadata fetches prior to broker registration
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub cluster_id: Option<String>,
    /// The broker ID of the follower
    pub replica_id: i32,
    /// The maximum bytes to fetch from all of the snapshots
    pub max_bytes: i32,
    /// The topics to fetch
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<TopicSnapshot>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for FetchSnapshotRequest {
    fn api_key(&self) -> i16 {
        59
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for FetchSnapshotRequest { }

impl Default for FetchSnapshotRequest {
    fn default() -> Self {
        FetchSnapshotRequest {
            cluster_id: None,
            replica_id: -1_i32,
            max_bytes: 0x7fffffff_i32,
            topics: Vec::<TopicSnapshot>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl FetchSnapshotRequest {
    pub fn new<S1: AsRef<str>>(cluster_id: Option<S1>, replica_id: i32, max_bytes: i32, topics: Vec<TopicSnapshot>) -> Self {
        Self {
            cluster_id: cluster_id.map(|s| s.as_ref().to_string()),
            replica_id,
            max_bytes,
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_fetch_snapshot_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = FetchSnapshotRequest::new(
            None::<String>,
            -1_i32,
            0x7fffffff_i32,
            Vec::<TopicSnapshot>::new(),
        );
        assert_eq!(d, FetchSnapshotRequest::default());
    }
}

impl Readable for FetchSnapshotRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let mut cluster_id = None;
        let replica_id = i32::read(input)?;
        let max_bytes = i32::read(input)?;
        let topics = read_array::<TopicSnapshot>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, tag_data: &[u8]| {
            match tag {
                0 => {
                    let mut cur = Cursor::new(tag_data);
                    cluster_id = Option::<String>::read_ext(&mut cur, "cluster_id", true)?;
                    Ok(true)
                },
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(FetchSnapshotRequest {
            cluster_id, replica_id, max_bytes, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for FetchSnapshotRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.replica_id.write(output)?;
        self.max_bytes.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        let mut known_tagged_fields = Vec::<RawTaggedField>::new();
        if self.cluster_id.is_some() {
            let mut cur = Cursor::new(Vec::<u8>::new());
            self.cluster_id.write_ext(&mut cur, "self.cluster_id", true)?;
            known_tagged_fields.push(RawTaggedField { tag: 0, data: cur.into_inner() });
        }
        write_tagged_fields(output, &known_tagged_fields, &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicSnapshot {
    /// The name of the topic to fetch
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partitions to fetch
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<PartitionSnapshot>,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for TopicSnapshot {
    fn api_key(&self) -> i16 {
        59
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for TopicSnapshot { }

impl Default for TopicSnapshot {
    fn default() -> Self {
        TopicSnapshot {
            name: String::from(""),
            partitions: Vec::<PartitionSnapshot>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicSnapshot {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<PartitionSnapshot>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_snapshot_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicSnapshot::new(
            String::from(""),
            Vec::<PartitionSnapshot>::new(),
        );
        assert_eq!(d, TopicSnapshot::default());
    }
}

impl Readable for TopicSnapshot {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<PartitionSnapshot>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicSnapshot {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicSnapshot {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionSnapshot {
    /// The partition index
    pub partition: i32,
    /// The current leader epoch of the partition, -1 for unknown leader epoch
    pub current_leader_epoch: i32,
    /// The snapshot endOffset and epoch to fetch
    pub snapshot_id: SnapshotId,
    /// The byte position within the snapshot to start fetching from
    pub position: i64,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for PartitionSnapshot {
    fn api_key(&self) -> i16 {
        59
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for PartitionSnapshot { }

impl Default for PartitionSnapshot {
    fn default() -> Self {
        PartitionSnapshot {
            partition: 0_i32,
            current_leader_epoch: 0_i32,
            snapshot_id: SnapshotId::default(),
            position: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl PartitionSnapshot {
    pub fn new(partition: i32, current_leader_epoch: i32, snapshot_id: SnapshotId, position: i64) -> Self {
        Self {
            partition,
            current_leader_epoch,
            snapshot_id,
            position,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_partition_snapshot_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionSnapshot::new(
            0_i32,
            0_i32,
            SnapshotId::default(),
            0_i64,
        );
        assert_eq!(d, PartitionSnapshot::default());
    }
}

impl Readable for PartitionSnapshot {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition = i32::read(input)?;
        let current_leader_epoch = i32::read(input)?;
        let snapshot_id = SnapshotId::read(input)?;
        let position = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(PartitionSnapshot {
            partition, current_leader_epoch, snapshot_id, position, _unknown_tagged_fields
        })
    }
}

impl Writable for PartitionSnapshot {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition.write(output)?;
        self.current_leader_epoch.write(output)?;
        self.snapshot_id.write(output)?;
        self.position.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct SnapshotId {
    /// 
    pub end_offset: i64,
    /// 
    pub epoch: i32,
    /// Unknown tagged fields
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for SnapshotId {
    fn api_key(&self) -> i16 {
        59
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for SnapshotId { }

impl Default for SnapshotId {
    fn default() -> Self {
        SnapshotId {
            end_offset: 0_i64,
            epoch: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl SnapshotId {
    pub fn new(end_offset: i64, epoch: i32) -> Self {
        Self {
            end_offset,
            epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_snapshot_id_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = SnapshotId::new(
            0_i64,
            0_i32,
        );
        assert_eq!(d, SnapshotId::default());
    }
}

impl Readable for SnapshotId {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let end_offset = i64::read(input)?;
        let epoch = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(SnapshotId {
            end_offset, epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for SnapshotId {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.end_offset.write(output)?;
        self.epoch.write(output)?;
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
        crate::test_utils::test_java_default::<FetchSnapshotRequest>("FetchSnapshotRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: FetchSnapshotRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: FetchSnapshotRequest) {
            crate::test_utils::test_java_arbitrary(&data, "FetchSnapshotRequest", 0);
        }
    }
}
