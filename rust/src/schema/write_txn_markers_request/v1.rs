// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// WriteTxnMarkersRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WriteTxnMarkersRequest {
    /// The transaction markers to be written.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub markers: Vec<WritableTxnMarker>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for WriteTxnMarkersRequest {
    fn api_key(&self) -> i16 {
        27
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for WriteTxnMarkersRequest { }

impl Default for WriteTxnMarkersRequest {
    fn default() -> Self {
        WriteTxnMarkersRequest {
            markers: Vec::<WritableTxnMarker>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl WriteTxnMarkersRequest {
    pub fn new(markers: Vec<WritableTxnMarker>) -> Self {
        Self {
            markers,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_write_txn_markers_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WriteTxnMarkersRequest::new(
            Vec::<WritableTxnMarker>::new(),
        );
        assert_eq!(d, WriteTxnMarkersRequest::default());
    }
}

impl Readable for WriteTxnMarkersRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let markers = read_array::<WritableTxnMarker>(input, "markers", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(WriteTxnMarkersRequest {
            markers, _unknown_tagged_fields
        })
    }
}

impl Writable for WriteTxnMarkersRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.markers", &self.markers, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// WritableTxnMarker, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WritableTxnMarker {
    /// The current producer ID.
    pub producer_id: i64,
    /// The current epoch associated with the producer ID.
    pub producer_epoch: i16,
    /// The result of the transaction to write to the partitions (false = ABORT, true = COMMIT).
    pub transaction_result: bool,
    /// Each topic that we want to write transaction marker(s) for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<WritableTxnMarkerTopic>,
    /// Epoch associated with the transaction state partition hosted by this transaction coordinator
    pub coordinator_epoch: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for WritableTxnMarker {
    fn default() -> Self {
        WritableTxnMarker {
            producer_id: 0_i64,
            producer_epoch: 0_i16,
            transaction_result: false,
            topics: Vec::<WritableTxnMarkerTopic>::new(),
            coordinator_epoch: 0_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl WritableTxnMarker {
    pub fn new(producer_id: i64, producer_epoch: i16, transaction_result: bool, topics: Vec<WritableTxnMarkerTopic>, coordinator_epoch: i32) -> Self {
        Self {
            producer_id,
            producer_epoch,
            transaction_result,
            topics,
            coordinator_epoch,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_writable_txn_marker_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WritableTxnMarker::new(
            0_i64,
            0_i16,
            false,
            Vec::<WritableTxnMarkerTopic>::new(),
            0_i32,
        );
        assert_eq!(d, WritableTxnMarker::default());
    }
}

impl Readable for WritableTxnMarker {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let producer_id = i64::read(input)?;
        let producer_epoch = i16::read(input)?;
        let transaction_result = bool::read(input)?;
        let topics = read_array::<WritableTxnMarkerTopic>(input, "topics", true)?;
        let coordinator_epoch = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(WritableTxnMarker {
            producer_id, producer_epoch, transaction_result, topics, coordinator_epoch, _unknown_tagged_fields
        })
    }
}

impl Writable for WritableTxnMarker {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.producer_id.write(output)?;
        self.producer_epoch.write(output)?;
        self.transaction_result.write(output)?;
        write_array(output, "self.topics", &self.topics, true)?;
        self.coordinator_epoch.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// WritableTxnMarkerTopic, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct WritableTxnMarkerTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The indexes of the partitions to write transaction markers for.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_indexes: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for WritableTxnMarkerTopic {
    fn default() -> Self {
        WritableTxnMarkerTopic {
            name: String::from(""),
            partition_indexes: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl WritableTxnMarkerTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partition_indexes: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partition_indexes,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_writable_txn_marker_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = WritableTxnMarkerTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, WritableTxnMarkerTopic::default());
    }
}

impl Readable for WritableTxnMarkerTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partition_indexes = read_array::<i32>(input, "partition_indexes", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(WritableTxnMarkerTopic {
            name, partition_indexes, _unknown_tagged_fields
        })
    }
}

impl Writable for WritableTxnMarkerTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_array(output, "self.partition_indexes", &self.partition_indexes, true)?;
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
        crate::test_utils::test_java_default::<WriteTxnMarkersRequest>("WriteTxnMarkersRequest", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: WriteTxnMarkersRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: WriteTxnMarkersRequest) {
            crate::test_utils::test_java_arbitrary(&data, "WriteTxnMarkersRequest", 1);
        }
    }
}
