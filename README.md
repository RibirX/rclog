# Ribir Changelog

Ribir Changelog is a tool that combines changelogs of pre-release versions into the more stable version and pulls out information for a specific version.

This tool helps with two main tasks for [Ribir](ribir.org) changelog management:

1. It automatically combines changelogs from multiple pre-release versions into the next version. For example, it can merge all alpha version changelogs into the beta version, and all beta version changelogs into the stable version.
2. It can pull out the details of a specific version to help create a release note for that version.

## Important

This tool works with changelogs in markdown format. It expects each version to be a second-level heading. If you're using the [keepachangelog](https://keepachangelog.com/en/1.0.0/) format for your changelog, this tool will work perfectly.

## Installation

```sh
cargo install rclog
```

## How to Use

To pull out the changelog from a specific version:

```sh
rclog -t 0.1.0 -p ./CHANGELOG.md extract
```

To merge all pre-release versions into the more stable version:

```sh
rclog -t 0.1.0 -p ./CHANGELOG.md merge
```

Run `rclog --help` for more information.

## Use in GitHub Action

This project also provides a reusable GitHub workflow to help Rust project in Github to release version. See 
[release-version.yml](./github/workflows/release-version.yml) to see how it works.

To publish a new version to crates.io you need to set your publish secret token of `crates.io` in your repository. This action will use `${{ secrets.CRATE_RELEASE_TOKEN }}` to access the token.

This workflow is based on [cargo-release](https://github.com/crate-ci/cargo-release) and `rclog`. 

If you want to merge changelogs from all pre-release versions into the release version, you need to configure `pre-release-hook` for `cargo-release` in your project's `Cargo.toml` or `release.toml` with this content:

```toml
pre-release-hook = ["./rclog_hook.sh"]
```

And the `rclog_hook.sh` should be like this:

```sh
#!/bin/bash

if $MERGE_CHANGELOG; then
  echo "$(rclog -t $NEW_VERSION -p ./CHANGELOG.md merge)" >| ./CHANGELOG.md
fi
```