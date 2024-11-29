//! XCEP protocol types and implementations
//!
//! XCEP, sometimes referred to as WS-Trust or Certificate Enrollment Web Services when discussing Windows environments, is a protocol for requesting and provisioning digital certificates for client devices through a certificate server.
//!
//! Not to be confused with SCEP, which is a platform independent standard for doing the same over an HTTP api.

use yaserde::{YaDeserialize, YaSerialize}; // Traits
use yaserde_derive::{YaDeserialize, YaSerialize}; // Proc-macro's

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct GetPoliciesRequestBody {
    #[yaserde(prefix = "xcep", rename = "GetPolicies")]
    pub get_policies: GetPolicies,
}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct GetPolicies {
    #[yaserde(prefix = "xcep", rename = "client")]
    pub client: Client,
    // #[yaserde(prefix = "xcep", rename = "requestFilter")]
    // pub request_filter: Option<RequestFilter>,
}

// impl Validate for GetPolicies {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "xcep",
default_namespace = "xcep",
namespaces = {
    "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
}
)]
pub struct GetPoliciesResponse {
    #[yaserde(prefix = "xcep", rename = "response")]
    pub response: Option<Response>,

    #[yaserde(prefix = "xcep", rename = "cAs")]
    pub c_as: Option<Cacollection>,

    #[yaserde(prefix = "xcep", rename = "oIDs")]
    pub o_i_ds: Option<Oidcollection>,
}

// impl Validate for GetPoliciesResponse {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
default_namespace = "xcep",
namespaces = {
    "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
})]
pub struct Attributes {
    #[yaserde(prefix = "xcep", rename = "commonName")]
    //pub common_name: CommonName,
    pub common_name: String,

    #[yaserde(prefix = "xcep", rename = "policySchema")]
    pub policy_schema: u32,

    #[yaserde(prefix = "xcep", rename = "certificateValidity")]
    pub certificate_validity: CertificateValidity,

    #[yaserde(prefix = "xcep", rename = "permission")]
    pub permission: EnrollmentPermission,

    #[yaserde(prefix = "xcep", rename = "privateKeyAttributes")]
    pub private_key_attributes: Vec<PrivateKeyAttributes>,

    #[yaserde(prefix = "xcep", rename = "revision")]
    pub revision: Revision,

    #[yaserde(prefix = "xcep", rename = "supersededPolicies")]
    pub superseded_policies: Option<SupersededPolicies>,

    #[yaserde(prefix = "xcep", rename = "privateKeyFlags")]
    pub private_key_flags: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "subjectNameFlags")]
    pub subject_name_flags: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "enrollmentFlags")]
    pub enrollment_flags: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "generalFlags")]
    pub general_flags: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "hashAlgorithmOIDReference")]
    pub hash_algorithm_oid_reference: Option<i32>,

    #[yaserde(prefix = "xcep", rename = "rARequirements")]
    pub r_a_requirements: Option<Rarequirements>,

    #[yaserde(prefix = "xcep", rename = "keyArchivalAttributes")]
    pub key_archival_attributes: Vec<KeyArchivalAttributes>,

    #[yaserde(prefix = "xcep", rename = "extensions")]
    pub extensions: Option<ExtensionCollection>,
}

// impl Validate for Attributes {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "xcep",
default_namespace = "xcep",
namespaces = {
    "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
},)]
pub struct Ca {
    #[yaserde(prefix = "xcep", rename = "uris")]
    pub uris: Cauricollection,

    #[yaserde(prefix = "xcep", rename = "certificate")]
    pub certificate: String,

    #[yaserde(prefix = "xcep", rename = "enrollPermission")]
    pub enroll_permission: bool,

    #[yaserde(prefix = "xcep", rename = "cAReferenceID")]
    pub c_a_reference_id: i32,
}

