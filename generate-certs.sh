#!/bin/bash

# Generate self-signed certificates for local development
# Run this script once before starting the stack

CERT_DIR="./traefik/certs"
DOMAIN="${LOCAL_DOMAIN:-home.local}"

mkdir -p "$CERT_DIR"

# Generate private key
openssl genrsa -out "$CERT_DIR/local.key" 2048

# Generate certificate signing request and certificate
openssl req -new -x509 \
    -key "$CERT_DIR/local.key" \
    -out "$CERT_DIR/local.crt" \
    -days 3650 \
    -subj "/C=BR/ST=SP/L=SaoPaulo/O=MediaStack/CN=*.${DOMAIN}" \
    -addext "subjectAltName=DNS:${DOMAIN},DNS:*.${DOMAIN},DNS:localhost,IP:127.0.0.1,IP:192.168.1.250"

echo "Certificates generated successfully in $CERT_DIR"
echo ""
echo "To trust the certificate on your system:"
echo ""
echo "  Linux:   sudo cp $CERT_DIR/local.crt /usr/local/share/ca-certificates/ && sudo update-ca-certificates"
echo "  macOS:   sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain $CERT_DIR/local.crt"
echo "  Windows: Import $CERT_DIR/local.crt to 'Trusted Root Certification Authorities'"
echo ""
