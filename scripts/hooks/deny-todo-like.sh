#!/bin/sh

# An hook script to verify changes to be committed do not contain
# any 'FIXME:' comments. Called by "git commit" with no arguments.
#
# The hook should exit with non-zero status after issuing an appropriate
# message if it stops the commit.
#
# To bypass this hook, use the "--no-verify" parameter when committing.

# Redirect output to stderr.
exec 1>&2

# Define colors
RED='\033[0;31m'
NC='\033[0m'

# Define what term will be searched for.
SEARCH_TERM_FIXME="FIXME"
SEARCH_TERM_TODO="TODO"

has_todo=0

for file in "$@"; do
  matches=$(grep -Ein "\b(${SEARCH_TERM_TODO}|${SEARCH_TERM_FIXME})\b" "$file" || true)
  if [ -n "$matches" ]; then
    echo "$matches"
    echo "  ===> Found TODO or FIXME in: ${RED}$file${NC}\n"
    has_todo=1
  fi
done

if [ "$has_todo" -eq 1 ]; then
  exit 1
fi