// impl Validate for Ca {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "xcep",
default_namespace = "xcep",
namespaces = {
    "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
},)]
pub struct Cacollection {
    #[yaserde(prefix = "xcep", rename = "cA")]
    pub c_a: Vec<Ca>,
}

// impl Validate for Cacollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "xcep",
default_namespace = "xcep",
namespaces = {
    "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
},)]
pub struct CareferenceCollection {
    #[yaserde(prefix = "xcep", rename = "cAReference")]
    pub c_a_reference: Vec<i32>,
}

// impl Validate for CareferenceCollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Cauri {
    #[yaserde(prefix = "xcep", rename = "clientAuthentication")]
    pub client_authentication: u32,

    #[yaserde(prefix = "xcep", rename = "uri")]
    pub uri: String,

    #[yaserde(prefix = "xcep", rename = "priority")]
    pub priority: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "renewalOnly")]
    pub renewal_only: bool,
}

// impl Validate for Cauri {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Cauricollection {
    #[yaserde(prefix = "xcep", rename = "cAURI")]
    pub c_auri: Vec<Cauri>,
}

// impl Validate for Cauricollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct CertificateEnrollmentPolicy {
    #[yaserde(prefix = "xcep", rename = "policyOIDReference")]
    pub policy_oid_reference: i32,

    #[yaserde(prefix = "xcep", rename = "cAs")]
    pub c_as: Option<CareferenceCollection>,

    #[yaserde(prefix = "xcep", rename = "attributes")]
    pub attributes: Attributes,
}

// impl Validate for CertificateEnrollmentPolicy {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct CertificateValidity {
    #[yaserde(prefix = "xcep", rename = "validityPeriodSeconds")]
    pub validity_period_seconds: u64,

    #[yaserde(prefix = "xcep", rename = "renewalPeriodSeconds")]
    pub renewal_period_seconds: u64,
}

// impl Validate for CertificateValidity {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        // ERROR; The standards forbid referring to the XSI namespace, hmm
        "xsi" = "http://www.w3.org/2001/XMLSchema-instance",
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct ClientLastUpdate {
    #[yaserde(prefix = "xsi", rename = "nil", attribute = true)]
    pub is_null: Option<bool>,

    #[yaserde(prefix = "xcep", text = true)]
    pub value: Option<crate::xsd_primitives::DateTime>,
    //pub value: Option<String>,
}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Client {
    #[yaserde(prefix = "xcep", rename = "lastUpdate")]
    pub client_last_update: ClientLastUpdate,
    // #[yaserde(prefix = "xcep", rename = "preferredLanguage")]
    // pub preferred_language: Option<String>,
}

// impl Validate for Client {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct CryptoProviders {
    #[yaserde(prefix = "xcep", rename = "provider")]
    pub provider: Vec<String>,
}

// impl Validate for CryptoProviders {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct EnrollmentPermission {
    #[yaserde(prefix = "xcep", rename = "enroll")]
    pub enroll: bool,

    #[yaserde(prefix = "xcep", rename = "autoEnroll")]
    pub auto_enroll: bool,
}

// impl Validate for EnrollmentPermission {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Extension {
    #[yaserde(prefix = "xcep", rename = "oIDReference")]
    pub o_id_reference: i32,

    #[yaserde(prefix = "xcep", rename = "critical")]
    pub critical: bool,

    #[yaserde(prefix = "xcep", rename = "value")]
    pub value: Option<String>,
}

// impl Validate for Extension {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct ExtensionCollection {
    #[yaserde(prefix = "xcep", rename = "extension")]
    pub extension: Vec<Extension>,
}

// impl Validate for ExtensionCollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct FilterOIDCollection {
    #[yaserde(prefix = "xcep", rename = "oid")]
    pub oid: Vec<String>,
}

// impl Validate for FilterOIDCollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct KeyArchivalAttributes {
    #[yaserde(prefix = "xcep", rename = "symmetricAlgorithmOIDReference")]
    pub symmetric_algorithm_oid_reference: i32,

    #[yaserde(prefix = "xcep", rename = "symmetricAlgorithmKeyLength")]
    pub symmetric_algorithm_key_length: u32,
}

