# TODO; Get the binary of the mkcert project
# REF; https://github.com/FiloSottile/mkcert

# TODO; Copy the rootCA.pem file from the certificate authority (CA) root directory 
# to your target system!
# WARN; Import the root certificate into the "Trusted Root Certification Authorities" store!
#
# NOTE; You can import a pem file without conversion using the certificate manager (certlm.msc for computer certificates),
# followed by right-clicking on a certificate tree node, followed by "All Tasks > Import...",
# followed by selecting the pem file (use all files filter). 
& "C:\Users\Bert\Downloads\mkcert-v1.4.4-windows-amd64.exe" -CAROOT
=> C:\Users\Bert\AppData\Local\mkcert

# The target system wants to connect over TLS with the server, so a TLS certificate for the
# server binary is required.
# TODO; Copy the produced certificate data to the folder "self_signed_certs".
# NOTE; No conversion of content is required
#
# WARN; The code has hardcoded paths, either change the code to match the filenames or change the
# filenames of the certificate data:
#   - certificate must be named cert.pem
#   - private key must be named key.pem
& "C:\Users\Bert\Downloads\mkcert-v1.4.4-windows-amd64.exe" "*.mdmwindows.com" "mdmwindows.com"
=> [..]
=> Created a new certificate valid for the following names 📜
=>  - "*.mdmwindows.com"
=>  - "mdmwindows.com"
=> [..]
=> The certificate is at "./_wildcard.mdmwindows.com+1.pem" and the key at "./_wildcard.mdmwindows.com+1-key.pem" ✅
