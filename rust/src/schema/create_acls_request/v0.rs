// This file was generated. Do not edit.

use std::io::{Read, Result, Write};

use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;

use crate::arrays::{read_array, write_array};
use crate::markers::{ApiMessage, Request};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;

/// CreateAclsRequest, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct CreateAclsRequest {
    /// The ACLs that we want to create.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub creations: Vec<AclCreation>,
}

impl ApiMessage for CreateAclsRequest {
    #[cfg(not(tarpaulin_include))]
    fn api_key(&self) -> i16 {
        30
    }
    
    #[cfg(not(tarpaulin_include))]
    fn version(&self) -> i16 {
        0
    }
}

impl Request for CreateAclsRequest { }

impl Default for CreateAclsRequest {
    fn default() -> Self {
        CreateAclsRequest {
            creations: Vec::<AclCreation>::new(),
        }
    }
}

impl CreateAclsRequest {
    pub fn new(creations: Vec<AclCreation>) -> Self {
        Self {
            creations,
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
        let creations = read_array::<AclCreation>(input, "creations", false)?;
        Ok(CreateAclsRequest {
            creations
        })
    }
}

impl Writable for CreateAclsRequest {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        write_array(output, "self.creations", &self.creations, false)?;
        Ok(())
    }
}

/// AclCreation, version 0.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AclCreation {
    /// The type of the resource.
    pub resource_type: i8,
    /// The resource name for the ACL.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
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
}

impl Default for AclCreation {
    fn default() -> Self {
        AclCreation {
            resource_type: 0_i8,
            resource_name: String::from(""),
            principal: String::from(""),
            host: String::from(""),
            operation: 0_i8,
            permission_type: 0_i8,
        }
    }
}

impl AclCreation {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(resource_type: i8, resource_name: S1, principal: S2, host: S3, operation: i8, permission_type: i8) -> Self {
        Self {
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
            principal: principal.as_ref().to_string(),
            host: host.as_ref().to_string(),
            operation,
            permission_type,
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
        let resource_name = String::read_ext(input, "resource_name", false)?;
        let principal = String::read_ext(input, "principal", false)?;
        let host = String::read_ext(input, "host", false)?;
        let operation = i8::read(input)?;
        let permission_type = i8::read(input)?;
        Ok(AclCreation {
            resource_type, resource_name, principal, host, operation, permission_type
        })
    }
}

impl Writable for AclCreation {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", false)?;
        self.principal.write_ext(output, "self.principal", false)?;
        self.host.write_ext(output, "self.host", false)?;
        self.operation.write(output)?;
        self.permission_type.write(output)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_java_default() {
        crate::test_utils::test_java_default::<CreateAclsRequest>("CreateAclsRequest", 0);
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
            crate::test_utils::test_java_arbitrary(&data, "CreateAclsRequest", 0);
        }
    }
}
