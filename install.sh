#!/bin/bash

SRC_DIR="src/proto"
DEST_DIR="/usr/local/bin"

echo "Building the Rust project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build succeeded."

    EXE_NAME="$(basename "$(pwd)")"
    echo "Installing $EXE_NAME to $DEST_DIR..."

    cp "target/release/$EXE_NAME" "$DEST_DIR"

    if [ $? -eq 0 ]; then
        echo "Installed $EXE_NAME to $DEST_DIR successfully."
    else
        echo "Failed to install $EXE_NAME to $DEST_DIR."
    fi

    echo "Copying $SRC_DIR to $DEST_DIR..."
    cp -r "$SRC_DIR" "$DEST_DIR"

    if [ $? -eq 0 ]; then
        echo "Copied $SRC_DIR to $DEST_DIR successfully."
    else
        echo "Failed to copy $SRC_DIR to $DEST_DIR."
    fi

else
    echo "Build failed. Please check the error messages above."
fi

