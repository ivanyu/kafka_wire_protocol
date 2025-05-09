// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_nullable_array, write_nullable_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ShareGroupHeartbeatRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ShareGroupHeartbeatRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The member id.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The current member epoch; 0 to join the group; -1 to leave the group.
    pub member_epoch: i32,
    /// null if not provided or if it didn't change since the last heartbeat; the rack ID of consumer otherwise.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack_id: Option<String>,
    /// null if it didn't change since the last heartbeat; the subscribed topic names otherwise.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub subscribed_topic_names: Option<Vec<String>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ShareGroupHeartbeatRequest {
    fn api_key(&self) -> i16 {
        76
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Request for ShareGroupHeartbeatRequest { }

impl Default for ShareGroupHeartbeatRequest {
    fn default() -> Self {
        ShareGroupHeartbeatRequest {
            group_id: String::from(""),
            member_id: String::from(""),
            member_epoch: 0_i32,
            rack_id: None,
            subscribed_topic_names: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ShareGroupHeartbeatRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(group_id: S1, member_id: S2, member_epoch: i32, rack_id: Option<S3>, subscribed_topic_names: Option<Vec<String>>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            member_id: member_id.as_ref().to_string(),
            member_epoch,
            rack_id: rack_id.map(|s| s.as_ref().to_string()),
            subscribed_topic_names,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_share_group_heartbeat_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ShareGroupHeartbeatRequest::new(
            String::from(""),
            String::from(""),
            0_i32,
            None::<String>,
            None::<Vec::<String>>,
        );
        assert_eq!(d, ShareGroupHeartbeatRequest::default());
    }
}

impl Readable for ShareGroupHeartbeatRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let member_id = String::read_ext(input, "member_id", true)?;
        let member_epoch = i32::read(input)?;
        let rack_id = Option::<String>::read_ext(input, "rack_id", true)?;
        let subscribed_topic_names = read_nullable_array::<String>(input, "subscribed_topic_names", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ShareGroupHeartbeatRequest {
            group_id, member_id, member_epoch, rack_id, subscribed_topic_names, _unknown_tagged_fields
        })
    }
}

impl Writable for ShareGroupHeartbeatRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.member_epoch.write(output)?;
        self.rack_id.write_ext(output, "self.rack_id", true)?;
        write_nullable_array(output, "self.subscribed_topic_names", self.subscribed_topic_names.as_deref(), true)?;
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
        crate::test_utils::test_java_default::<ShareGroupHeartbeatRequest>("ShareGroupHeartbeatRequest", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ShareGroupHeartbeatRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ShareGroupHeartbeatRequest) {
            crate::test_utils::test_java_arbitrary(&data, "ShareGroupHeartbeatRequest", 0);
        }
    }
}
