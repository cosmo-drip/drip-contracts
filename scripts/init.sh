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

echo "==> Done."
