// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ElectLeadersResponse, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ElectLeadersResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The top level response error code.
    pub error_code: i16,
    /// The election results, or an empty array if the requester did not have permission and the request asks for all partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub replica_election_results: Vec<ReplicaElectionResult>,
}

impl ApiMessage for ElectLeadersResponse {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        43
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        1
    }
}

impl Response for ElectLeadersResponse { }

impl Default for ElectLeadersResponse {
    fn default() -> Self {
        ElectLeadersResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            replica_election_results: Vec::<ReplicaElectionResult>::new(),
        }
    }
}

impl ElectLeadersResponse {
    pub fn new(throttle_time_ms: i32, error_code: i16, replica_election_results: Vec<ReplicaElectionResult>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            replica_election_results,
        }
    }
}

#[cfg(test)]
mod tests_elect_leaders_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ElectLeadersResponse::new(
            0_i32,
            0_i16,
            Vec::<ReplicaElectionResult>::new(),
        );
        assert_eq!(d, ElectLeadersResponse::default());
    }
}

impl Readable for ElectLeadersResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let replica_election_results = read_array::<ReplicaElectionResult>(input, "replica_election_results", false)?;
        Ok(ElectLeadersResponse {
            throttle_time_ms, error_code, replica_election_results
        })
    }
}

impl Writable for ElectLeadersResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        write_array(output, "self.replica_election_results", &self.replica_election_results, false)?;
        Ok(())
    }
}

/// ReplicaElectionResult, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ReplicaElectionResult {
    /// The topic name
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic: String,
    /// The results for each partition
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partition_result: Vec<PartitionResult>,
}

impl Default for ReplicaElectionResult {
    fn default() -> Self {
        ReplicaElectionResult {
            topic: String::from(""),
            partition_result: Vec::<PartitionResult>::new(),
        }
    }
}

impl ReplicaElectionResult {
    pub fn new<S1: AsRef<str>>(topic: S1, partition_result: Vec<PartitionResult>) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            partition_result,
        }
    }
}

#[cfg(test)]
mod tests_replica_election_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ReplicaElectionResult::new(
            String::from(""),
            Vec::<PartitionResult>::new(),
        );
        assert_eq!(d, ReplicaElectionResult::default());
    }
}

impl Readable for ReplicaElectionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic = String::read_ext(input, "topic", false)?;
        let partition_result = read_array::<PartitionResult>(input, "partition_result", false)?;
        Ok(ReplicaElectionResult {
            topic, partition_result
        })
    }
}

impl Writable for ReplicaElectionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic.write_ext(output, "self.topic", false)?;
        write_array(output, "self.partition_result", &self.partition_result, false)?;
        Ok(())
    }
}

/// PartitionResult, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct PartitionResult {
    /// The partition id
    pub partition_id: i32,
    /// The result error, or zero if there was no error.
    pub error_code: i16,
    /// The result message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
}

impl Default for PartitionResult {
    fn default() -> Self {
        PartitionResult {
            partition_id: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
        }
    }
}

impl PartitionResult {
    pub fn new<S1: AsRef<str>>(partition_id: i32, error_code: i16, error_message: Option<S1>) -> Self {
        Self {
            partition_id,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
        }
    }
}

#[cfg(test)]
mod tests_partition_result_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = PartitionResult::new(
            0_i32,
            0_i16,
            Some(String::from("")),
        );
        assert_eq!(d, PartitionResult::default());
    }
}

impl Readable for PartitionResult {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let partition_id = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        Ok(PartitionResult {
            partition_id, error_code, error_message
        })
    }
}

impl Writable for PartitionResult {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.partition_id.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<ElectLeadersResponse>("ElectLeadersResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ElectLeadersResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ElectLeadersResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ElectLeadersResponse", 1);
        }
    }
}
