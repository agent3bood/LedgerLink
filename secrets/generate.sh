#!/bin/bash

# Define file names
private_key_pem="secp256k1_private.pem"
private_key_der="secp256k1_private_pkcs8.der"
public_key_der="secp256k1_public.der"
private_key_base64="secp256k1_private_pkcs8_base64.txt"
public_key_base64="secp256k1_public_base64.txt"

# Generate ECDSA secp256k1 Private Key in PEM format
openssl ecparam -genkey -name secp256k1 -out "$private_key_pem"

# Convert the Private Key to PKCS#8 DER format
openssl pkcs8 -topk8 -inform PEM -outform DER -in "$private_key_pem" -out "$private_key_der" -nocrypt

# Extract the Public Key in DER format
openssl ec -in "$private_key_pem" -pubout -outform DER -out "$public_key_der"

# Encode the DER files to Base64 and remove new lines
base64 < "$private_key_der" | tr -d '\n' > "$private_key_base64"
base64 < "$public_key_der" | tr -d '\n' > "$public_key_base64"

# Output the Base64 encoded keys
echo "Private Key (Base64 Encoded):"
cat "$private_key_base64"
echo ""
echo "Public Key (Base64 Encoded):"
cat "$public_key_base64"
echo ""
