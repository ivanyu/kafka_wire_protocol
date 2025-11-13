// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, read_nullable_array, write_array, write_nullable_array};
use crate::markers::{ApiMessage, Response};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// StreamsGroupDescribeResponse, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct StreamsGroupDescribeResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// Each described group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub groups: Vec<DescribedGroup>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for StreamsGroupDescribeResponse {
    fn api_key(&self) -> i16 {
        89
    }
    
    fn version(&self) -> i16 {
        0
    }
}

impl Response for StreamsGroupDescribeResponse { }

impl Default for StreamsGroupDescribeResponse {
    fn default() -> Self {
        StreamsGroupDescribeResponse {
            throttle_time_ms: 0_i32,
            groups: Vec::<DescribedGroup>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl StreamsGroupDescribeResponse {
    pub fn new(throttle_time_ms: i32, groups: Vec<DescribedGroup>) -> Self {
        Self {
            throttle_time_ms,
            groups,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_streams_group_describe_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = StreamsGroupDescribeResponse::new(
            0_i32,
            Vec::<DescribedGroup>::new(),
        );
        assert_eq!(d, StreamsGroupDescribeResponse::default());
    }
}

impl Readable for StreamsGroupDescribeResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let groups = read_array::<DescribedGroup>(input, "groups", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(StreamsGroupDescribeResponse {
            throttle_time_ms, groups, _unknown_tagged_fields
        })
    }
}

impl Writable for StreamsGroupDescribeResponse {
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
    /// The topology metadata currently initialized for the streams application. Can be null in case of a describe error.
    pub topology: Option<Topology>,
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
            topology: None,
            members: Vec::<Member>::new(),
            authorized_operations: -2147483648_i32,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl DescribedGroup {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(error_code: i16, error_message: Option<S1>, group_id: S2, group_state: S3, group_epoch: i32, assignment_epoch: i32, topology: Option<Topology>, members: Vec<Member>, authorized_operations: i32) -> Self {
        Self {
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            group_id: group_id.as_ref().to_string(),
            group_state: group_state.as_ref().to_string(),
            group_epoch,
            assignment_epoch,
            topology,
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
            None::<Topology>,
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
        let topology = (if i8::read(input)? < 0 { Ok(None) } else { Topology::read(input).map(Some) })?;
        let members = read_array::<Member>(input, "members", true)?;
        let authorized_operations = i32::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(DescribedGroup {
            error_code, error_message, group_id, group_state, group_epoch, assignment_epoch, topology, members, authorized_operations, _unknown_tagged_fields
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
        (if let Some(v) = &self.topology { 1_i8.write(output)?; v.write(output) } else { (-1_i8).write(output) })?;
        write_array(output, "self.members", &self.members, true)?;
        self.authorized_operations.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Topology, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Topology {
    /// The epoch of the currently initialized topology for this group.
    pub epoch: i32,
    /// The subtopologies of the streams application. This contains the configured subtopologies, where the number of partitions are set and any regular expressions are resolved to actual topics. Null if the group is uninitialized, source topics are missing or incorrectly partitioned.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_vec()"))]
    pub subtopologies: Option<Vec<Subtopology>>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Topology {
    fn default() -> Self {
        Topology {
            epoch: 0_i32,
            subtopologies: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Topology {
    pub fn new(epoch: i32, subtopologies: Option<Vec<Subtopology>>) -> Self {
        Self {
            epoch,
            subtopologies,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topology_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Topology::new(
            0_i32,
            None::<Vec::<Subtopology>>,
        );
        assert_eq!(d, Topology::default());
    }
}

impl Readable for Topology {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let epoch = i32::read(input)?;
        let subtopologies = read_nullable_array::<Subtopology>(input, "subtopologies", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Topology {
            epoch, subtopologies, _unknown_tagged_fields
        })
    }
}

impl Writable for Topology {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.epoch.write(output)?;
        write_nullable_array(output, "self.subtopologies", self.subtopologies.as_deref(), true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Subtopology, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Subtopology {
    /// String to uniquely identify the subtopology.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub subtopology_id: String,
    /// The topics the subtopology reads from.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub source_topics: Vec<String>,
    /// The repartition topics the subtopology writes to.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub repartition_sink_topics: Vec<String>,
    /// The set of state changelog topics associated with this subtopology. Created automatically.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub state_changelog_topics: Vec<TopicInfo>,
    /// The set of source topics that are internally created repartition topics. Created automatically.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub repartition_source_topics: Vec<TopicInfo>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Subtopology {
    fn default() -> Self {
        Subtopology {
            subtopology_id: String::from(""),
            source_topics: Vec::<String>::new(),
            repartition_sink_topics: Vec::<String>::new(),
            state_changelog_topics: Vec::<TopicInfo>::new(),
            repartition_source_topics: Vec::<TopicInfo>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Subtopology {
    pub fn new<S1: AsRef<str>>(subtopology_id: S1, source_topics: Vec<String>, repartition_sink_topics: Vec<String>, state_changelog_topics: Vec<TopicInfo>, repartition_source_topics: Vec<TopicInfo>) -> Self {
        Self {
            subtopology_id: subtopology_id.as_ref().to_string(),
            source_topics,
            repartition_sink_topics,
            state_changelog_topics,
            repartition_source_topics,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_subtopology_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Subtopology::new(
            String::from(""),
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<TopicInfo>::new(),
            Vec::<TopicInfo>::new(),
        );
        assert_eq!(d, Subtopology::default());
    }
}

impl Readable for Subtopology {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let subtopology_id = String::read_ext(input, "subtopology_id", true)?;
        let source_topics = read_array::<String>(input, "source_topics", true)?;
        let repartition_sink_topics = read_array::<String>(input, "repartition_sink_topics", true)?;
        let state_changelog_topics = read_array::<TopicInfo>(input, "state_changelog_topics", true)?;
        let repartition_source_topics = read_array::<TopicInfo>(input, "repartition_source_topics", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Subtopology {
            subtopology_id, source_topics, repartition_sink_topics, state_changelog_topics, repartition_source_topics, _unknown_tagged_fields
        })
    }
}

impl Writable for Subtopology {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.subtopology_id.write_ext(output, "self.subtopology_id", true)?;
        write_array(output, "self.source_topics", &self.source_topics, true)?;
        write_array(output, "self.repartition_sink_topics", &self.repartition_sink_topics, true)?;
        write_array(output, "self.state_changelog_topics", &self.state_changelog_topics, true)?;
        write_array(output, "self.repartition_source_topics", &self.repartition_source_topics, true)?;
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
    /// The member epoch.
    pub member_epoch: i32,
    /// The member instance ID for static membership.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub instance_id: Option<String>,
    /// The rack ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub rack_id: Option<String>,
    /// The client ID.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_id: String,
    /// The client host.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub client_host: String,
    /// The epoch of the topology on the client.
    pub topology_epoch: i32,
    /// Identity of the streams instance that may have multiple clients. 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub process_id: String,
    /// User-defined endpoint for Interactive Queries. Null if not defined for this client.
    pub user_endpoint: Option<Endpoint>,
    /// Used for rack-aware assignment algorithm.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub client_tags: Vec<KeyValue>,
    /// Cumulative changelog offsets for tasks.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub task_offsets: Vec<TaskOffset>,
    /// Cumulative changelog end offsets for tasks.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub task_end_offsets: Vec<TaskOffset>,
    /// The current assignment.
    pub assignment: Assignment,
    /// The target assignment.
    pub target_assignment: Assignment,
    /// True for classic members that have not been upgraded yet.
    pub is_classic: bool,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Member {
    fn default() -> Self {
        Member {
            member_id: String::from(""),
            member_epoch: 0_i32,
            instance_id: None,
            rack_id: None,
            client_id: String::from(""),
            client_host: String::from(""),
            topology_epoch: 0_i32,
            process_id: String::from(""),
            user_endpoint: None,
            client_tags: Vec::<KeyValue>::new(),
            task_offsets: Vec::<TaskOffset>::new(),
            task_end_offsets: Vec::<TaskOffset>::new(),
            assignment: Assignment::default(),
            target_assignment: Assignment::default(),
            is_classic: false,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Member {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>, S5: AsRef<str>, S6: AsRef<str>>(member_id: S1, member_epoch: i32, instance_id: Option<S2>, rack_id: Option<S3>, client_id: S4, client_host: S5, topology_epoch: i32, process_id: S6, user_endpoint: Option<Endpoint>, client_tags: Vec<KeyValue>, task_offsets: Vec<TaskOffset>, task_end_offsets: Vec<TaskOffset>, assignment: Assignment, target_assignment: Assignment, is_classic: bool) -> Self {
        Self {
            member_id: member_id.as_ref().to_string(),
            member_epoch,
            instance_id: instance_id.map(|s| s.as_ref().to_string()),
            rack_id: rack_id.map(|s| s.as_ref().to_string()),
            client_id: client_id.as_ref().to_string(),
            client_host: client_host.as_ref().to_string(),
            topology_epoch,
            process_id: process_id.as_ref().to_string(),
            user_endpoint,
            client_tags,
            task_offsets,
            task_end_offsets,
            assignment,
            target_assignment,
            is_classic,
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
            0_i32,
            None::<String>,
            None::<String>,
            String::from(""),
            String::from(""),
            0_i32,
            String::from(""),
            None::<Endpoint>,
            Vec::<KeyValue>::new(),
            Vec::<TaskOffset>::new(),
            Vec::<TaskOffset>::new(),
            Assignment::default(),
            Assignment::default(),
            false,
        );
        assert_eq!(d, Member::default());
    }
}

impl Readable for Member {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let member_id = String::read_ext(input, "member_id", true)?;
        let member_epoch = i32::read(input)?;
        let instance_id = Option::<String>::read_ext(input, "instance_id", true)?;
        let rack_id = Option::<String>::read_ext(input, "rack_id", true)?;
        let client_id = String::read_ext(input, "client_id", true)?;
        let client_host = String::read_ext(input, "client_host", true)?;
        let topology_epoch = i32::read(input)?;
        let process_id = String::read_ext(input, "process_id", true)?;
        let user_endpoint = (if i8::read(input)? < 0 { Ok(None) } else { Endpoint::read(input).map(Some) })?;
        let client_tags = read_array::<KeyValue>(input, "client_tags", true)?;
        let task_offsets = read_array::<TaskOffset>(input, "task_offsets", true)?;
        let task_end_offsets = read_array::<TaskOffset>(input, "task_end_offsets", true)?;
        let assignment = Assignment::read(input)?;
        let target_assignment = Assignment::read(input)?;
        let is_classic = bool::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Member {
            member_id, member_epoch, instance_id, rack_id, client_id, client_host, topology_epoch, process_id, user_endpoint, client_tags, task_offsets, task_end_offsets, assignment, target_assignment, is_classic, _unknown_tagged_fields
        })
    }
}

impl Writable for Member {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.member_epoch.write(output)?;
        self.instance_id.write_ext(output, "self.instance_id", true)?;
        self.rack_id.write_ext(output, "self.rack_id", true)?;
        self.client_id.write_ext(output, "self.client_id", true)?;
        self.client_host.write_ext(output, "self.client_host", true)?;
        self.topology_epoch.write(output)?;
        self.process_id.write_ext(output, "self.process_id", true)?;
        (if let Some(v) = &self.user_endpoint { 1_i8.write(output)?; v.write(output) } else { (-1_i8).write(output) })?;
        write_array(output, "self.client_tags", &self.client_tags, true)?;
        write_array(output, "self.task_offsets", &self.task_offsets, true)?;
        write_array(output, "self.task_end_offsets", &self.task_end_offsets, true)?;
        self.assignment.write(output)?;
        self.target_assignment.write(output)?;
        self.is_classic.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Assignment, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Assignment {
    /// Active tasks for this client.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub active_tasks: Vec<TaskIds>,
    /// Standby tasks for this client.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub standby_tasks: Vec<TaskIds>,
    /// Warm-up tasks for this client. 
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub warmup_tasks: Vec<TaskIds>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Assignment {
    fn default() -> Self {
        Assignment {
            active_tasks: Vec::<TaskIds>::new(),
            standby_tasks: Vec::<TaskIds>::new(),
            warmup_tasks: Vec::<TaskIds>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Assignment {
    pub fn new(active_tasks: Vec<TaskIds>, standby_tasks: Vec<TaskIds>, warmup_tasks: Vec<TaskIds>) -> Self {
        Self {
            active_tasks,
            standby_tasks,
            warmup_tasks,
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
            Vec::<TaskIds>::new(),
            Vec::<TaskIds>::new(),
            Vec::<TaskIds>::new(),
        );
        assert_eq!(d, Assignment::default());
    }
}

impl Readable for Assignment {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let active_tasks = read_array::<TaskIds>(input, "active_tasks", true)?;
        let standby_tasks = read_array::<TaskIds>(input, "standby_tasks", true)?;
        let warmup_tasks = read_array::<TaskIds>(input, "warmup_tasks", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Assignment {
            active_tasks, standby_tasks, warmup_tasks, _unknown_tagged_fields
        })
    }
}

impl Writable for Assignment {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.active_tasks", &self.active_tasks, true)?;
        write_array(output, "self.standby_tasks", &self.standby_tasks, true)?;
        write_array(output, "self.warmup_tasks", &self.warmup_tasks, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// Endpoint, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Endpoint {
    /// host of the endpoint
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// port of the endpoint
    pub port: u16,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint {
            host: String::from(""),
            port: 0_u16,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl Endpoint {
    pub fn new<S1: AsRef<str>>(host: S1, port: u16) -> Self {
        Self {
            host: host.as_ref().to_string(),
            port,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_endpoint_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = Endpoint::new(
            String::from(""),
            0_u16,
        );
        assert_eq!(d, Endpoint::default());
    }
}

impl Readable for Endpoint {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let host = String::read_ext(input, "host", true)?;
        let port = u16::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(Endpoint {
            host, port, _unknown_tagged_fields
        })
    }
}

impl Writable for Endpoint {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.host.write_ext(output, "self.host", true)?;
        self.port.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// KeyValue, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct KeyValue {
    /// key of the config
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub key: String,
    /// value of the config
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub value: String,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for KeyValue {
    fn default() -> Self {
        KeyValue {
            key: String::from(""),
            value: String::from(""),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl KeyValue {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(key: S1, value: S2) -> Self {
        Self {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_string(),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_key_value_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = KeyValue::new(
            String::from(""),
            String::from(""),
        );
        assert_eq!(d, KeyValue::default());
    }
}

impl Readable for KeyValue {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let key = String::read_ext(input, "key", true)?;
        let value = String::read_ext(input, "value", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(KeyValue {
            key, value, _unknown_tagged_fields
        })
    }
}

impl Writable for KeyValue {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.key.write_ext(output, "self.key", true)?;
        self.value.write_ext(output, "self.value", true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TaskIds, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TaskIds {
    /// The subtopology identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub subtopology_id: String,
    /// The partitions of the input topics processed by this member.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub partitions: Vec<i32>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TaskIds {
    fn default() -> Self {
        TaskIds {
            subtopology_id: String::from(""),
            partitions: Vec::<i32>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TaskIds {
    pub fn new<S1: AsRef<str>>(subtopology_id: S1, partitions: Vec<i32>) -> Self {
        Self {
            subtopology_id: subtopology_id.as_ref().to_string(),
            partitions,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_task_ids_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TaskIds::new(
            String::from(""),
            Vec::<i32>::new(),
        );
        assert_eq!(d, TaskIds::default());
    }
}

impl Readable for TaskIds {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let subtopology_id = String::read_ext(input, "subtopology_id", true)?;
        let partitions = read_array::<i32>(input, "partitions", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TaskIds {
            subtopology_id, partitions, _unknown_tagged_fields
        })
    }
}

impl Writable for TaskIds {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.subtopology_id.write_ext(output, "self.subtopology_id", true)?;
        write_array(output, "self.partitions", &self.partitions, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TaskOffset, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TaskOffset {
    /// The subtopology identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub subtopology_id: String,
    /// The partition.
    pub partition: i32,
    /// The offset.
    pub offset: i64,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TaskOffset {
    fn default() -> Self {
        TaskOffset {
            subtopology_id: String::from(""),
            partition: 0_i32,
            offset: 0_i64,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TaskOffset {
    pub fn new<S1: AsRef<str>>(subtopology_id: S1, partition: i32, offset: i64) -> Self {
        Self {
            subtopology_id: subtopology_id.as_ref().to_string(),
            partition,
            offset,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_task_offset_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TaskOffset::new(
            String::from(""),
            0_i32,
            0_i64,
        );
        assert_eq!(d, TaskOffset::default());
    }
}

impl Readable for TaskOffset {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let subtopology_id = String::read_ext(input, "subtopology_id", true)?;
        let partition = i32::read(input)?;
        let offset = i64::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TaskOffset {
            subtopology_id, partition, offset, _unknown_tagged_fields
        })
    }
}

impl Writable for TaskOffset {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.subtopology_id.write_ext(output, "self.subtopology_id", true)?;
        self.partition.write(output)?;
        self.offset.write(output)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// TopicInfo, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct TopicInfo {
    /// The name of the topic.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The number of partitions in the topic. Can be 0 if no specific number of partitions is enforced. Always 0 for changelog topics.
    pub partitions: i32,
    /// The replication factor of the topic. Can be 0 if the default replication factor should be used.
    pub replication_factor: i16,
    /// Topic-level configurations as key-value pairs.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub topic_configs: Vec<KeyValue>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for TopicInfo {
    fn default() -> Self {
        TopicInfo {
            name: String::from(""),
            partitions: 0_i32,
            replication_factor: 0_i16,
            topic_configs: Vec::<KeyValue>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl TopicInfo {
    pub fn new<S1: AsRef<str>>(name: S1, partitions: i32, replication_factor: i16, topic_configs: Vec<KeyValue>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            partitions,
            replication_factor,
            topic_configs,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_topic_info_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = TopicInfo::new(
            String::from(""),
            0_i32,
            0_i16,
            Vec::<KeyValue>::new(),
        );
        assert_eq!(d, TopicInfo::default());
    }
}

impl Readable for TopicInfo {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let partitions = i32::read(input)?;
        let replication_factor = i16::read(input)?;
        let topic_configs = read_array::<KeyValue>(input, "topic_configs", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(TopicInfo {
            name, partitions, replication_factor, topic_configs, _unknown_tagged_fields
        })
    }
}

impl Writable for TopicInfo {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        self.partitions.write(output)?;
        self.replication_factor.write(output)?;
        write_array(output, "self.topic_configs", &self.topic_configs, true)?;
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
        crate::test_utils::test_java_default::<StreamsGroupDescribeResponse>("StreamsGroupDescribeResponse", 0);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: StreamsGroupDescribeResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: StreamsGroupDescribeResponse) {
            crate::test_utils::test_java_arbitrary(&data, "StreamsGroupDescribeResponse", 0);
        }
    }
}
