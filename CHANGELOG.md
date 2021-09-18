# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]
### Added
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
