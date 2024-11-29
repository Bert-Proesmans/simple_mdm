use yaserde::{YaDeserialize, YaSerialize}; // Traits
use yaserde_derive::{YaDeserialize, YaSerialize}; // Proc-macro's

#[derive(YaSerialize, YaDeserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct DefaultHeader {}

#[derive(YaSerialize, YaDeserialize, Debug, Default, Clone, Eq, PartialEq)]
#[yaserde(
    prefix = "s",
    rename = "Envelope",
    namespaces = {
        "s" = "http://www.w3.org/2003/05/soap-envelope",
        "a" = "http://www.w3.org/2005/08/addressing",
    },
)]
pub struct SoapEnvelope<TBODY, THEADER = DefaultHeader>
where
    THEADER: YaSerialize + YaDeserialize,
    TBODY: YaSerialize + YaDeserialize,
{
    #[yaserde(rename = "encodingStyle", prefix = "s", attribute = true)]
    pub encoding_style: Option<String>,
    #[yaserde(rename = "u", prefix = "xmlns", attribute = true)]
    pub tnsattr: Option<String>,
    #[yaserde(rename = "urn", prefix = "xmlns", attribute = true)]
    pub urnattr: Option<String>,
    #[yaserde(rename = "xsi", prefix = "xmlns", attribute = true)]
    pub xsiattr: Option<String>,
    #[yaserde(rename = "Header", prefix = "s")]
    pub header: THEADER,
    #[yaserde(rename = "Body", prefix = "s")]
    pub body: TBODY,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespaces = {
        "s" = "http://www.w3.org/2003/05/soap-envelope",
    },
    prefix = "s"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode")]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring")]
    pub fault_string: Option<String>,
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}

// pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;
