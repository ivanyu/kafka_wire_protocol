// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// ShareGroupDescribeResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct ShareGroupDescribeResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each described group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<DescribedGroup>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for ShareGroupDescribeResponse {
    fn api_key(&self) -> i16 {
        77
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for ShareGroupDescribeResponse { }

impl Default for ShareGroupDescribeResponse {
    fn default() -> Self {
        ShareGroupDescribeResponse {
            throttle_time_ms: 0_i32,
            groups: Vec::<DescribedGroup>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl ShareGroupDescribeResponse {
    pub fn new(throttle_time_ms: i32, groups: Vec<DescribedGroup>) -> Self {
        Self {
            throttle_time_ms,
            groups,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_share_group_describe_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = ShareGroupDescribeResponse::new(
            0_i32,
            Vec::<DescribedGroup>::new(),
        );
        assert_eq!(d, ShareGroupDescribeResponse::default());
    }
}

impl Readable for ShareGroupDescribeResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let groups = read_array::<DescribedGroup>(input, "groups", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(ShareGroupDescribeResponse {
            throttle_time_ms, groups, _unknown_tagged_fields
        })
    }
}

impl Writable for ShareGroupDescribeResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        write_array(output, "self.groups", &self.groups, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// DescribedGroup, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribedGroup {
    /// The describe error, or 0 if there was no error.
    pub error_code: i16,
    /// The top-level error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// The group ID string.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The group state string, or the empty string.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_state: String,
    /// The group epoch.
    pub group_epoch: i32,
    /// The assignment epoch.
    pub assignment_epoch: i32,
    /// The selected assignor.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub assignor_name: String,
    /// The members.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub members: Vec<Member>,
    /// 32-bit bitfield to represent authorized operations for this group.
    pub authorized_operations: i32,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for DescribedGroup {
    fn default() -> Self {
        DescribedGroup {
            error_code: 0_i16,
            error_message: None,
            group_id: String::from(""),
            group_state: String::from(""),
            group_epoch: 0_i32,
            assignment_epoch: 0_i32,
            assignor_name: String::from(""),
            members: Vec::<Member>::new(),
            authorized_operations: -2147483648_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribedGroup {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>>(error_code: i16, error_message: Option<S1>, group_id: S2, group_state: S3, group_epoch: i32, assignment_epoch: i32, assignor_name: S4, members: Vec<Member>, authorized_operations: i32) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            group_id: group_id.as_ref().to_string(),
            group_state: group_state.as_ref().to_string(),
            group_epoch,
            assignment_epoch,
            assignor_name: assignor_name.as_ref().to_string(),
            members,
            authorized_operations,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_described_group_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribedGroup::new(
            0_i16,
            None::<String>,
            String::from(""),
            String::from(""),
            0_i32,
            0_i32,
            String::from(""),
            Vec::<Member>::new(),
            -2147483648_i32,
        );
        assert_eq!(d, DescribedGroup::default());
    }
}

impl Readable for DescribedGroup {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", true)?;
        let group_id = String::read_ext(input, "group_id", true)?;
        let group_state = String::read_ext(input, "group_state", true)?;
        let group_epoch = i32::read(input)?;
        let assignment_epoch = i32::read(input)?;
        let assignor_name = String::read_ext(input, "assignor_name", true)?;
        let members = read_array::<Member>(input, "members", true)?;
        let authorized_operations = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribedGroup {
            error_code, error_message, group_id, group_state, group_epoch, assignment_epoch, assignor_name, members, authorized_operations, _unknown_tagged_fields
        })
    }
}

impl Writable for DescribedGroup {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", true)?;
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.group_state.write_ext(output, "self.group_state", true)?;
        self.group_epoch.write(output)?;
        self.assignment_epoch.write(output)?;
        self.assignor_name.write_ext(output, "self.assignor_name", true)?;
        write_array(output, "self.members", &self.members, true)?;
        self.authorized_operations.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Member, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Member {
    /// The member ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The member rack ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack_id: Option<String>,
    /// The current member epoch.
    pub member_epoch: i32,
    /// The client ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_id: String,
    /// The client host.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_host: String,
    /// The subscribed topic names.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub subscribed_topic_names: Vec<String>,
    /// The current assignment.
    pub assignment: Assignment,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Member {
    fn default() -> Self {
        Member {
            member_id: String::from(""),
            rack_id: None,
            member_epoch: 0_i32,
            client_id: String::from(""),
            client_host: String::from(""),
            subscribed_topic_names: Vec::<String>::new(),
            assignment: Assignment::default(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Member {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>>(member_id: S1, rack_id: Option<S2>, member_epoch: i32, client_id: S3, client_host: S4, subscribed_topic_names: Vec<String>, assignment: Assignment) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            rack_id: rack_id.map(|s| s.as_ref().to_string()),
            member_epoch,
            client_id: client_id.as_ref().to_string(),
            client_host: client_host.as_ref().to_string(),
            subscribed_topic_names,
            assignment,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_member_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Member::new(
            String::from(""),
            None::<String>,
            0_i32,
            String::from(""),
            String::from(""),
            Vec::<String>::new(),
            Assignment::default(),
        );
        assert_eq!(d, Member::default());
    }
}

impl Readable for Member {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", true)?;
        let rack_id = Option::<String>::read_ext(input, "rack_id", true)?;
        let member_epoch = i32::read(input)?;
        let client_id = String::read_ext(input, "client_id", true)?;
        let client_host = String::read_ext(input, "client_host", true)?;
        let subscribed_topic_names = read_array::<String>(input, "subscribed_topic_names", true)?;
        let assignment = Assignment::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Member {
            member_id, rack_id, member_epoch, client_id, client_host, subscribed_topic_names, assignment, _unknown_tagged_fields
        })
    }
}

impl Writable for Member {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.rack_id.write_ext(output, "self.rack_id", true)?;
        self.member_epoch.write(output)?;
        self.client_id.write_ext(output, "self.client_id", true)?;
        self.client_host.write_ext(output, "self.client_host", true)?;
        write_array(output, "self.subscribed_topic_names", &self.subscribed_topic_names, true)?;
        self.assignment.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Assignment, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Assignment {
    /// The assigned topic-partitions to the member.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topic_partitions: Vec<TopicPartitions>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Assignment {
    fn default() -> Self {
        Assignment {
            topic_partitions: Vec::<TopicPartitions>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Assignment {
    pub fn new(topic_partitions: Vec<TopicPartitions>) -> Self {
        Self {
            topic_partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_assignment_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Assignment::new(
            Vec::<TopicPartitions>::new(),
        );
        assert_eq!(d, Assignment::default());
    }
}

impl Readable for Assignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_partitions = read_array::<TopicPartitions>(input, "topic_partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Assignment {
            topic_partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for Assignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.topic_partitions", &self.topic_partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicPartitions, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicPartitions {
    /// The topic ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::uuid()"))]
    pub topic_id: Uuid,
    /// The topic name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub topic_name: String,
    /// The partitions.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicPartitions {
    fn default() -> Self {
        TopicPartitions {
            topic_id: Uuid::nil(),
            topic_name: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicPartitions {
    pub fn new<S1: AsRef<str>>(topic_id: Uuid, topic_name: S1, partitions: Vec<i32>) -> Self {
        Self {
            topic_id,
            topic_name: topic_name.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_partitions_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicPartitions::new(
            Uuid::nil(),
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, TopicPartitions::default());
    }
}

impl Readable for TopicPartitions {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let topic_id = Uuid::read(input)?;
        let topic_name = String::read_ext(input, "topic_name", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicPartitions {
            topic_id, topic_name, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicPartitions {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.topic_id.write(output)?;
        self.topic_name.write_ext(output, "self.topic_name", true)?;
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
        crate::test_utils::test_java_default::<ShareGroupDescribeResponse>("ShareGroupDescribeResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: ShareGroupDescribeResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: ShareGroupDescribeResponse) {
            crate::test_utils::test_java_arbitrary(&data, "ShareGroupDescribeResponse", 0);
        }
    }
}
