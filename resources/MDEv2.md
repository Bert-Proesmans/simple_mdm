# MDE v2

Mobile Device Enrollment Protocol Version 2

REF; https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-mde2/4d7eadd5-3951-4f1c-8159-c39e07cbe692

I attempted to create a SOAP descriptor file (wsdl extension) that matches observed requests/responses from the windows MDM client.
The PDF is a more readable version of the protocol details.
The SOAP envelope part of the protocol is incomplete, and I filled in details from the SOAP/XML processing crates in Rust.
The wdsl file should define all necessary information to at least create a compliant client application, at best enough detail to create
a server application/service. I've never made use of SOAP before, nor do the rust crates implement all features I find sprinkled all over
the internet. I had to make do with the most common (basic) schema and adjust manually to make stuff work.

[Zeep - XSD/WSDL client code generator for Rust]: https://github.com/mibes404/zeep/
[xsd/wsdl - SOAP/XSDL/WSDL model Rust code generator written in Rust]: https://github.com/lumeohq/xsd-parser-rs