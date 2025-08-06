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

echo "==> Done."
