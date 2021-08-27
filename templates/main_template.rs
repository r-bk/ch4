use crate::{args::Args, fmt::{zone::Output, rrset, rdata}};
use anyhow::{bail, Result};
use rsdns::{constants::{RClass, RType}, records::data::{self, RData}};
use std::time::SystemTime;

{% if async == "true" %}
{% set as = "async"  %}
{% set aw = ".await" %}
cfg_if::cfg_if! {
    if #[cfg(feature = "net-tokio")] {
        use rsdns::resolvers::tokio::Resolver;
    } else if #[cfg(feature = "net-async-std")] {
        use rsdns::resolvers::async_std::Resolver;
    } else if #[cfg(feature = "net-smol")] {
        use rsdns::resolvers::smol::Resolver;
    } else {
        compile_error!("One of the async net features must be enabled!!!");
    }
}
{% else %}
{% set as = "" %}
{% set aw = "" %}
use rsdns::resolvers::std::Resolver;
{% endif %}

{{ as }} fn query_rrset_impl<D: RData>(r: &mut Resolver, qname: &str) -> Result<()>
where
    rdata::RDataFmt: rdata::RDataFormatter<String, D>
{
    let rrset = r.query_rrset::<D>(qname, RClass::In){{ aw }}?;
    let mut buf = String::new();
    rrset::fmt_short(&mut buf, &rrset)?;
    print!("{}", buf);
    Ok(())
}


{{ as }} fn query_rrset(r: &mut Resolver, qtype: RType, qname: &str) -> Result<()> {
    match qtype {
        RType::A => query_rrset_impl::<data::A>(r, qname){{ aw }},
        RType::Ns => query_rrset_impl::<data::Ns>(r, qname){{ aw }},
        RType::Md => query_rrset_impl::<data::Md>(r, qname){{ aw }},
        RType::Mf => query_rrset_impl::<data::Mf>(r, qname){{ aw }},
        RType::Cname => query_rrset_impl::<data::Cname>(r, qname){{ aw }},
        RType::Soa => query_rrset_impl::<data::Soa>(r, qname){{ aw }},
        RType::Mb => query_rrset_impl::<data::Mb>(r, qname){{ aw }},
        RType::Mg => query_rrset_impl::<data::Mg>(r, qname){{ aw }},
        RType::Mr => query_rrset_impl::<data::Mr>(r, qname){{ aw }},
        RType::Null => query_rrset_impl::<data::Null>(r, qname){{ aw }},
        RType::Wks => query_rrset_impl::<data::Wks>(r, qname){{ aw }},
        RType::Ptr => query_rrset_impl::<data::Ptr>(r, qname){{ aw }},
        RType::Hinfo => query_rrset_impl::<data::Hinfo>(r, qname){{ aw }},
        RType::Minfo => query_rrset_impl::<data::Minfo>(r, qname){{ aw }},
        RType::Mx => query_rrset_impl::<data::Mx>(r, qname){{ aw }},
        RType::Txt => query_rrset_impl::<data::Txt>(r, qname){{ aw }},
        RType::Aaaa => query_rrset_impl::<data::Aaaa>(r, qname){{ aw }},
        RType::Axfr | RType::Mailb | RType::Maila | RType::Any => bail!("invalid qtype"),
    }
}

pub {{ as }} fn main() -> Result<()> {
    let mut buf = [0u8; u16::MAX as usize];

    let mut args = Args::get()?;
    let (conf, qtype, qnames) = args.parse()?;

    let mut resolver = Resolver::new(conf.clone()){{ aw }}?;

    for (index, qname) in qnames.iter().enumerate() {
        if !args.short || qtype.is_meta_type() {
            let now = SystemTime::now();
            let size = resolver
                .query_raw(qname, qtype, RClass::In, &mut buf){{ aw }}?;
            let elapsed = now.elapsed().expect("time failed");

            let output = Output::new(&args, qname, qtype, &buf[..size], now, elapsed, &conf)?;
            output.print()?;
            if qnames.len() > 1 && index < qnames.len() - 1 {
                println!();
            }
        } else {
            query_rrset(&mut resolver, qtype, qname){{ aw }}?;
        }
    }

    Ok(())
}
