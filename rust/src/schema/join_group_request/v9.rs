// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::bytes::{read_bytes, write_bytes};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::{proptest_strategies, serde_bytes};

/// JoinGroupRequest, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct JoinGroupRequest {
    /// The group identifier.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub group_id: String,
    /// The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds.
    pub session_timeout_ms: i32,
    /// The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group.
    pub rebalance_timeout_ms: i32,
    /// The member id assigned by the group coordinator.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub member_id: String,
    /// The unique identifier of the consumer instance provided by end user.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub group_instance_id: Option<String>,
    /// The unique name the for class of protocols implemented by the group we want to join.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub protocol_type: String,
    /// The list of protocols that the member supports.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub protocols: Vec<JoinGroupRequestProtocol>,
    /// The reason why the member (re-)joins the group.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub reason: Option<String>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for JoinGroupRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        11
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        9
    }
}

impl Request for JoinGroupRequest { }

impl Default for JoinGroupRequest {
    fn default() -> Self {
        JoinGroupRequest {
            group_id: String::from(""),
            session_timeout_ms: 0_i32,
            rebalance_timeout_ms: -1_i32,
            member_id: String::from(""),
            group_instance_id: None,
            protocol_type: String::from(""),
            protocols: Vec::<JoinGroupRequestProtocol>::new(),
            reason: None,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl JoinGroupRequest {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>, S4: AsRef<str>, S5: AsRef<str>>(group_id: S1, session_timeout_ms: i32, rebalance_timeout_ms: i32, member_id: S2, group_instance_id: Option<S3>, protocol_type: S4, protocols: Vec<JoinGroupRequestProtocol>, reason: Option<S5>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            session_timeout_ms,
            rebalance_timeout_ms,
            member_id: member_id.as_ref().to_string(),
            group_instance_id: group_instance_id.map(|s| s.as_ref().to_string()),
            protocol_type: protocol_type.as_ref().to_string(),
            protocols,
            reason: reason.map(|s| s.as_ref().to_string()),
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_join_group_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = JoinGroupRequest::new(
            String::from(""),
            0_i32,
            -1_i32,
            String::from(""),
            None::<String>,
            String::from(""),
            Vec::<JoinGroupRequestProtocol>::new(),
            None::<String>,
        );
        assert_eq!(d, JoinGroupRequest::default());
    }
}

impl Readable for JoinGroupRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let group_id = String::read_ext(input, "group_id", true)?;
        let session_timeout_ms = i32::read(input)?;
        let rebalance_timeout_ms = i32::read(input)?;
        let member_id = String::read_ext(input, "member_id", true)?;
        let group_instance_id = Option::<String>::read_ext(input, "group_instance_id", true)?;
        let protocol_type = String::read_ext(input, "protocol_type", true)?;
        let protocols = read_array::<JoinGroupRequestProtocol>(input, "protocols", true)?;
        let reason = Option::<String>::read_ext(input, "reason", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(JoinGroupRequest {
            group_id, session_timeout_ms, rebalance_timeout_ms, member_id, group_instance_id, protocol_type, protocols, reason, _unknown_tagged_fields
        })
    }
}

impl Writable for JoinGroupRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.group_id.write_ext(output, "self.group_id", true)?;
        self.session_timeout_ms.write(output)?;
        self.rebalance_timeout_ms.write(output)?;
        self.member_id.write_ext(output, "self.member_id", true)?;
        self.group_instance_id.write_ext(output, "self.group_instance_id", true)?;
        self.protocol_type.write_ext(output, "self.protocol_type", true)?;
        write_array(output, "self.protocols", &self.protocols, true)?;
        self.reason.write_ext(output, "self.reason", true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// JoinGroupRequestProtocol, version 9.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct JoinGroupRequestProtocol {
    /// The protocol name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub name: String,
    /// The protocol metadata.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::bytes()"))]
    #[cfg_attr(test, serde(with="serde_bytes"))]
    pub metadata: Vec<u8>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for JoinGroupRequestProtocol {
    fn default() -> Self {
        JoinGroupRequestProtocol {
            name: String::from(""),
            metadata: Vec::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl JoinGroupRequestProtocol {
    pub fn new<S1: AsRef<str>>(name: S1, metadata: Vec<u8>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            metadata,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_join_group_request_protocol_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = JoinGroupRequestProtocol::new(
            String::from(""),
            Vec::new(),
        );
        assert_eq!(d, JoinGroupRequestProtocol::default());
    }
}

impl Readable for JoinGroupRequestProtocol {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let name = String::read_ext(input, "name", true)?;
        let metadata = read_bytes(input, "metadata", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(JoinGroupRequestProtocol {
            name, metadata, _unknown_tagged_fields
        })
    }
}

impl Writable for JoinGroupRequestProtocol {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.name.write_ext(output, "self.name", true)?;
        write_bytes(output, "self.metadata", &self.metadata, true)?;
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
        crate::test_utils::test_java_default::<JoinGroupRequest>("JoinGroupRequest", 9);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: JoinGroupRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: JoinGroupRequest) {
            crate::test_utils::test_java_arbitrary(&data, "JoinGroupRequest", 9);
        }
    }
}
