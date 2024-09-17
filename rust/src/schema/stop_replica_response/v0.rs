// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// StopReplicaResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaResponse {
    /// The top-level error code, or 0 if there was no top-level error.
    pub error_code: i16,
    /// The responses for each partition.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_errors: Vec<StopReplicaPartitionError>,
}

impl ApiMessage for StopReplicaResponse {
    fn api_key(&self) -> i16 {
        5
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for StopReplicaResponse { }

impl Default for StopReplicaResponse {
    fn default() -> Self {
        StopReplicaResponse {
            error_code: 0_i16,
            partition_errors: Vec::<StopReplicaPartitionError>::new(),
        }
    }
}

impl StopReplicaResponse {
    pub fn new(error_code: i16, partition_errors: Vec<StopReplicaPartitionError>) -> Self {
        Self {
            error_code,
            partition_errors,
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaResponse::new(
            0_i16,
            Vec::<StopReplicaPartitionError>::new(),
        );
        assert_eq!(d, StopReplicaResponse::default());
    }
}

impl Readable for StopReplicaResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let partition_errors = read_array::<StopReplicaPartitionError>(input, "partition_errors", false)?;
        Ok(StopReplicaResponse {
            error_code, partition_errors
        })
    }
}

impl Writable for StopReplicaResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.partition_errors", &self.partition_errors, false)?;
        Ok(())
    }
}

/// StopReplicaPartitionError, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StopReplicaPartitionError {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partition index.
    pub partition_index: i32,
    /// The partition error code, or 0 if there was no partition error.
    pub error_code: i16,
}

impl Default for StopReplicaPartitionError {
    fn default() -> Self {
        StopReplicaPartitionError {
            topic_name: String::from(""),
            partition_index: 0_i32,
            error_code: 0_i16,
        }
    }
}

impl StopReplicaPartitionError {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32, error_code: i16) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
            error_code,
        }
    }
}

#[cfg(test)]
mod tests_stop_replica_partition_error_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StopReplicaPartitionError::new(
            String::from(""),
            0_i32,
            0_i16,
        );
        assert_eq!(d, StopReplicaPartitionError::default());
    }
}

impl Readable for StopReplicaPartitionError {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", false)?;
        let partition_index = i32::read(input)?;
        let error_code = i16::read(input)?;
        Ok(StopReplicaPartitionError {
            topic_name, partition_index, error_code
        })
    }
}

impl Writable for StopReplicaPartitionError {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_name.write_ext(output, "self.topic_name", false)?;
        self.partition_index.write(output)?;
        self.error_code.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<StopReplicaResponse>("StopReplicaResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: StopReplicaResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: StopReplicaResponse) {
            crate::test_utils::test_java_arbitrary(&data, "StopReplicaResponse", 0);
        }
    }
}
