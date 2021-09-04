use crate::{args::Args, fmt::Format};
use anyhow::Result;
use rsdns::constants::Class;
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


pub {{ as }} fn main() -> Result<()> {
    let mut buf = [0u8; u16::MAX as usize];

    let args = Args::get()?;
    let mut format = Format::new(&args);
    let mut resolver = Resolver::new(args.config.clone()){{ aw }}?;

    for qname in args.qnames.iter() {
        let now = SystemTime::now();
        let size = resolver
            .query_raw(qname, args.qtype(), Class::In, &mut buf){{ aw }}?;
        let elapsed = now.elapsed().expect("time failed");
        format.add(qname, &buf[..size], now, elapsed)?;
    }

    format.done()?;

    Ok(())
}
