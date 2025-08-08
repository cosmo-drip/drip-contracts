#!/bin/sh
set -e

# Detect system architecture for proper Docker image selection
ARCH=$(uname -m)
OPTIMIZER_VERSION="0.17.0"

if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
  IMAGE="cosmwasm/optimizer-arm64:${OPTIMIZER_VERSION}"
  echo "==> Detected ARM architecture ($ARCH), using $IMAGE"
else
  IMAGE="cosmwasm/optimizer:${OPTIMIZER_VERSION}"
  echo "==> Detected x86_64 architecture ($ARCH), using $IMAGE"
fi

# Optional: Provide path to external Cargo.lock as first argument
EXTERNAL_CARGO_LOCK="$1"

# Conditionally mount external Cargo.lock if provided
if [ -n "$EXTERNAL_CARGO_LOCK" ]; then
  # Ensure there is no local Cargo.lock to avoid overwrite issues
  if [ -f "Cargo.lock" ]; then
    echo " ===! Error: Local Cargo.lock file exists while external lock file is being mounted."
    echo "      Refusing to proceed to prevent possible conflicts."
    exit 1
  fi

  ABS_LOCK_PATH=$(cd "$(dirname "$EXTERNAL_CARGO_LOCK")" && pwd)/$(basename "$1")
  echo "==> Using external Cargo.lock from $ABS_LOCK_PATH"
  echo " ===> Running optimizer..."

  docker run --rm \
    -v "$(pwd)":/code \
    -v $ABS_LOCK_PATH:/code/Cargo.lock \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    "$IMAGE"

  LOCK_FILE="Cargo.lock"

  # Attempting to remove an unexpected empty Cargo.lock file.
  # This sometimes appears to be created when a file is mounted inside a bind-mounted directory.
  # Possibly a side-effect of Docker's mount behavior. Needs further investigation if persistent.
  if [ -f "$LOCK_FILE" ] && [ ! -s "$LOCK_FILE" ]; then
    echo " ==> Removing empty $LOCK_FILE"
    rm "$LOCK_FILE"
  fi
else
  echo "==> No external Cargo.lock specified, using local if present"
  # Ensure local Cargo.lock file exists
  if [ ! -f "Cargo.lock" ]; then
    echo " ===! Error: No external Cargo.lock specified, and local Cargo.lock not found."
    echo "      Either pass a path to Cargo.lock as an argument or ensure it's present locally."
    exit 1
  fi
  docker run --rm \
    -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    "$IMAGE"
fi

echo " ===> Optimization complete."
