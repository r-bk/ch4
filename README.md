# ch4 - DNS Client Tool

**ch4** is a DNS Client CLI tool built around [rsdns](https://github.com/r-bk/rsdns).

*ch4's* primary goal is to expose *rsdns's* capabilities in a concise manner.
It can be used as a simple substitute for [dig](https://en.wikipedia.org/wiki/Dig_(command)),
with which it tries to be mostly compatible.

*ch4* by default uses the [tokio](https://github.com/tokio-rs/tokio) async runtime and the
corresponding *rsdns* client. It can be built with all clients and async runtimes supported
by *rsdns* via the same set of features `net-tokio`, `net-async-std`, `net-smol` and `net-std`.

[![crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/ch4.svg
[crates-url]: https://crates.io/crates/ch4

## Features

1. Built on Linux, Windows, and MacOS
2. Auto-detection of operating system's default nameserver


## Supported RFCs
- *rsdns* [supported RFCs](https://github.com/r-bk/rsdns#supported-rfcs)
- [RFC 3597] - text representation of unknown record types

[RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5


## Installation

```shell
cargo install ch4
```

*ch4* currently doesn't provide pre-built binaries.
See [rustrup.rs](https://rustup.rs) for installation of cargo and the Rust toolchain.


## Examples

Nameserver address can be specified with the `@` prefix.

```shell
$> ch4 @8.8.8.8 A docs.rs
```
```text
; <<>> ch4 0.16.0 git:d7236e0 <<>> @8.8.8.8 A docs.rs
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 31376
;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 512
;; QUESTION SECTION:
;docs.rs.                      IN     A      

;; ANSWER SECTION:
docs.rs.                60     IN     A      13.226.2.120
docs.rs.                60     IN     A      13.226.2.68
docs.rs.                60     IN     A      13.226.2.78
docs.rs.                60     IN     A      13.226.2.103

;; Query time: 62.709564ms
;; SERVER: 8.8.8.8:53
;; WHEN: Fri, 1 Nov 2024 08:56:36 +0200
;; MSG SIZE rcvd: 100
```

When nameserver is not specified, it is auto-detected from the OS configuration.

```shell
$> ch4 crates.io ANY
```
```text
; <<>> ch4 0.16.0 git:d7236e0 <<>> crates.io ANY
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 53667
;; flags: qr rd ra; QUERY: 1, ANSWER: 12, AUTHORITY: 0, ADDITIONAL: 6

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 65494
;; QUESTION SECTION:
;crates.io.                      IN     ANY    

;; ANSWER SECTION:
crates.io.                300    IN     TXT    "v=spf1 include:mailgun.org ~all"
crates.io.                300    IN     MX     10 mxa.mailgun.org.
crates.io.                300    IN     MX     10 mxb.mailgun.org.
crates.io.                900    IN     SOA    ns-1064.awsdns-05.org. awsdns-hostmaster.amazon.com. 1 7200 900 1209600 86400
crates.io.                60     IN     A      13.226.2.64
crates.io.                60     IN     A      13.226.2.33
crates.io.                60     IN     A      13.226.2.63
crates.io.                60     IN     A      13.226.2.87
crates.io.                60     IN     NS     ns-817.awsdns-38.net.
crates.io.                60     IN     NS     ns-1064.awsdns-05.org.
crates.io.                60     IN     NS     ns-1543.awsdns-00.co.uk.
crates.io.                60     IN     NS     ns-217.awsdns-27.com.

;; ADDITIONAL SECTION:
ns-217.awsdns-27.com.     22700  IN     A      205.251.192.217
ns-817.awsdns-38.net.     151496 IN     A      205.251.195.49
ns-1064.awsdns-05.org.    87690  IN     A      205.251.196.40
ns-1543.awsdns-00.co.uk.  35046  IN     A      205.251.198.7
ns-817.awsdns-38.net.     4691   IN     AAAA   2600:9000:5303:3100::1

;; Query time: 16.550292ms
;; SERVER: 127.0.0.53:53
;; WHEN: Fri, 1 Nov 2024 08:55:57 +0200
;; MSG SIZE rcvd: 487
```


## Options

*ch4's* options follow the features exposed by *rsdns*.
The detailed list of options is shown via `--help` flag.

```shell
$> ch4 --help
```
```text
DNS Client

Usage: ch4 [OPTIONS] [POSITIONAL]...

Arguments:
  [POSITIONAL]...
          Positional arguments ...
          
          Positional arguments may be specified without any particular order.
          Arguments specified later take precedence.
          Arguments that are not recognized as special are treated as names
          to be queried.
          
          
          @<nameserver> - specifies the nameserver IP address.
                          If not specified, the first nameserver from OS
                          configuration is used.
          
          <qtype>       - query type (A, AAAA, NS ...).
                          An argument matching any of the supported query types
                          is considered as query type. Trailing dot can be
                          used to disambiguate a query name (A.).
          
          +udp          - sets the Udp protocol strategy. UDP is used by default.
                          Truncated responses are retried using TCP.
          
          +tcp          - sets the Tcp protocol strategy.
                          Only TCP is used for all queries.
          
          +notcp        - sets NoTcp protocol strategy. Only UDP is used.
                          Truncated queries are returned as is, without retry.
          
          +[no]rec      - enables (disables) recursive query.
                          Queries are recursive by default.
          
          +[no]short    - enables (disables) short output.
                          When enabled, only record data is printed,
                          one record on a line.
          
          +bufsize=#    - sets the EDNS0 max udp payload size [512, 65535].
                          [default: 4096]
          
          +[no]edns[=#] - enables/disables EDNS0.
                          Optionally, sets the EDNS version [0, 255].
                          By default, EDNS is enabled with version 0.
          
          +[no]rust     - enables (disables) rust output.
                          When enabled, prints the response as a Rust array.
          
          +[no]gen      - forces generic output (RFC 3597 s. 5) on all record
                          types. By default, only unknown record types are
                          formatted this way.

Options:
  -p, --port <PORT>
          [default: 53]

  -l, --query-lifetime <QUERY_LIFETIME>
          query lifetime (in msec).
          
          [default: 10000]

  -t, --query-timeout <QUERY_TIMEOUT>
          query timeout (in msec). Use 0 to disable.
          
          [default: 2000]

      --info
          Prints build information

      --list-nameservers
          Lists system nameservers

  -s, --save <SAVE_PATH>
          save responses to file

  -r, --read <READ_PATH>
          read responses from file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

The list of operating system configured nameservers is shown via the `--list-nameservers` flag.

```shell
$> ch4 --list-nameservers
```
```text
8.8.8.8
208.67.222.222
8.26.56.26
```

Build information is shown via `--info` flag.

```shell
C:\> ch4.exe --info
```
```text
build time:          Fri, 1 Nov 2024 06:47:29 +0000
ch4 semver:          0.15.0
rsdns semver:        0.19.0
git hash:            0a00162c47289da7b28d76904af13e0b90248a8f
compiler:            C:\\Users\\runneradmin\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\bin\\rustc.exe
rustc:               rustc 1.82.0 (f6e511eec 2024-10-15)
cargo features:      net_tokio, tokio
cargo profile:       debug
cargo target:        x86_64-pc-windows-msvc
endianness:          little
pointer width:       64
build system name:   Windows
build os version:    Windows Server 2022 Datacenter
build cpu vendor:    AuthenticAMD
build cpu brand:     AMD EPYC 7763 64-Core Processor
```

## Changelog

The changelog is maintained in [CHANGELOG.md](CHANGELOG.md)


## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
