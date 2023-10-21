# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.12.0] - 2023-10-21

### Added

- `ch4` now supports unknown record types, i.e. record types which are still
not officially supported in `rsdns`. Such record types can be specified following
rules of [RFC 3597 section 5] for unknown types and classes. Note that `dig`
also supports types specified in this format. The following snippet shows a
query for `CAA` records on `bbc.com`.

```text
$> ch4 TYPE257 bbc.com

; <<>> ch4 0.12.0 <<>> TYPE257 bbc.com
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 50331
;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 65494
;; QUESTION SECTION:
;bbc.com.                      IN     TYPE257

;; ANSWER SECTION:
bbc.com.                300    IN     TYPE257 \# 32 0005696f 6465666d 61696c74 6f3a7365 63757269 74794062 62632e63 6f2e756b
bbc.com.                300    IN     TYPE257 \# 25 00096973 73756577 696c6467 6c6f6261 6c736967 6e2e636f 6d
bbc.com.                300    IN     TYPE257 \# 21 00056973 73756567 6c6f6261 6c736967 6e2e636f 6d
bbc.com.                300    IN     TYPE257 \# 19 00056973 73756564 69676963 6572742e 636f6d

$> dig TYPE257 bbc.com

; <<>> DiG 9.18.18-0ubuntu2-Ubuntu <<>> TYPE257 bbc.com
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 25011
;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 65494
;; QUESTION SECTION:
;bbc.com.   IN CAA

;; ANSWER SECTION:
bbc.com.  116 IN CAA 0 issue "globalsign.com"
bbc.com.  116 IN CAA 0 issuewild "globalsign.com"
bbc.com.  116 IN CAA 0 iodef "mailto:security@bbc.co.uk"
bbc.com.  116 IN CAA 0 issue "digicert.com"

;; Query time: 0 msec
;; SERVER: 127.0.0.53#53(127.0.0.53) (UDP)
;; WHEN: Sat Oct 21 22:22:10 IDT 2023
;; MSG SIZE  rcvd: 181

```

[RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5

### Changed

- the MSRV is `1.66` now due to various dependencies
- upgrade to `rsdns v0.16.0`
- upgrade to `built v0.7.1`
- upgrade to `windows v0.51.0`
- upgrade to `tera v0.19.0`
- upgrade to `sysinfo v0.29.9`

## [0.11.2] - 2023-04-08

### Changed

- upgrade to `tera v1.18.1`
- upgrade to `base64 v0.21.0`
- upgrade to `built v0.6.0`
- upgrade to `sysinfo v0.28.4`
- upgrade to `windows v0.48.0`
- upgrade to `rsdns v0.15.0`

### Fixed

- fix `clippy::derivable_impls` warning
- fix `clippy::needless-borrow` warning

## [0.11.1] - 2022-12-23

### Added

- show `rsdns` version in `ch4 --info`

## [0.11.0] - 2022-12-23

### Changed

- the MSRV is `1.64.0` now (was `1.60.0` before)
- upgrade to `windows v0.43.0`
- upgrade to `rsdns v0.14.0`
- upgrade to `sysinfo v0.27.1`
- upgrade to `base64 v0.20.0`
- auto-update other dependencies

### Fixed

- fix `clippy::uninlined_format_args` warning

## [0.10.3] - 2022-10-04

### Changed

- refresh dependencies
- upgrade to `windows v0.42.0`
- upgrade to `tera v0.17.1`
- fix `clippy` warnings detected with the latest beta releases

## [0.10.2] - 2022-07-30

### Changed

- refresh dependencies
- upgrade to `windows v0.39.0`
- raise MSRV to `v1.60.0`

## [0.10.1] - 2022-05-27

### Changed

- refresh dependencies
- upgrade to `windows v0.37.0`

### Fixed

- fix formatting of TTL in zone file format (must be explicitly left aligned)
- fix calculation of column sizes in zone file format (skip `OPT` records in sizes calculation)

## [0.10.0] - 2022-04-29

### Changed

- upgrade to `windows v0.36.1`.
- The MSRV is `v1.59.0` now, due to upgrade to the latest windows crate version.
- upgrade to `rsdns v0.13.0`. This adds support for the underscore character `_` in domain names.

## [0.9.1] - 2022-01-21

This release only refreshes the dependencies, without changing anything in *ch4* logic itself.

### Changed

- upgrade to `windows v0.30.0`
- upgrade to `sysinfo v0.23.0`
- upgrade to `rsdns v0.12.0`

## [0.9.0] - 2021-11-20

### Added

- add support for EDNS0. EDNS is enabled by default with `version: 0` and `bufsize: 4096`

  The following options allow EDNS customization:
  - `+bufsize=<n_bytes>` - set the UDP payload size advertised in the `OPT` pseudo-record
  - `+noedns` - disable EDNS
  - `+edns[=<ver>]` - enable EDNS and optionally set a custom version

### Changed

- upgrade to `windows v0.28.0`
- upgrade to `tera v1.15.0`

## [0.8.0] - 2021-10-30

### Changed

- move to `Rust 2021`. The minimum supported rust version was raised to `1.56` (`MSRV 1.56`)
- move to `rsdns 0.8.0`

## [0.7.0] - 2021-10-01

### Added

- add `+gen` output format. The flag forces `RFC 3597` generic output on all record types,
  known and unknown alike.

  ```text
  ; <<>> ch4 0.6.0 git:096f3be <<>> crates.io +gen
  ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 9844
  ;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 0

  ;; QUESTION SECTION:
  ;crates.io.                    IN     A

  ;; ANSWER SECTION:
  crates.io.              60     IN     A      \# 4 0de2003d
  crates.io.              60     IN     A      \# 4 0de20017
  crates.io.              60     IN     A      \# 4 0de2006c
  crates.io.              60     IN     A      \# 4 0de20021

  ;; Query time: 64.822579ms
  ;; SERVER: 127.0.0.53:53
  ;; WHEN: Fri, 01 Oct 2021 00:03:10 +0300
  ;; MSG SIZE rcvd: 91
  ```

### Changed

- upgrade to `rsdns v0.6.0`
- improve `+rust` formatting option:
  1. Properly align the last, possibly shorter, line of the array.
  2. Include the line offset (in bytes) in the visual comment.
  3. Add `#[rustfmt::skip]` to the generated array.

  The output of `+rust` looks like this now.

  ```text
  // A bbc.com
  #[rustfmt::skip]
  const M0: [u8; 89] = [
      0xbf, 0xe2, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, // |............| 0
      0x03, 0x62, 0x62, 0x63, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, // |.bbc.com....| 12
      0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x55, 0x00, // |..........U.| 24
      0x04, 0x97, 0x65, 0xc0, 0x51, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, // |..e.Q.......| 36
      0x00, 0x00, 0x55, 0x00, 0x04, 0x97, 0x65, 0x40, 0x51, 0xc0, 0x0c, 0x00, // |..U...e@Q...| 48
      0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x55, 0x00, 0x04, 0x97, 0x65, 0x00, // |......U...e.| 60
      0x51, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x55, 0x00, // |Q.........U.| 72
      0x04, 0x97, 0x65, 0x80, 0x51, /*                                     */ // |..e.Q| 84
  ];
  ```

- support [RFC 3597 section 5](https://www.rfc-editor.org/rfc/rfc3597.html#section-5)
  for unknown record types. Unknown record data is formatted now as an array of bytes.
  Moreover, all obsolete record types are formatted this way as well. Previously, unknown record
  types were silently ignored. See `+gen` formatting option above for an output example.

## [0.6.0] - 2021-09-14

### Added

- add `-s/--save` option to save the received messages to a file, in addition to formatting them.

  The following command saves the received response to file `crates.io.ch4`:

  ```shell
  $> ch4 crates.io --save crates.io.ch4
  ```

  ```text
  ; <<>> ch4 0.5.0 git:3b1ecc4 <<>> --save crates.io.ch4 crates.io
  ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 2825
  ;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 0

  ;; QUESTION SECTION:
  ;crates.io.                    IN     A

  ;; ANSWER SECTION:
  crates.io.              60     IN     A      13.225.255.29
  crates.io.              60     IN     A      13.225.255.26
  crates.io.              60     IN     A      13.225.255.54
  crates.io.              60     IN     A      13.225.255.16

  ;; Query time: 67.559554ms
  ;; SERVER: 127.0.0.53:53
  ;; WHEN: Sat, 11 Sep 2021 10:07:19 +0300
  ;; MSG SIZE rcvd: 91
  ```

  the saved file has the following format:

  ```json
  [
    {
      "data": "CwmBgAABAAQAAAAABmNyYXRlcwJpbwAAAQABwAwAAQABAAAAPAAEDeH/HcAMAAEAAQAAADwABA3h/xrADAABAAEAAAA8AAQN4f82wAwAAQABAAAAPAAEDeH/EA==",
      "duration": {
        "nanos": 67559554,
        "secs": 0
      },
      "nameserver": "127.0.0.53:53",
      "qname": "crates.io",
      "qtype": "A",
      "timestamp": {
        "nanos": 347228847,
        "secs": 1631344039
      }
    }
  ]
  ```

- add `-r/--read` option to read messages written by `-s/--save`, instead of querying a nameserver.

  Now a message can be saved, and later read several times, possibly with different output formatting.

  ```shell
  $> ch4 --read crates.io.ch4 +rust
  ```

  ```text
  // A crates.io
  const M0: [u8; 91] = [
      0x0b, 0x09, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,  // |............|
      0x06, 0x63, 0x72, 0x61, 0x74, 0x65, 0x73, 0x02, 0x69, 0x6f, 0x00, 0x00,  // |.crates.io..|
      0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,  // |............|
      0x3c, 0x00, 0x04, 0x0d, 0xe1, 0xff, 0x1d, 0xc0, 0x0c, 0x00, 0x01, 0x00,  // |<...........|
      0x01, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x04, 0x0d, 0xe1, 0xff, 0x1a, 0xc0,  // |....<.......|
      0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x04, 0x0d,  // |........<...|
      0xe1, 0xff, 0x36, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,  // |..6.........|
      0x3c, 0x00, 0x04, 0x0d, 0xe1, 0xff, 0x10,                                // |<......|
  ];
  ```

## [0.5.0] - 2021-09-10

### Added

- add `+rust` option to print DNS responses as Rust arrays.

  So, the following message:

  ```text
  $> ch4 A docs.rs

  ; <<>> ch4 0.5.0 git:45400fd <<>> A docs.rs
  ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 31431
  ;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 0

  ;; QUESTION SECTION:
  ;docs.rs.                      IN     A

  ;; ANSWER SECTION:
  docs.rs.                15     IN     A      13.225.255.26
  docs.rs.                15     IN     A      13.225.255.46
  docs.rs.                15     IN     A      13.225.255.38
  docs.rs.                15     IN     A      13.225.255.105

  ;; Query time: 988.261µs
  ;; SERVER: 127.0.0.53:53
  ;; WHEN: Fri, 10 Sep 2021 07:41:11 +0300
  ;; MSG SIZE rcvd: 89
  ```

  can be printed as a Rust array the following way

  ```text
  $> ch4 A docs.rs +rust

  // A docs.rs
  const M0: [u8; 89] = [
      0xa1, 0xa7, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,  // |............|
      0x04, 0x64, 0x6f, 0x63, 0x73, 0x02, 0x72, 0x73, 0x00, 0x00, 0x01, 0x00,  // |.docs.rs....|
      0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0d, 0x00,  // |............|
      0x04, 0x0d, 0xe1, 0xff, 0x1a, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00,  // |............|
      0x00, 0x00, 0x0d, 0x00, 0x04, 0x0d, 0xe1, 0xff, 0x2e, 0xc0, 0x0c, 0x00,  // |............|
      0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x04, 0x0d, 0xe1, 0xff,  // |............|
      0x26, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0d, 0x00,  // |&...........|
      0x04, 0x0d, 0xe1, 0xff, 0x69,                                            // |....i|
  ];
  ```

### Changed

- upgrade to `rsdns v0.5.0`
- update *ch4* documentation to be aligned with *DNS Client* definition of *rsdns*

## [0.4.0] - 2021-09-04

### Added

- support `RFC 8482` for synthetic `HINFO` response on `ANY` query.
  Till now `HINFO` record type was treated as obsolete, and its data wasn't formatted in zone output.

### Changed

- starting from this release tag names are prefixed with `v`. Old tags were adjusted accordingly.
- `UDP` is used by default for all queries, including `ANY`
- upgrade to `rsdns v0.4.0`

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

[0.1.0]: https://github.com/r-bk/ch4/releases/tag/v0.1.0
[0.2.0]: https://github.com/r-bk/ch4/compare/v0.1.0...v0.2.0
[0.3.0]: https://github.com/r-bk/ch4/compare/v0.2.0...v0.3.0
[0.3.1]: https://github.com/r-bk/ch4/compare/v0.3.0...v0.3.1
[0.4.0]: https://github.com/r-bk/ch4/compare/v0.3.1...v0.4.0
[0.5.0]: https://github.com/r-bk/ch4/compare/v0.4.0...v0.5.0
[0.6.0]: https://github.com/r-bk/ch4/compare/v0.5.0...v0.6.0
[0.7.0]: https://github.com/r-bk/ch4/compare/v0.6.0...v0.7.0
[0.8.0]: https://github.com/r-bk/ch4/compare/v0.7.0...v0.8.0
[0.9.0]: https://github.com/r-bk/ch4/compare/v0.8.0...v0.9.0
[0.9.1]: https://github.com/r-bk/ch4/compare/v0.9.0...v0.9.1
[0.10.0]: https://github.com/r-bk/ch4/compare/v0.9.1...v0.10.0
[0.10.1]: https://github.com/r-bk/ch4/compare/v0.10.0...v0.10.1
[0.10.2]: https://github.com/r-bk/ch4/compare/v0.10.1...v0.10.2
[0.10.3]: https://github.com/r-bk/ch4/compare/v0.10.2...v0.10.3
[0.11.0]: https://github.com/r-bk/ch4/compare/v0.10.3...v0.11.0
[0.11.1]: https://github.com/r-bk/ch4/compare/v0.11.0...v0.11.1
[0.11.2]: https://github.com/r-bk/ch4/compare/v0.11.1...v0.11.2
[0.12.0]: https://github.com/r-bk/ch4/compare/v0.11.2...v0.12.0
