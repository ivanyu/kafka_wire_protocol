// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AlterReplicaLogDirsRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDirsRequest {
    /// The alterations to make for each directory.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub dirs: Vec<AlterReplicaLogDir>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for AlterReplicaLogDirsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        34
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        2
    }
}

impl Request for AlterReplicaLogDirsRequest { }

impl Default for AlterReplicaLogDirsRequest {
    fn default() -> Self {
        AlterReplicaLogDirsRequest {
            dirs: Vec::<AlterReplicaLogDir>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterReplicaLogDirsRequest {
    pub fn new(dirs: Vec<AlterReplicaLogDir>) -> Self {
        Self {
            dirs,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_replica_log_dirs_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterReplicaLogDirsRequest::new(
            Vec::<AlterReplicaLogDir>::new(),
        );
        assert_eq!(d, AlterReplicaLogDirsRequest::default());
    }
}

impl Readable for AlterReplicaLogDirsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let dirs = read_array::<AlterReplicaLogDir>(input, "dirs", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterReplicaLogDirsRequest {
            dirs, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterReplicaLogDirsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.dirs", &self.dirs, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AlterReplicaLogDir, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDir {
    /// The absolute directory path.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub path: String,
    /// The topics to add to the directory.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<AlterReplicaLogDirTopic>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AlterReplicaLogDir {
    fn default() -> Self {
        AlterReplicaLogDir {
            path: String::from(""),
            topics: Vec::<AlterReplicaLogDirTopic>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterReplicaLogDir {
    pub fn new<S1: AsRef<str>>(path: S1, topics: Vec<AlterReplicaLogDirTopic>) -> Self {
        Self {
            path: path.as_ref().to_string(),
            topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_replica_log_dir_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterReplicaLogDir::new(
            String::from(""),
            Vec::<AlterReplicaLogDirTopic>::new(),
        );
        assert_eq!(d, AlterReplicaLogDir::default());
    }
}

impl Readable for AlterReplicaLogDir {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let path = String::read_ext(input, "path", true)?;
        let topics = read_array::<AlterReplicaLogDirTopic>(input, "topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterReplicaLogDir {
            path, topics, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterReplicaLogDir {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.path.write_ext(output, "self.path", true)?;
        write_array(output, "self.topics", &self.topics, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AlterReplicaLogDirTopic, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDirTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partition indexes.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AlterReplicaLogDirTopic {
    fn default() -> Self {
        AlterReplicaLogDirTopic {
            name: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AlterReplicaLogDirTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_alter_replica_log_dir_topic_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AlterReplicaLogDirTopic::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, AlterReplicaLogDirTopic::default());
    }
}

impl Readable for AlterReplicaLogDirTopic {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AlterReplicaLogDirTopic {
            name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for AlterReplicaLogDirTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
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
        crate::test_utils::test_java_default::<AlterReplicaLogDirsRequest>("AlterReplicaLogDirsRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: AlterReplicaLogDirsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: AlterReplicaLogDirsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "AlterReplicaLogDirsRequest", 2);
        }
    }
}
