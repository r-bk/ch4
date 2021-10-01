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
; <<>> ch4 0.7.0 git:3db4ab4 <<>> @8.8.8.8 A docs.rs
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 47114
;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;docs.rs.                      IN     A

;; ANSWER SECTION:
docs.rs.                60     IN     A      13.226.0.76
docs.rs.                60     IN     A      13.226.0.90
docs.rs.                60     IN     A      13.226.0.123
docs.rs.                60     IN     A      13.226.0.85

;; Query time: 66.594518ms
;; SERVER: 8.8.8.8:53
;; WHEN: Fri, 01 Oct 2021 09:11:40 +0300
;; MSG SIZE rcvd: 89
```

When nameserver is not specified, it is auto-detected from the OS configuration.

```shell
$> ch4 crates.io ANY
```
```text
; <<>> ch4 0.7.0 git:3db4ab4 <<>> crates.io ANY
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 46122
;; flags: qr rd ra; QUERY: 1, ANSWER: 12, AUTHORITY: 0, ADDITIONAL: 0

;; QUESTION SECTION:
;crates.io.                    IN     ANY

;; ANSWER SECTION:
crates.io.              300    IN     TXT    "v=spf1 include:mailgun.org ~all"
crates.io.              300    IN     MX     10 mxa.mailgun.org.
crates.io.              300    IN     MX     10 mxb.mailgun.org.
crates.io.              900    IN     SOA    ns-1064.awsdns-05.org. awsdns-hostmaster.amazon.com. 1 7200 900 1209600 86400
crates.io.              60     IN     A      13.226.0.23
crates.io.              60     IN     A      13.226.0.61
crates.io.              60     IN     A      13.226.0.33
crates.io.              60     IN     A      13.226.0.108
crates.io.              60     IN     NS     ns-1543.awsdns-00.co.uk.
crates.io.              60     IN     NS     ns-1064.awsdns-05.org.
crates.io.              60     IN     NS     ns-217.awsdns-27.com.
crates.io.              60     IN     NS     ns-817.awsdns-38.net.

;; Query time: 15.904685ms
;; SERVER: 127.0.0.53:53
;; WHEN: Fri, 01 Oct 2021 09:12:24 +0300
;; MSG SIZE rcvd: 384
```


## Options

*ch4's* options follow the features exposed by *rsdns*.
The detailed list of options is shown via `--help` flag.

```shell
$> ch4 --help
```
```text
ch4 0.7.0 git:3db4ab4
DNS Client

USAGE:
    ch4 [FLAGS] [OPTIONS] [positional]...

FLAGS:
    -h, --help
            Prints help information

        --info
            Prints build information

        --list-nameservers
            Lists system nameservers

    -V, --version
            Prints version information


OPTIONS:
    -p, --port <port>
             [default: 53]

    -l, --query-lifetime <query-lifetime>
            query lifetime (in msec). [default: 10000]

    -t, --query-timeout <query-timeout>
            query timeout (in msec). Use 0 to disable. [default: 2000]

    -r, --read <read-path>
            read responses from file

    -s, --save <save-path>
            save responses to file


ARGS:
    <positional>...
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

            +[no]rust     - enables (disables) rust output.
                            When enabled, prints the response as a Rust array.

            +[no]gen      - forces generic output (RFC 3597 s. 5) on all record
                            types. By default, only unknown record types are
                            formatted this way.
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
build time:          Fri, 01 Oct 2021 06:08:38 +0000
ch4 semver:          0.7.0
git hash:            n/a
compiler:            rustc
rustc:               rustc 1.55.0 (c8dfcfe04 2021-09-06)
cargo features:      net_tokio, tokio
cargo profile:       debug
cargo target:        x86_64-pc-windows-msvc
endianness:          little
pointer width:       64
build system name:   Windows
build os version:    Windows Server 2019 Datacenter
build cpu vendor:    GenuineIntel
build cpu brand:     Intel(R) Xeon(R) CPU E5-2673 v4 @ 2.30GHz
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
