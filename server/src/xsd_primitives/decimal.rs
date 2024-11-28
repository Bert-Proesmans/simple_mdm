use std::{
    fmt,
    io::{Read, Write},
    str::FromStr,
};
use yaserde::{de::Deserializer, ser::Serializer, YaDeserialize, YaSerialize};

#[derive(Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct Decimal(rust_decimal::Decimal);

impl Decimal {
    fn as_internal(&self) -> rust_decimal::Decimal {
        self.0.clone()
    }
}

impl TryFrom<f32> for Decimal {
    type Error = String;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Decimal(
            rust_decimal::Decimal::from_f32_retain(value).ok_or("Value out of range")?,
        ))
    }
}

impl TryFrom<f64> for Decimal {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Ok(Decimal(
            rust_decimal::Decimal::from_f64_retain(value).ok_or("Value out of range")?,
        ))
    }
}

impl FromStr for Decimal {
    type Err = rust_decimal::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal(rust_decimal::Decimal::from_str(s)?))
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl YaSerialize for Decimal {
    fn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        yaserde::primitives::serialize_primitives(self, "Decimal", writer, Decimal::to_string)
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

impl YaDeserialize for Decimal {
    fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
        yaserde::primitives::deserialize_primitives(reader, |str| {
            Decimal::from_str(str).map_err(|e| e.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
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

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct DecimalPair {
        #[yaserde(prefix = "t", rename = "First")]
        pub first: Decimal,

        #[yaserde(prefix = "t", rename = "Second")]
        pub second: Decimal,
    }

    #[test]
    fn decimal_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i = DecimalPair {
            first: Decimal(rust_decimal::Decimal::new(1234, 5)),
            second: Decimal(rust_decimal::Decimal::new(-1234, 2)),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn decimal_deserialize_test() {
        // Value "+0.01234" is used to check optional plus sign deserialization.
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:DecimalPair xmlns:t="test">
                <t:First>+0.01234</t:First>
                <t:Second>-12.34</t:Second>
            </t:DecimalPair>
            "#;
        let i: DecimalPair = yaserde::de::from_str(s).unwrap();
        assert_eq!(i.first.as_internal(), rust_decimal::Decimal::new(1234, 5));
        assert_eq!(i.second.as_internal(), rust_decimal::Decimal::new(-1234, 2));
    }
}
