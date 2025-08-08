# Contributing Guidelines

Thank you for your interest in contributing to this project!

## Commit Message Style: Conventional Commits

We use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
specification for all commit messages.

**Example formats:**
* `feat: add support for XYZ`  
* `fix(module): handle edge case in ABC logic`  
* `chore(ci): configure GitHub Actions for linting`

**Tip for AI Assistants in IDEs:**

You can use the following prompt to help generate commit messages:

> *Generate concise and clear commit messages using the Conventional Commits
> specification. Start with a valid type (e.g., feat, fix, chore, refactor,
> docs, test, style, build, ci), optionally followed by a scope in parentheses.
> Use an imperative tone and keep the message under 70 characters. Focus only
> on the purpose of the change, not the implementation details. Do not wrap the
> commit message in any quotes, backticks, or code formatting.
> Please append the following line at the end of the commit message to reference the related ticket:
> Refs: [TICKET-ID]*

## Development Setup

To set up the repository for development:

```bash
make setup
```

This script will:
* Check if pre-commit is installed
* Install the necessary Git hooks
* Automatically keep hooks up to date

If pre-commit is not installed, the script will guide you with install
instructions (via pip or brew).

## Pre-Commit Hooks

We enforce code consistency and project hygiene using pre-commit.

Hooks are triggered automatically on each commit to validate formatting,
file structure, and other important aspects.

You can run all hooks manually via:
```bash
pre-commit run --all-files
```

> Using Git hooks locally is recommended but NOT strictly enforced.
While pre-commit hooks improve code quality and consistency during local
development, they remain optional. In some cases — such as rapid
prototyping, testing, or when making frequent small commits — hooks may
slow down the workflow or get in the way. We aim not to block productive
development, but to enforce standards where it matters most:  
**hooks are always run in CI and must pass before merging into protected branches.**