// impl Validate for KeyArchivalAttributes {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Oid {
    #[yaserde(prefix = "xcep", rename = "value")]
    pub value: String,

    #[yaserde(prefix = "xcep", rename = "group")]
    pub group: u32,

    #[yaserde(prefix = "xcep", rename = "oIDReferenceID")]
    pub o_id_reference_id: i32,

    #[yaserde(prefix = "xcep", rename = "defaultName")]
    pub default_name: Option<String>,
}

// impl Validate for Oid {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Oidcollection {
    #[yaserde(prefix = "xcep", rename = "oID")]
    pub o_id: Vec<Oid>,
}

// impl Validate for Oidcollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct OidreferenceCollection {
    #[yaserde(prefix = "xcep", rename = "oIDReference")]
    pub o_id_reference: Vec<i32>,
}

// impl Validate for OidreferenceCollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct PolicyCollection {
    #[yaserde(prefix = "xcep", rename = "policy")]
    pub policy: Vec<CertificateEnrollmentPolicy>,
}

// impl Validate for PolicyCollection {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct PrivateKeyAttributes {
    #[yaserde(prefix = "xcep", rename = "minimalKeyLength")]
    pub minimal_key_length: u32,

    #[yaserde(prefix = "xcep", rename = "keySpec")]
    pub key_spec: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "keyUsageProperty")]
    pub key_usage_property: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "permissions")]
    pub permissions: Option<String>,

    #[yaserde(prefix = "xcep", rename = "algorithmOIDReference")]
    pub algorithm_oid_reference: Option<i32>,

    #[yaserde(prefix = "xcep", rename = "cryptoProviders")]
    pub crypto_providers: Option<CryptoProviders>,
}

// impl Validate for PrivateKeyAttributes {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Rarequirements {
    #[yaserde(prefix = "xcep", rename = "rASignatures")]
    pub r_a_signatures: u32,

    #[yaserde(prefix = "xcep", rename = "rAEKUs")]
    pub r_aek_us: Option<OidreferenceCollection>,

    #[yaserde(prefix = "xcep", rename = "rAPolicies")]
    pub r_a_policies: Option<OidreferenceCollection>,
}

// impl Validate for Rarequirements {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct RequestFilter {
    #[yaserde(prefix = "xcep", rename = "policyOIDs")]
    pub policy_oi_ds: Option<FilterOIDCollection>,
}

// impl Validate for RequestFilter {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Response {
    #[yaserde(prefix = "xcep", rename = "policyID")]
    pub policy_id: String,

    #[yaserde(prefix = "xcep", rename = "policyFriendlyName")]
    pub policy_friendly_name: Option<String>,

    #[yaserde(prefix = "xcep", rename = "nextUpdateHours")]
    pub next_update_hours: Option<u32>,

    #[yaserde(prefix = "xcep", rename = "policiesNotChanged")]
    pub policies_not_changed: Option<bool>,

    #[yaserde(prefix = "xcep", rename = "policies")]
    pub policies: Option<PolicyCollection>,
}

// impl Validate for Response {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct Revision {
    #[yaserde(prefix = "xcep", rename = "majorRevision")]
    pub major_revision: u32,

    #[yaserde(prefix = "xcep", rename = "minorRevision")]
    pub minor_revision: u32,
}

// impl Validate for Revision {}

#[derive(Clone, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "xcep",
    default_namespace = "xcep",
    namespaces = {
        "xcep" = "http://schemas.microsoft.com/windows/pki/2009/01/enrollmentpolicy",
    },
)]
pub struct SupersededPolicies {
    #[yaserde(prefix = "xcep", rename = "commonName")]
    //pub common_name: Vec<CommonName>,
    pub common_name: Vec<String>,
}

// impl Validate for SupersededPolicies {}
