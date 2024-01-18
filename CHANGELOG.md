# Changelog

All notable changes to this project will be documented in this file.

Please keep one empty line before and after all headers. (This is required for `git` to produce a conflict when a release is made while a PR is open and the PR's changelog entry would go into the wrong section).

There are 5 types of changes:

- `Features` for new features.
- `Changed` for changes in existing functionality.
- `Fixed` for any bug fixes.
- `Breaking` for the detail of any backward incompatible changes.
- `Infrastructure`for all the tools and processes that support our development, such as CI/CD, testing frameworks, build tools, etc.

Please only add new entries below the [Unreleased](#unreleased---releasedate) header with the following format:

``` md
-  description of change (#PR @contributor)
```

<!-- next-header -->

## [@Unreleased] - @ReleaseDate

## [0.1.3-alpha.1](https://github.com/RibirX/rclog/compare/v0.1.2...v0.1.3-alpha.1) - 2024-01-18

## [0.1.2](https://github.com/RibirX/rclog/compare/v0.1.1-alpha.1...v0.1.2) - 2024-01-08

### Infrastructure

- Add github actions to manage rclog release(\#1 @M-Adoo)

### Features

<!-- next-url -->
[@Unreleased]: https://github.com/RibirX/rclog/compare/v0.1.3-alpha.1...HEAD

- support extract changelog from a specific version: `rclog -t 0.1.0 -p ./CHANGELOG.md extract`
- support merge changelog from multi pre-release version to the more stable version: `rclog -t 0.1.0 -p ./CHANGELOG.md merge`

The initial version of the changelog management tool of [Ribir](ribir.org). Run `rclog -h` to see the usage.

- Add a reusable workflow to help Rust project in Github to release version. (\#1 @M-Adoo)
  - Optionally, merge changelogs from all pre-release versions into the release version
  - Publish a new version to crates.io.
  - Extract the changelog of a specific version and create a new release note on GitHub.

### Fixed

- Return error when the changelog content is empty. (\#1 @M-Adoo)
- Fix merge a non-exist version may delete the pre-release version. (\#1 @M-Adoo)
