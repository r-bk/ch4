use crate::{args::Args, fmt::{zone::Output, rrset, rdata}};
use anyhow::{bail, Result};
use rsdns::{constants::{RClass, RType}, records::data::{self, RData}};
use std::{fmt::Write, time::SystemTime};

{% if async == "true" %}
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
use rsdns::resolvers::std::Resolver;
{% endif %}

{% if async == "true" %}async{% endif %} fn query_rrset_impl<D: RData>(r: &mut Resolver, qname: &str) -> Result<()>
where
    rdata::RDataFmt: rdata::RDataFormatter<String, D>
{
    let rrset = r.query_rrset::<D>(qname, RClass::In){% if async == "true" %}.await{% endif %}?;
    let mut buf = String::new();
    writeln!(&mut buf, "QName: {}", qname)?;
    rrset::fmt(&mut buf, &rrset)?;
    println!("{}", buf);
    Ok(())
}


{% if async == "true" %}async{% endif %} fn query_rrset(r: &mut Resolver, qtype: RType, qname: &str) -> Result<()> {
    match qtype {
        RType::A => query_rrset_impl::<data::A>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Ns => query_rrset_impl::<data::Ns>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Md => query_rrset_impl::<data::Md>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Mf => query_rrset_impl::<data::Mf>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Cname => query_rrset_impl::<data::Cname>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Soa => query_rrset_impl::<data::Soa>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Mb => query_rrset_impl::<data::Mb>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Mg => query_rrset_impl::<data::Mg>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Mr => query_rrset_impl::<data::Mr>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Null => query_rrset_impl::<data::Null>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Wks => query_rrset_impl::<data::Wks>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Ptr => query_rrset_impl::<data::Ptr>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Hinfo => query_rrset_impl::<data::Hinfo>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Minfo => query_rrset_impl::<data::Minfo>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Mx => query_rrset_impl::<data::Mx>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Txt => query_rrset_impl::<data::Txt>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Aaaa => query_rrset_impl::<data::Aaaa>(r, qname){% if async == "true" %}.await{% endif %},
        RType::Axfr | RType::Mailb | RType::Maila | RType::Any => bail!("invalid qtype"),
    }
}

pub {% if async == "true" %}async{% endif %} fn main() -> Result<()> {
    let mut buf = [0u8; u16::MAX as usize];

    let args = Args::get();
    let (conf, qtype, qnames) = args.parse()?;

    let mut resolver = Resolver::new(conf.clone()){% if async == "true" %}.await{% endif %}?;

    for (index, qname) in qnames.iter().enumerate() {
        if !args.rrset {
            let now = SystemTime::now();
            let size = resolver
                .query_raw(qname, qtype, RClass::In, &mut buf){% if async == "true" %}.await{% endif %}?;
            let elapsed = now.elapsed().expect("time failed");

            let output = Output::new(&args, qname, qtype, &buf[..size], now, elapsed, &conf);
            output.print()?;
            if index < qnames.len() - 1 {
                println!();
            }
        } else {
            query_rrset(&mut resolver, qtype, qname){% if async == "true" %}.await{% endif %}?;
        }
    }

    Ok(())
}
