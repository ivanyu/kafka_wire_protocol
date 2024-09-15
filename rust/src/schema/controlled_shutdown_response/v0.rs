// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ControlledShutdownResponse {
    /// The top-level error code.
    pub error_code: i16,
    /// The partitions that the broker still leads.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub remaining_partitions: Vec<RemainingPartition>,
}

impl ApiMessage for ControlledShutdownResponse {
    fn api_key(&self) -> i16 {
        7
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ControlledShutdownResponse { }

impl Default for ControlledShutdownResponse {
    fn default() -> Self {
        ControlledShutdownResponse {
            error_code: 0_i16,
            remaining_partitions: Vec::<RemainingPartition>::new(),
        }
    }
}

impl ControlledShutdownResponse {
    pub fn new(error_code: i16, remaining_partitions: Vec<RemainingPartition>) -> Self {
        Self {
            error_code,
            remaining_partitions,
        }
    }
}

#[cfg(test)]
mod tests_controlled_shutdown_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ControlledShutdownResponse::new(
            0_i16,
            Vec::<RemainingPartition>::new(),
        );
        assert_eq!(d, ControlledShutdownResponse::default());
    }
}

impl Readable for ControlledShutdownResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let remaining_partitions = read_array::<RemainingPartition>(input, "remaining_partitions", false)?;
        Ok(ControlledShutdownResponse {
            error_code, remaining_partitions
        })
    }
}

impl Writable for ControlledShutdownResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        write_array(output, "self.remaining_partitions", &self.remaining_partitions, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RemainingPartition {
    /// The name of the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The index of the partition.
    pub partition_index: i32,
}

impl ApiMessage for RemainingPartition {
    fn api_key(&self) -> i16 {
        7
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for RemainingPartition { }

impl Default for RemainingPartition {
    fn default() -> Self {
        RemainingPartition {
            topic_name: String::from(""),
            partition_index: 0_i32,
        }
    }
}

impl RemainingPartition {
    pub fn new<S1: AsRef<str>>(topic_name: S1, partition_index: i32) -> Self {
        Self {
            topic_name: topic_name.as_ref().to_string(),
            partition_index,
        }
    }
}

#[cfg(test)]
mod tests_remaining_partition_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = RemainingPartition::new(
            String::from(""),
            0_i32,
        );
        assert_eq!(d, RemainingPartition::default());
    }
}

impl Readable for RemainingPartition {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_name = String::read_ext(input, "topic_name", false)?;
        let partition_index = i32::read(input)?;
        Ok(RemainingPartition {
            topic_name, partition_index
        })
    }
}

impl Writable for RemainingPartition {
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
        crate::test_utils::test_java_default::<ControlledShutdownResponse>("ControlledShutdownResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ControlledShutdownResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ControlledShutdownResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ControlledShutdownResponse", 0);
        }
    }
}
