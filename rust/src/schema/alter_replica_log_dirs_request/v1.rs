// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// AlterReplicaLogDirsRequest, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDirsRequest {
    /// The alterations to make for each directory.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub dirs: Vec<AlterReplicaLogDir>,
}

impl ApiMessage for AlterReplicaLogDirsRequest {
    fn api_key(&self) -> i16 {
        34
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Request for AlterReplicaLogDirsRequest { }

impl Default for AlterReplicaLogDirsRequest {
    fn default() -> Self {
        AlterReplicaLogDirsRequest {
            dirs: Vec::<AlterReplicaLogDir>::new(),
        }
    }
}

impl AlterReplicaLogDirsRequest {
    pub fn new(dirs: Vec<AlterReplicaLogDir>) -> Self {
        Self {
            dirs,
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
        let dirs = read_array::<AlterReplicaLogDir>(input, "dirs", false)?;
        Ok(AlterReplicaLogDirsRequest {
            dirs
        })
    }
}

impl Writable for AlterReplicaLogDirsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.dirs", &self.dirs, false)?;
        Ok(())
    }
}

/// AlterReplicaLogDir, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDir {
    /// The absolute directory path.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub path: String,
    /// The topics to add to the directory.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topics: Vec<AlterReplicaLogDirTopic>,
}

impl Default for AlterReplicaLogDir {
    fn default() -> Self {
        AlterReplicaLogDir {
            path: String::from(""),
            topics: Vec::<AlterReplicaLogDirTopic>::new(),
        }
    }
}

impl AlterReplicaLogDir {
    pub fn new<S1: AsRef<str>>(path: S1, topics: Vec<AlterReplicaLogDirTopic>) -> Self {
        Self {
            path: path.as_ref().to_string(),
            topics,
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
        let path = String::read_ext(input, "path", false)?;
        let topics = read_array::<AlterReplicaLogDirTopic>(input, "topics", false)?;
        Ok(AlterReplicaLogDir {
            path, topics
        })
    }
}

impl Writable for AlterReplicaLogDir {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.path.write_ext(output, "self.path", false)?;
        write_array(output, "self.topics", &self.topics, false)?;
        Ok(())
    }
}

/// AlterReplicaLogDirTopic, version 1.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AlterReplicaLogDirTopic {
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The partition indexes.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
}

impl Default for AlterReplicaLogDirTopic {
    fn default() -> Self {
        AlterReplicaLogDirTopic {
            name: String::from(""),
            partitions: Vec::<i32>::new(),
        }
    }
}

impl AlterReplicaLogDirTopic {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: Vec<i32>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
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
        let name = String::read_ext(input, "name", false)?;
        let partitions = read_array::<i32>(input, "partitions", false)?;
        Ok(AlterReplicaLogDirTopic {
            name, partitions
        })
    }
}

impl Writable for AlterReplicaLogDirTopic {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", false)?;
        write_array(output, "self.partitions", &self.partitions, false)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<AlterReplicaLogDirsRequest>("AlterReplicaLogDirsRequest", 1);
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
            crate::test_utils::test_java_arbitrary(&data, "AlterReplicaLogDirsRequest", 1);
        }
    }
}
