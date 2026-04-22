use zyn::TokenStream;

pub fn render(is_async: bool) -> TokenStream {
    zyn::zyn! {
        use crate::{args::Args, fmt::Format};
        use anyhow::Result;
        use rsdns::records::Class;
        use std::time::SystemTime;

        @if (is_async) {
            cfg_if::cfg_if! {
                if #[cfg(feature = "net-tokio")] {
                    use rsdns::clients::tokio::Client;
                } else if #[cfg(feature = "net-async-std")] {
                    use rsdns::clients::async_std::Client;
                } else if #[cfg(feature = "net-smol")] {
                    use rsdns::clients::smol::Client;
                } else {
                    compile_error!("One of the async net features must be enabled!!!");
                }
            }
        }
        @else {
            use rsdns::clients::std::Client;
        }

        pub @if (is_async) { async } fn main() -> Result<()> {
            let mut buf = [0u8; u16::MAX as usize];

            let args = Args::get()?;
            let mut format = Format::new(&args);
            if args.has_read_path() {
                return format.read();
            }

            let mut client = Client::new(args.config.clone()) @if (is_async) { .await } ?;

            for qname in args.qnames.iter() {
                let now = SystemTime::now();
                let size = client
                    .query_raw(qname, args.qtype(), Class::IN, &mut buf) @if (is_async) { .await } ?;
                let elapsed = now.elapsed().expect("time failed");
                format.add(
                    Some(qname),
                    Some(args.qtype()),
                    &buf[..size],
                    Some(args.config.nameserver()),
                    Some(now),
                    Some(elapsed),
                )?;
            }

            format.done()?;

            Ok(())
        }
    }
    .into()
}
