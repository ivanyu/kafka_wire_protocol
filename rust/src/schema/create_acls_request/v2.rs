// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
use crate::tagged_fields::{RawTaggedField, read_tagged_fields, write_tagged_fields};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// CreateAclsRequest, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateAclsRequest {
    /// The ACLs that we want to create.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub creations: Vec<AclCreation>,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl ApiMessage for CreateAclsRequest {
    fn api_key(&self) -> i16 {
        30
    }
    
    fn version(&self) -> i16 {
        2
    }
}

impl Request for CreateAclsRequest { }

impl Default for CreateAclsRequest {
    fn default() -> Self {
        CreateAclsRequest {
            creations: Vec::<AclCreation>::new(),
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl CreateAclsRequest {
    pub fn new(creations: Vec<AclCreation>) -> Self {
        Self {
            creations,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_create_acls_request_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = CreateAclsRequest::new(
            Vec::<AclCreation>::new(),
        );
        assert_eq!(d, CreateAclsRequest::default());
    }
}

impl Readable for CreateAclsRequest {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let creations = read_array::<AclCreation>(input, "creations", true)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(CreateAclsRequest {
            creations, _unknown_tagged_fields
        })
    }
}

impl Writable for CreateAclsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.creations", &self.creations, true)?;
        write_tagged_fields(output, &[], &self._unknown_tagged_fields)?;
        Ok(())
    }
}

/// AclCreation, version 2.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AclCreation {
    /// The type of the resource.
    pub resource_type: i8,
    /// The resource name for the ACL.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
    /// The pattern type for the ACL.
    pub resource_pattern_type: i8,
    /// The principal for the ACL.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal: String,
    /// The host for the ACL.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The operation type for the ACL (read, write, etc.).
    pub operation: i8,
    /// The permission type for the ACL (allow, deny, etc.).
    pub permission_type: i8,
    /// Unknown tagged fields.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::unknown_tagged_fields()"))]
    pub _unknown_tagged_fields: Vec<RawTaggedField>,
}

impl Default for AclCreation {
    fn default() -> Self {
        AclCreation {
            resource_type: 0_i8,
            resource_name: String::from(""),
            resource_pattern_type: 3_i8,
            principal: String::from(""),
            host: String::from(""),
            operation: 0_i8,
            permission_type: 0_i8,
            _unknown_tagged_fields: Vec::new(),
        }
    }
}

impl AclCreation {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(resource_type: i8, resource_name: S1, resource_pattern_type: i8, principal: S2, host: S3, operation: i8, permission_type: i8) -> Self {
        Self {
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
            resource_pattern_type,
            principal: principal.as_ref().to_string(),
            host: host.as_ref().to_string(),
            operation,
            permission_type,
            _unknown_tagged_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests_acl_creation_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AclCreation::new(
            0_i8,
            String::from(""),
            3_i8,
            String::from(""),
            String::from(""),
            0_i8,
            0_i8,
        );
        assert_eq!(d, AclCreation::default());
    }
}

impl Readable for AclCreation {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", true)?;
        let resource_pattern_type = i8::read(input)?;
        let principal = String::read_ext(input, "principal", true)?;
        let host = String::read_ext(input, "host", true)?;
        let operation = i8::read(input)?;
        let permission_type = i8::read(input)?;
        let tagged_fields_callback = |tag: i32, _: &[u8]| {
            match tag {
                _ => Ok(false)
            }
        };
        let _unknown_tagged_fields = read_tagged_fields(input, tagged_fields_callback)?;
        Ok(AclCreation {
            resource_type, resource_name, resource_pattern_type, principal, host, operation, permission_type, _unknown_tagged_fields
        })
    }
}

impl Writable for AclCreation {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", true)?;
        self.resource_pattern_type.write(output)?;
        self.principal.write_ext(output, "self.principal", true)?;
        self.host.write_ext(output, "self.host", true)?;
        self.operation.write(output)?;
        self.permission_type.write(output)?;
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
        crate::test_utils::test_java_default::<CreateAclsRequest>("CreateAclsRequest", 2);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: CreateAclsRequest) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: CreateAclsRequest) {
            crate::test_utils::test_java_arbitrary(&data, "CreateAclsRequest", 2);
        }
    }
}
