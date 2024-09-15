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
pub struct DescribeAclsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
    /// The error code, or 0 if there was no error.
    pub error_code: i16,
    /// The error message, or null if there was no error.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::optional_string()"))]
    pub error_message: Option<String>,
    /// Each Resource that is referenced in an ACL.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub resources: Vec<DescribeAclsResource>,
}

impl ApiMessage for DescribeAclsResponse {
    fn api_key(&self) -> i16 {
        29
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for DescribeAclsResponse { }

impl Default for DescribeAclsResponse {
    fn default() -> Self {
        DescribeAclsResponse {
            throttle_time_ms: 0_i32,
            error_code: 0_i16,
            error_message: Some(String::from("")),
            resources: Vec::<DescribeAclsResource>::new(),
        }
    }
}

impl DescribeAclsResponse {
    pub fn new<S1: AsRef<str>>(throttle_time_ms: i32, error_code: i16, error_message: Option<S1>, resources: Vec<DescribeAclsResource>) -> Self {
        Self {
            throttle_time_ms,
            error_code,
            error_message: error_message.map(|s| s.as_ref().to_string()),
            resources,
        }
    }
}

#[cfg(test)]
mod tests_describe_acls_response_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeAclsResponse::new(
            0_i32,
            0_i16,
            Some(String::from("")),
            Vec::<DescribeAclsResource>::new(),
        );
        assert_eq!(d, DescribeAclsResponse::default());
    }
}

impl Readable for DescribeAclsResponse {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let throttle_time_ms = i32::read(input)?;
        let error_code = i16::read(input)?;
        let error_message = Option::<String>::read_ext(input, "error_message", false)?;
        let resources = read_array::<DescribeAclsResource>(input, "resources", false)?;
        Ok(DescribeAclsResponse {
            throttle_time_ms, error_code, error_message, resources
        })
    }
}

impl Writable for DescribeAclsResponse {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.throttle_time_ms.write(output)?;
        self.error_code.write(output)?;
        self.error_message.write_ext(output, "self.error_message", false)?;
        write_array(output, "self.resources", &self.resources, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct DescribeAclsResource {
    /// The resource type.
    pub resource_type: i8,
    /// The resource name.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub resource_name: String,
    /// The resource pattern type.
    pub pattern_type: i8,
    /// The ACLs.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    pub acls: Vec<AclDescription>,
}

impl ApiMessage for DescribeAclsResource {
    fn api_key(&self) -> i16 {
        29
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for DescribeAclsResource { }

impl Default for DescribeAclsResource {
    fn default() -> Self {
        DescribeAclsResource {
            resource_type: 0_i8,
            resource_name: String::from(""),
            pattern_type: 3_i8,
            acls: Vec::<AclDescription>::new(),
        }
    }
}

impl DescribeAclsResource {
    pub fn new<S1: AsRef<str>>(resource_type: i8, resource_name: S1, pattern_type: i8, acls: Vec<AclDescription>) -> Self {
        Self {
            resource_type,
            resource_name: resource_name.as_ref().to_string(),
            pattern_type,
            acls,
        }
    }
}

#[cfg(test)]
mod tests_describe_acls_resource_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = DescribeAclsResource::new(
            0_i8,
            String::from(""),
            3_i8,
            Vec::<AclDescription>::new(),
        );
        assert_eq!(d, DescribeAclsResource::default());
    }
}

impl Readable for DescribeAclsResource {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let resource_type = i8::read(input)?;
        let resource_name = String::read_ext(input, "resource_name", false)?;
        let pattern_type = i8::read(input)?;
        let acls = read_array::<AclDescription>(input, "acls", false)?;
        Ok(DescribeAclsResource {
            resource_type, resource_name, pattern_type, acls
        })
    }
}

impl Writable for DescribeAclsResource {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
        self.resource_type.write(output)?;
        self.resource_name.write_ext(output, "self.resource_name", false)?;
        self.pattern_type.write(output)?;
        write_array(output, "self.acls", &self.acls, false)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct AclDescription {
    /// The ACL principal.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub principal: String,
    /// The ACL host.
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::string()"))]
    pub host: String,
    /// The ACL operation.
    pub operation: i8,
    /// The ACL permission type.
    pub permission_type: i8,
}

impl ApiMessage for AclDescription {
    fn api_key(&self) -> i16 {
        29
    }
    
    fn version(&self) -> i16 {
        1
    }
}

impl Response for AclDescription { }

impl Default for AclDescription {
    fn default() -> Self {
        AclDescription {
            principal: String::from(""),
            host: String::from(""),
            operation: 0_i8,
            permission_type: 0_i8,
        }
    }
}

impl AclDescription {
    pub fn new<S1: AsRef<str>, S2: AsRef<str>>(principal: S1, host: S2, operation: i8, permission_type: i8) -> Self {
        Self {
            principal: principal.as_ref().to_string(),
            host: host.as_ref().to_string(),
            operation,
            permission_type,
        }
    }
}

#[cfg(test)]
mod tests_acl_description_new_and_default {
    use super::*;
    
    #[test]
    fn test() {
        let d = AclDescription::new(
            String::from(""),
            String::from(""),
            0_i8,
            0_i8,
        );
        assert_eq!(d, AclDescription::default());
    }
}

impl Readable for AclDescription {
    fn read(#[allow(unused)] input: &mut impl Read) -> Result<Self> {
        let principal = String::read_ext(input, "principal", false)?;
        let host = String::read_ext(input, "host", false)?;
        let operation = i8::read(input)?;
        let permission_type = i8::read(input)?;
        Ok(AclDescription {
            principal, host, operation, permission_type
        })
    }
}

impl Writable for AclDescription {
    fn write(&self, #[allow(unused)] output: &mut impl Write) -> Result<()> {
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
        crate::test_utils::test_java_default::<DescribeAclsResponse>("DescribeAclsResponse", 1);
    }
    
    proptest! {
        #[test]
        fn test_serde(data: DescribeAclsResponse) {
            crate::test_utils::test_serde(&data)?;
        }
    }
    
    proptest! {
        #[test]
        fn test_java_arbitrary(data: DescribeAclsResponse) {
            crate::test_utils::test_java_arbitrary(&data, "DescribeAclsResponse", 1);
        }
    }
}
