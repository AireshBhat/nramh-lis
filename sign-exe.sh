#!/bin/bash

# Check if required arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <path-to-exe> <path-to-pfx>"
    exit 1
fi

EXE_PATH="$1"
PFX_PATH="$2"
SIGNED_OUTPUT="${EXE_PATH%.*}_signed.exe"

# Check if files exist
if [ ! -f "$EXE_PATH" ]; then
    echo "Error: Executable file not found: $EXE_PATH"
    exit 1
fi

if [ ! -f "$PFX_PATH" ]; then
    echo "Error: PFX certificate file not found: $PFX_PATH"
    exit 1
fi

# Check if osslsigncode is installed
if ! command -v osslsigncode &> /dev/null; then
    echo "osslsigncode is not installed. Installing via Homebrew..."
    brew install osslsigncode
fi

# Prompt for PFX password
read -sp "Enter PFX certificate password: " PFX_PASSWORD
echo

# Sign the executable
echo "Signing executable..."
osslsigncode sign \
    -pkcs12 "$PFX_PATH" \
    -pass "$PFX_PASSWORD" \
    -n "Application Name" \
    -i "http://your.website.com/" \
    -in "$EXE_PATH" \
    -out "$SIGNED_OUTPUT"

# Check if signing was successful
if [ $? -eq 0 ]; then
    echo "Successfully signed. Output file: $SIGNED_OUTPUT"
    
    # Verify the signature
    echo "Verifying signature..."
    osslsigncode verify "$SIGNED_OUTPUT"
    
    echo -e "\nWould you like to replace the original file with the signed version? (y/n)"
    read -r response
    if [[ "$response" =~ ^[Yy]$ ]]; then
        mv "$SIGNED_OUTPUT" "$EXE_PATH"
        echo "Original file replaced with signed version."
    else
        echo "Signed file kept as separate file: $SIGNED_OUTPUT"
    fi
else
    echo "Error: Signing failed"
    exit 1
fi