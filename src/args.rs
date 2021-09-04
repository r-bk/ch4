use anyhow::Result;
use rsdns::{
    constants::Type,
    resolvers::{ProtocolStrategy, Recursion, ResolverConfig},
};
use std::{
    net::{IpAddr, SocketAddr},
    process::exit,
    str::FromStr,
    time::Duration,
};
use structopt::StructOpt;

#[allow(dead_code)]
pub mod bi {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, StructOpt)]
#[structopt(about = "DNS Stub Resolver", version = env!("CH4_VERSION"))]
pub struct Args {
    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    #[structopt(short, long)]
    bind_device: Option<String>,

    #[structopt(short, long, default_value = "53")]
    port: u16,

    #[structopt(
        short = "l",
        long,
        default_value = "10000",
        help = "query lifetime (in msec)."
    )]
    query_lifetime: u64,

    #[structopt(
        short = "t",
        long,
        default_value = "2000",
        help = "query timeout (in msec). Use 0 to disable."
    )]
    query_timeout: u64,

    #[structopt(long, help = "Prints build information")]
    info: bool,

    #[structopt(skip)]
    pub short: bool,

    #[structopt(long, help = "Lists system nameservers")]
    list_nameservers: bool,

    #[structopt(verbatim_doc_comment)]
    /// Positional arguments ...
    ///
    /// Positional arguments may be specified without any particular order.
    /// Arguments specified later take precedence.
    /// Arguments that are not recognized as special are treated as names
    /// to be queried.
    ///
    ///
    /// @<nameserver> - specifies the nameserver IP address.
    ///                 If not specified, the first nameserver from OS
    ///                 configuration is used.
    ///
    /// <qtype>       - query type (A, AAAA, NS ...).
    ///                 An argument matching any of the supported query types
    ///                 is considered as query type. Trailing dot can be
    ///                 used to disambiguate a query name (A.).
    ///
    /// +udp          - sets the Udp protocol strategy. UDP is used by default.
    ///                 Truncated responses are retried using TCP.
    ///
    /// +tcp          - sets the Tcp protocol strategy.
    ///                 Only TCP is used for all queries.
    ///
    /// +notcp        - sets NoTcp protocol strategy. Only UDP is used.
    ///                 Truncated queries are returned as is, without retry.
    ///
    /// +[no]rec      - enables (disables) recursive query.
    ///                 Queries are recursive by default.
    ///
    /// +[no]short    - enables (disables) short output.
    ///                 When enabled, only record data is printed,
    ///                 one record on a line.
    pub positional: Vec<String>,
}

impl Args {
    pub fn get() -> Result<Args> {
        let args = Args::from_args();

        if args.info {
            Args::show_info();
            exit(0);
        }

        if args.list_nameservers {
            Args::list_nameservers()?;
            exit(0);
        }

        Ok(args)
    }

    fn show_info() {
        println!("build time:          {}", bi::BUILT_TIME_UTC);
        println!("ch4 semver:          {}", bi::PKG_VERSION);
        println!(
            "git hash:            {}",
            bi::GIT_COMMIT_HASH.or(Some("n/a")).unwrap()
        );

        println!("compiler:            {}", bi::RUSTC);
        println!("rustc:               {}", bi::RUSTC_VERSION);

        println!("cargo features:      {}", bi::FEATURES_STR.to_lowercase());
        println!("cargo profile:       {}", bi::PROFILE);
        println!("cargo target:        {}", bi::TARGET);
        println!("endianness:          {}", bi::CFG_ENDIAN);
        println!("pointer width:       {}", bi::CFG_POINTER_WIDTH);

        println!("build system name:   {}", env!("CH4_SYSINFO_NAME"));
        println!("build os version:    {}", env!("CH4_SYSINFO_OS_VERSION"));
        println!("build cpu vendor:    {}", env!("CH4_SYSINFO_CPU_VENDOR"));
        println!("build cpu brand:     {}", env!("CH4_SYSINFO_CPU_BRAND"));
    }

    fn list_nameservers() -> Result<()> {
        let dns_servers = crate::os_nameservers()?;
        for addr in dns_servers.iter() {
            println!("{}", addr);
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<(ResolverConfig, Type, Vec<String>)> {
        let mut protocol_strategy = ProtocolStrategy::Udp;
        let mut nameserver_ip_addr: Option<IpAddr> = None;
        let mut recursion = Recursion::On;
        let mut short = false;
        let mut free_args = Vec::new();
        let mut qtype = Type::A;

        for a in self.positional.iter() {
            match a.as_str() {
                "+udp" => protocol_strategy = ProtocolStrategy::Udp,
                "+tcp" => protocol_strategy = ProtocolStrategy::Tcp,
                "+notcp" => protocol_strategy = ProtocolStrategy::NoTcp,
                "+rec" => recursion = Recursion::On,
                "+norec" => recursion = Recursion::Off,
                "+short" => short = true,
                "+noshort" => short = false,
                s if s.starts_with('@') => match IpAddr::from_str(&s[1..]) {
                    Ok(addr) => nameserver_ip_addr = Some(addr),
                    Err(_) => {
                        eprintln!("failed to parse nameserver ip address");
                        exit(1);
                    }
                },
                s if Type::from_str(&s.to_uppercase()).is_ok() => {
                    qtype = Type::from_str(&s.to_uppercase()).unwrap()
                }
                _ => free_args.push(a.clone()),
            }
        }

        self.short = short;

        if !qtype.is_data_type() && qtype != Type::Any {
            eprintln!("only data-type queries are supported or ANY: {}", qtype);
            exit(1);
        }

        let nameserver = match nameserver_ip_addr {
            Some(addr) => SocketAddr::from((addr, self.port)),
            None => {
                if let Ok(v) = crate::os_nameservers() {
                    SocketAddr::from((v[0], self.port))
                } else {
                    eprintln!("no nameservers");
                    exit(1);
                }
            }
        };

        #[allow(unused_mut)]
        let mut conf = ResolverConfig::with_nameserver(nameserver)
            .set_protocol_strategy(protocol_strategy)
            .set_recursion(recursion)
            .set_query_timeout(if self.query_timeout > 0 {
                Some(Duration::from_millis(self.query_timeout))
            } else {
                None
            })
            .set_query_lifetime(Duration::from_millis(self.query_lifetime));

        #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
        if let Some(ref bd) = self.bind_device {
            conf = conf.set_bind_device(Some(bd))?;
        }

        Ok((conf, qtype, free_args))
    }
}
