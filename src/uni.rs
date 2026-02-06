use anyhow::Result;
use std::{
    io::{BufRead, BufReader},
    net::IpAddr,
    str::FromStr,
};

pub fn get_dns_servers() -> Result<Vec<IpAddr>> {
    let mut addr_list = Vec::new();

    let f = std::fs::File::open("/etc/resolv.conf")?;
    for line in BufReader::new(f).lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        if let Some(conf_option) = parts.next() {
            match conf_option {
                "nameserver" => {
                    if let Some(address) = parts.next()
                        && let Ok(ip_addr) = IpAddr::from_str(address)
                    {
                        addr_list.push(ip_addr);
                    }
                }
                _ => continue,
            }
        }
    }

    Ok(addr_list)
}
