# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2021-08-27
### Changed
- fix `@` nameserver prefix example in the README file
- add a link to the CHANGELOG in the README file

## [0.3.0] - 2021-08-27
### Added
- add `+[no]short` flag, for compatibility with `dig`. When enabled, `ch4` performs
  CNAME flattening to find the requested record type. Only record data is printed,
  one record on a line.

### Fixed
- `-p, --port` option was fixed on Windows. Was ignored in favor of OS configured port.

### Changed
- update Cargo dependencies

### Removed
- remove `--rrset` in favor of the `+short` flag

## [0.2.0] - 2021-08-13
### Added
- update to `rsdns 0.3.0`
- support leading digits in domain names `RFC 1101`
- add installation instructions in README

## [0.1.0] - 2021-08-13
- First release
