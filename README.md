# Ribir Changelog

Ribir Changelog is a tool that makes your development process easier. It combines changelogs from different pre-release versions and pulls out information for a specific version.

This tool helps with two main tasks for [Ribir](ribir.org) changelog management:

1. It automatically combines changelogs from multiple pre-release versions into the next version. For example, it can merge all alpha version changelogs into the beta version, and all beta version changelogs into the stable version.
2. It can pull out the details of a specific version to help create a release note for that version.

## Important

This tool works with changelogs in markdown format. It expects each version to be a second-level header. If you're using the [keepachangelog](https://keepachangelog.com/en/1.0.0/) format for your changelog, this tool will work perfectly.

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