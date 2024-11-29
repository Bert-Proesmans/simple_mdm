//! Copied parts directly from project xsd-parser-rs
//! Canonical source; https://github.com/lumeohq/xsd-parser-rs
//!
//! MIT License
//!
//! Copyright (c) 2020 Lumeo, Inc.
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

use std::{
    fmt,
    io::{Read, Write},
    ops::Deref,
    str::FromStr,
};

use chrono::{format::ParseError, DateTime as CDateTime, FixedOffset};
use yaserde::{de::Deserializer, ser::Serializer, YaDeserialize, YaSerialize};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct DateTime {
    inner: CDateTime<FixedOffset>,
    // NOTE; Required for deref into &str
    // WARN; This is a workaround for not properly serializing struct fields that are marked with
    // #[yaserder(text = true)]
    // ERROR; This approach doesn't work for deserialization, there is too much specialization on the text element
    // being of type "String"
    serialized: String,
}

impl DateTime {
    fn new(inner: CDateTime<FixedOffset>) -> Self {
        let serialized = inner.to_rfc3339();
        Self { inner, serialized }
    }

    pub fn as_internal(&self) -> CDateTime<FixedOffset> {
        self.inner.clone()
    }
}

impl FromStr for DateTime {
    type Err = ParseError;

    // Note:
    // `parse_from_rfc3339` parses an RFC 3339 and ISO 8601 date and time string.
    // XSD follows ISO 8601, which allows no time zone at the end of literal.
    // Since RFC 3339 does not allow such behavior, the function tries to add
    // 'Z' (which equals "+00:00") in case there is no timezone provided.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tz_provided = s.ends_with('Z') || s.contains('+') || s.matches('-').count() == 3;
        let s_with_timezone = if tz_provided {
            s.to_string()
        } else {
            format!("{}Z", s)
        };
        match CDateTime::parse_from_rfc3339(&s_with_timezone) {
            Ok(cdt) => Ok(DateTime::new(cdt)),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.serialized)
    }
}

impl Deref for DateTime {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.serialized
    }
}

impl YaSerialize for DateTime {
    fn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        yaserde::primitives::serialize_primitives(self, "DateTime", writer, DateTime::to_string)
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<xml::attribute::OwnedAttribute>,
        namespace: xml::namespace::Namespace,
    ) -> Result<
        (
            Vec<xml::attribute::OwnedAttribute>,
            xml::namespace::Namespace,
        ),
        String,
    > {
        Ok((attributes, namespace))
    }
}

impl YaDeserialize for DateTime {
    fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
        yaserde::primitives::deserialize_primitives(reader, |str| {
            DateTime::from_str(str).map_err(|e| e.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use xml::reader::{Error as XmlError, XmlEvent};
    use yaserde_derive::{YaDeserialize, YaSerialize};

    use super::*;

    pub fn assert_xml_eq(actual: &str, expected: &str) {
        for (a, e) in without_whitespaces(actual).zip(without_whitespaces(expected)) {
            assert_eq!(a, e);
        }
    }

    fn without_whitespaces(
        expected: &str,
    ) -> impl Iterator<Item = Result<XmlEvent, XmlError>> + '_ {
        xml::EventReader::new(expected.as_bytes())
            .into_iter()
            .filter(|e| !matches!(e, Ok(XmlEvent::Whitespace(_))))
    }

    #[test]
    fn datetime_parse_test() {
        // No timezone.
        let offset = FixedOffset::east_opt(0).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00"),
            Ok(DateTime::new(dt))
        );
        // Timezone "Z".
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00Z"),
            Ok(DateTime::new(dt))
        );

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00+06:30"),
            Ok(DateTime::new(dt))
        );

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTime::from_str("2020-03-07T04:40:00-06:30"),
            Ok(DateTime::new(dt))
        );
    }

    #[test]
    fn datetime_display_test() {
        // Timezone +00:00.
        let offset = FixedOffset::east_opt(0).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime::new(dt).to_string(), "2020-03-07T04:40:00+00:00");

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime::new(dt).to_string(), "2020-03-07T04:40:00+06:30");

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime::new(dt).to_string(), "2020-03-07T04:40:00-06:30");
    }

    #[derive(Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {
        "t" = "test"
        }
    )]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: DateTime,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn datetime_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-03-07T04:40:00+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;

        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        let m = Message {
            created_at: DateTime::new(dt),
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn datetime_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-03-07T04:40:00-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(s).unwrap();

        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc = NaiveDate::from_ymd_opt(2020, 3, 7)
            .unwrap()
            .and_hms_opt(4, 40, 0)
            .unwrap()
            - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);

        assert_eq!(m.created_at.as_internal(), dt);
        assert_eq!(m.text, "Hello world".to_string());
    }
}
