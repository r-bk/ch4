# 0.3.1 (Aug 27, 2021)

- fix `@` nameserver prefix example in the README file
- add a link to the CHANGELOG in the README file

# 0.3.0 (Aug 27, 2021)

- `--rrset` flag was removed
- `+[no]short` flag was added, for compatibility with `dig`. When enabled, `ch4` performs
  CNAME flattening to show the requested record type.
- `-p, --port` option was fixed on Windows
- dependencies were updated

# 0.2.0 (Aug 13, 2021)

- update to `rsdns 0.3.0`
- support leading digits in domain names (`rfc1101`)
- add installation instructions in README

# 0.1.0 (Aug 13, 2021)

- First release
