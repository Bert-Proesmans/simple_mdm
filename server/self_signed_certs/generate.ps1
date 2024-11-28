# Output the path where CA data is stored
& "C:\Users\Bert\Downloads\mkcert-v1.4.4-windows-amd64.exe" -CAROOT

# Create a new client certificate for the server
 & "C:\Users\Bert\Downloads\mkcert-v1.4.4-windows-amd64.exe" "*.mdmwindows.com" "mdmwindows.com"