#!/bin/bash
# This script is used to install the Rust SGX SDK

# get the environment variable TEACLAVE_SDK if it is set or use the default value
TEACLAVE_SDK=${TEACLAVE_SDK:-"rust-sgx-sdk"}

# git clone the source code from the remote repository to TEACLAVE_SDK
# Set your remote repository URL and branch name
REMOTE_URL="https://gitlab.com/dexlabs/incubator-teaclave-sgx-sdk"
BRANCH_NAME="v2.0.0-sgx-emm"

# Check if the local directory exists
if [ ! -d "$TEACLAVE_SDK" ]; then
    # If it doesn't exist, create the directory and fetch the branch
    mkdir -p "$TEACLAVE_SDK"
    git clone --depth 1 --branch "$BRANCH_NAME" "$REMOTE_URL" "$TEACLAVE_SDK"
    echo "Local directory created and branch fetched."
else
    echo "Local directory is not empty. Just fetch."
    git -C "$TEACLAVE_SDK" pull
fi

