//! MSDEV2 protocol types and implementations
//!
//! MSDE is the discovery service, used by MDM clients to bootstrap with the MDM server.
//! The server replies with other service endpoints.

use yaserde::{YaDeserialize, YaSerialize}; // Traits
use yaserde_derive::{YaDeserialize, YaSerialize}; // Proc-macro's

#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "a",
    namespaces = {
        "a" = "http://www.w3.org/2005/08/addressing",
    },
)]
pub struct DiscoverHeader {
    #[yaserde(prefix = "a", rename = "Action")]
    pub action: String,

    #[yaserde(prefix = "a", rename = "MessageID")]
    pub message_id: String,

    #[yaserde(prefix = "a", rename = "ReplyTo")]
    pub reply_to: discover_header::ReplyToType,

    #[yaserde(prefix = "a", rename = "To")]
    pub to: String,
}

// impl Validate for DiscoverHeader {}

pub mod discover_header {
    use super::*;

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "a",
        namespaces = {
            "a" = "http://www.w3.org/2005/08/addressing",
        },
    )]
    pub struct ReplyToType {
        #[yaserde(prefix = "a", rename = "Address")]
        pub address: String,
    }

    // impl Validate for ReplyToType {}
}

#[derive(PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "enroll",
    default_namespace = "enroll",
    namespaces = {
        "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
    },
)]
pub enum AuthPolicyType {
    Certificate,
    Federated,
    OnPremise,
    __Unknown__(String),
}

impl Default for AuthPolicyType {
    fn default() -> AuthPolicyType {
        Self::__Unknown__("No valid variants".into())
    }
}

// impl Validate for AuthPolicyType {}

#[derive(PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "enroll",
    default_namespace = "enroll",
    namespaces = {
        "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
    },
)]
pub enum DeviceType {
    #[yaserde(rename = "CIMClient_Windows")]
    CIMClientWindows,
    WindowsPhone,
    __Unknown__(String),
}

impl Default for DeviceType {
    fn default() -> DeviceType {
        Self::__Unknown__("No valid variants".into())
    }
}

// impl Validate for DeviceType {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    namespaces = {
        "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
    },
)]
pub struct DiscoverRequestBody {
    #[yaserde(prefix = "enroll", rename = "Discover")]
    pub discover: discover::Discover,
}

pub mod discover {
    use super::*;
    use crate::xsd_primitives::Decimal;

    #[derive(Clone, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "enroll",
        default_namespace = "enroll",
        namespaces = {
            "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
        },
    )]
    pub struct Discover {
        #[yaserde(prefix = "enroll", rename = "request")]
        pub request: RequestType,
    }

    // impl Validate for Discover {}

    #[derive(Clone, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "enroll",
        default_namespace = "enroll",
        namespaces = {
            "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
        },
    )]
    pub struct RequestType {
        #[yaserde(prefix = "enroll", rename = "EmailAddress")]
        // NOTE; Not processing the contents, so not parsing string
        // ERROR; No content element could also represent empty string, depends on encoder!
        pub email_address: Option<String>,

        #[yaserde(prefix = "enroll", rename = "RequestVersion")]
        pub request_version: Decimal,

        #[yaserde(prefix = "enroll", rename = "DeviceType")]
        pub device_type: DeviceType,

        #[yaserde(prefix = "enroll", rename = "ApplicationVersion")]
        // NOTE; Not processing the contents, so not parsing string
        pub application_version: String,

        #[yaserde(prefix = "enroll", rename = "OSEdition")]
        pub os_edition: u32,

        #[yaserde(prefix = "enroll", rename = "AuthPolicies")]
        pub auth_policies: request_type::AuthPoliciesType,
    }

    // impl Validate for RequestType {}

    pub mod request_type {
        use super::*;

        #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde(
            prefix = "enroll",
            default_namespace = "enroll",
            namespaces = {
                "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
            },
        )]
        pub struct AuthPoliciesType {
            #[yaserde(prefix = "enroll", rename = "AuthPolicy")]
            pub auth_policy: Vec<AuthPolicyType>,
        }

        // impl Validate for AuthPoliciesType {}
    }
}

#[derive(Debug, PartialEq, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    prefix = "a",
    namespaces = {
        "a" = "http://www.w3.org/2005/08/addressing",
    },
)]
pub struct DiscoverResponseHeader {
    #[yaserde(rename = "Action", prefix = "a")]
    pub action: String,
    #[yaserde(rename = "ActivityId", prefix = "a")]
    // NOTE; Made optional since it's part of Microsoft diagnostics
    pub activity_id: Option<String>,
    #[yaserde(rename = "RelatesTo", prefix = "a")]
    pub relates_to: String,
}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    namespaces = {
        "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
    },
)]
pub struct DiscoverResponseBody {
    #[yaserde(prefix = "enroll", rename = "DiscoverResponse")]
    pub discover: discover_response::DiscoverResponse,
}

pub mod discover_response {
    use super::*;
    use crate::xsd_primitives::Decimal;

    #[derive(Clone, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(
        prefix = "enroll",
        default_namespace = "enroll",
        namespaces = {
            "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
        },
    )]
    pub struct DiscoverResponse {
        #[yaserde(rename = "DiscoverResult", prefix = "enroll")]
        pub response: DiscoverResult,
    }

    #[derive(Debug, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        prefix = "enroll",
        default_namespace = "enroll",
        namespaces = {
            "enroll" = "http://schemas.microsoft.com/windows/management/2012/01/enrollment",
        },
    )]
    pub struct DiscoverResult {
        #[yaserde(rename = "AuthPolicy", prefix = "enroll")]
        pub auth_policy: AuthPolicyType,
        #[yaserde(rename = "EnrollmentPolicyServiceUrl", prefix = "enroll")]
        pub enrollment_policy_service_url: Option<String>,
        #[yaserde(rename = "EnrollmentServiceUrl", prefix = "enroll")]
        pub enrollment_service_url: String,
        #[yaserde(rename = "AuthenticationServiceUrl", prefix = "enroll")]
        pub authentication_service_url: Option<String>,
        #[yaserde(rename = "EnrollmentVersion", prefix = "enroll")]
        pub enrollment_version: Option<Decimal>,
    }
}
