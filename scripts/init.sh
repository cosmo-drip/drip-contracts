#!/bin/bash
set -e

echo "==> Setting up development environment..."

# Check that pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
  echo "[ERROR] 'pre-commit' is not installed."
  echo "        You can install it with:"
  echo "          - pip install pre-commit"
  echo "          - brew install pre-commit  # (macOS recommended)"
  exit 1
fi

# Install Git pre-commit hook
echo "==> Installing pre-commit hook..."
pre-commit install

# Optionally update hook versions
echo "==> Updating pre-commit hook versions..."
pre-commit autoupdate

echo "==> Checking for git-flow..."
# Check if git-flow is installed
if ! command -v git-flow &> /dev/null; then
  echo "[ERROR] 'git-flow' is not installed."
  echo "        You can install it for macOS:"
  echo "          - brew install git-flow-avh"
  echo "        You can install it for Linux:"
  echo "          - $ sudo apt-get install git-flow"
  exit 1
else
  echo "==> Initializing Git Flow..."

  git flow init \
    -f \
    --feature    "feature/" \
    --bugfix     "bugfix/" \
    --release    "release/" \
    --hotfix     "hotfix/" \
    --support    "support/" \
    --tag        "v" \
    --defaults

  echo "==> Git Flow initialized."
fi

check_and_copy() {
  local config_file="$1"
  local config_example_file="$2"
  local red='\033[0;31m'
  local reset='\033[0m'

  echo "==> Checking for $config_file..."
  if [ ! -f "$config_file" ]; then
    if [ -f "$config_example_file" ]; then
      echo "==> $config_file not found. Copying from $config_example_file..."
      cp "$config_example_file" "$config_file"
      echo -e "==> Please ${red}manually update${reset} $config_file with your local settings."
    else
      echo "[WARNING] Neither $config_file nor $config_example_file found."
      echo "          Please create $config_file manually."
    fi
  else
    echo "==> $config_file already exists. Skipping."
  fi
}

check_and_copy "configs/config.mk" "configs/config.mk.example"
check_and_copy "configs/instantiate_msg.json" "configs/instantiate_msg.json.example"

echo "==> Done."
