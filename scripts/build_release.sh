#!/bin/bash

# You do need "tomlq" installed on your system.
# If you use ubuntu linux, you can install it with:
# sudo apt-get install yq

VERSION=$(tomlq ".package.version" Cargo.toml -r | tr -d '\r\n')
echo "Building flohole version $VERSION …"

# Build the release version of the flohole application
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Copy the binary to the release directory
mkdir -p release/flohole_${VERSION}_linux
mkdir -p release/flohole_${VERSION}_windows

cp target/x86_64-unknown-linux-gnu/release/flohole release/flohole_${VERSION}_linux/flohole
cp target/x86_64-pc-windows-gnu/release/flohole.exe release/flohole_${VERSION}_windows/flohole.exe

# Create a zip file of the release directory
cd release
zip -qr flohole_${VERSION}_windows.zip flohole_${VERSION}_windows
zip -qr flohole_${VERSION}_linux.zip flohole_${VERSION}_linux

# Clean up the release directory
rm -rf flohole_${VERSION}_linux flohole_${VERSION}_windows
