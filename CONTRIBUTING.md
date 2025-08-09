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

## Branch Naming and Origins (GitFlow)

### General Patterns
**Branch name templat for `feature`, `bugfix`, `hotfix`:**  
`<type>/<scope>-<short-description>`

**Branch name templat for `release`:**  
`release/<major>.<minor>.<patch>`

**Branch name templat for `support`:**  
`support/<major>.<minor>`

Where:
- **type** — branch type (see table below).
- **scope** — area/context related to the change (module, service, subsystem).  
  Examples: `auth`, `payment`, `ui`, `api`.
- **short-description** — brief description of the change; lowercase, words separated by `-`.  
  Examples: `login-endpoint`, `currency-rounding-fix`.

**Examples:**  
`feature/auth-login-endpoint`  
`bugfix/payment-currency-rounding`

> Recommendations:
> - Use only `a-z0-9-`
> - No slashes in `scope` or description
> - Keep under ~60–70 characters
> - Optionally append task ID at the end:  
>   `feature/auth-login-endpoint-PROJ-123`

## Permanent Branches (not used as `<type>`)
- **main** — production source of truth.
- **develop** — integration branch for new changes.

**From `main` create only:** `hotfix/*` and (sometimes) long-living `support/*` branches.  
**All features should branch from:** `develop` (or from `support/x.y` for LTS maintenance).

## Branch Types and Their Origins

| Type      | Purpose                                     | Branch from                           | Merge into after completion                                             | Examples |
|-----------|---------------------------------------------|---------------------------------------|-------------------------------------------------------------------------|----------|
| `feature` | New functionality                           | `develop` \| `support/x.y`            | `develop` \| same `support/x.y`                                         | `feature/auth-login-endpoint` |
| `bugfix`  | Fixes before release / in release branch    | `develop` \| `release/x.y.z`          | Source branch and then `develop`                                        | `bugfix/payment-currency-rounding` |
| `release` | Preparing a release                         | `develop`                             | `main` (with tag) and back to `develop`                                 | `release/1.4.0` |
| `hotfix`  | Urgent fix in production                    | `main` \| `support/x.y`               | `main` (+tag), `develop`, `support`                                     | `hotfix/api-rate-limit` |
| `support` | Long-living maintenance branch for a version| **Once** from `main` at release point | Fixes stay in `support/x.y`; optionally cherry-pick to `main`/`develop` | `support/1.4` |

## Policy Summary
- `main` and `develop` are **long-living** and **not** used as `<type>`.
- No regular development starts from `main`: only `hotfix/*` and creation of `support/*` branches.
- For maintaining older versions, use `support/x.y` and branch `feature/`, `bugfix/`, or `hotfix/` from it as needed.

## Release Maintenance Branches

Non-functional changes required to prepare or adjust a release — such as version updates,
changelog edits, CI/CD pipeline adjustments, or configuration tweaks — must be committed via short-lived branches:  
`bugfix/<scope>-<short-description>`

- Branch from the target `release/<major>.<minor>.<patch>` or `support/<major>.<minor>`.
- Merge back into the same branch via a Pull Request (no direct pushes).
- Tag such PRs with the `chore` label to indicate operational or release-maintenance work.
- Ensure these changes are also merged into `develop` after the release (back-merge or cherry-pick).

## Protected branches & PR policy

todo
