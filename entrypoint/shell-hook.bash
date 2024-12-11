#!/bin/bash
echo "Setting up; PWD=$PWD EUID=$EUID JOBS=${JOBS} DEBUG_MODE=${DEBUG_MODE} SGX_PRERELEASE=${SGX_PRERELEASE}"
if [[ "$DEBUG_MODE" -eq 1 ]]; then
  echo "Building in debug mode"
  if [[ "$SGX_DEBUG" -ne 1 ]]; then
    echo "SGX_DEBUG should be set"
    exit 1
  fi
  if [[ "$SGX_PRERELEASE" -eq 1 ]]; then
    echo "SGX_PRERELEASE should not be set"
    exit 1
  fi
else
  echo "Building in release mode"
  if [[ "$SGX_DEBUG" -eq 1 ]]; then
    echo "SGX_DEBUG should not be set"
    exit 1
  fi
fi
all_ps="$(ps aux)"
# Start the AESM service if it is not already running.
if [[ ! "${all_ps}" =~ 'aesm_service' ]]; then
  if [[ $EUID -ne 0 ]]; then
    # Background on AESM service issues in Docker: https://gitlab.com/dexlabs/derivadex/-/issues/3872
    # Simplified container-appropriate variant of Intel's initd config: `/opt/intel/sgx-aesm-service/startup.sh`
    sudo -b LD_LIBRARY_PATH=/opt/intel/sgx-aesm-service/aesm/ /opt/intel/sgx-aesm-service/aesm/aesm_service 2>&1 &
    echo "Started AESM service at root"
  else
    /opt/intel/sgx-aesm-service/aesm/aesm_service 2>&1 &
    echo "Started AESM service"
  fi
else
  echo "AESM service is already running"
fi
exec bash -i -l
