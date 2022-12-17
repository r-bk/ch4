use anyhow::Result;
use rsdns::{
    clients::{ClientConfig, EDns, ProtocolStrategy, Recursion},
    constants::Type,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutputFormat {
    Zone,
    ZoneRfc3597,
    Short,
    Rust,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "DNS Client", version = env!("CH4_VERSION"))]
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

    #[structopt(long, help = "Lists system nameservers")]
    list_nameservers: bool,

    #[structopt(skip)]
    pub format: OutputFormat,

    #[structopt(skip)]
    pub config: ClientConfig,

    #[structopt(skip)]
    pub qtype: Option<Type>,

    #[structopt(skip)]
    pub qnames: Vec<String>,

    #[structopt(skip)]
    pub nameservers: Vec<String>,

    #[structopt(short = "s", long = "save", help = "save responses to file")]
    pub save_path: Option<String>,

    #[structopt(short = "r", long = "read", help = "read responses from file")]
    pub read_path: Option<String>,

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
    ///
    /// +bufsize=#    - sets the EDNS0 max udp payload size [512, 65535].
    ///                 [default: 4096]
    ///
    /// +[no]edns[=#] - enables/disables EDNS0.
    ///                 Optionally, sets the EDNS version [0, 255].
    ///                 By default, EDNS is enabled with version 0.
    ///
    /// +[no]rust     - enables (disables) rust output.
    ///                 When enabled, prints the response as a Rust array.
    ///
    /// +[no]gen      - forces generic output (RFC 3597 s. 5) on all record
    ///                 types. By default, only unknown record types are
    ///                 formatted this way.
    pub positional: Vec<String>,
}

impl Args {
    pub fn get() -> Result<Args> {
        let mut args = Args::from_args();

        if args.info {
            Args::show_info();
            exit(0);
        }

        if args.list_nameservers {
            Args::list_nameservers()?;
            exit(0);
        }

        args.parse()?;

        Ok(args)
    }

    pub fn cmd_line(&self) -> String {
        let pfx = if self.has_save_path() {
            format!("--save {} ", self.save_path.as_deref().unwrap())
        } else if self.has_read_path() {
            format!("--read {} ", self.read_path.as_deref().unwrap())
        } else {
            "".to_string()
        };
        format!("{}{}", pfx, self.positional.join(" "))
    }

    pub fn has_save_path(&self) -> bool {
        if let Some(ref path) = self.save_path {
            !path.is_empty()
        } else {
            false
        }
    }

    pub fn has_read_path(&self) -> bool {
        if let Some(ref path) = self.read_path {
            !path.is_empty()
        } else {
            false
        }
    }

    pub fn qtype(&self) -> Type {
        self.qtype.unwrap()
    }

    fn show_info() {
        println!("build time:          {}", bi::BUILT_TIME_UTC);
        println!("ch4 semver:          {}", bi::PKG_VERSION);
        println!(
            "git hash:            {}",
            bi::GIT_COMMIT_HASH.unwrap_or("n/a")
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
            println!("{addr}");
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<()> {
        let mut protocol_strategy = ProtocolStrategy::Udp;
        let mut nameserver_ip_addr: Option<IpAddr> = None;
        let mut recursion = Recursion::On;
        let mut qnames = Vec::new();
        let mut qtype = Type::A;
        let mut format = OutputFormat::Zone;
        let mut edns_enabled = true;
        let mut edns_version: u8 = 0;
        let mut edns_udp_payload_size: u16 = 4096;

        for a in self.positional.iter() {
            match a.as_str() {
                "+udp" => protocol_strategy = ProtocolStrategy::Udp,
                "+tcp" => protocol_strategy = ProtocolStrategy::Tcp,
                "+notcp" => protocol_strategy = ProtocolStrategy::NoTcp,
                "+rec" => recursion = Recursion::On,
                "+norec" => recursion = Recursion::Off,
                "+short" => format = OutputFormat::Short,
                "+noshort" => format = OutputFormat::Zone,
                "+rust" => format = OutputFormat::Rust,
                "+norust" => format = OutputFormat::Zone,
                "+gen" => format = OutputFormat::ZoneRfc3597,
                "+nogen" => format = OutputFormat::Zone,
                "+noedns" => edns_enabled = false,
                "+edns" => {
                    edns_enabled = true;
                    edns_version = 0
                }
                s if s.starts_with("+edns=") => {
                    edns_enabled = true;
                    edns_version = get_param_val(s)
                }
                s if s.starts_with("+bufsize=") => edns_udp_payload_size = get_param_val(s),
                s if s.starts_with('@') => match IpAddr::from_str(&s[1..]) {
                    Ok(addr) => {
                        self.nameservers.push(s[1..].to_string());
                        nameserver_ip_addr = Some(addr)
                    }
                    Err(_) => {
                        eprintln!("failed to parse nameserver ip address");
                        exit(1);
                    }
                },
                s if Type::from_str(&s.to_uppercase()).is_ok() => {
                    qtype = Type::from_str(&s.to_uppercase()).unwrap()
                }
                s => {
                    if s.starts_with('+') {
                        eprintln!("bad option: {s}");
                        exit(1);
                    }
                    qnames.push(a.clone())
                }
            }
        }

        self.format = format;

        if qtype == Type::Opt || (!qtype.is_data_type() && qtype != Type::Any) {
            eprintln!("only data-type queries are supported or ANY: {qtype}");
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

        let edns = if edns_enabled {
            EDns::On {
                version: edns_version,
                udp_payload_size: edns_udp_payload_size,
            }
        } else {
            EDns::Off
        };

        #[allow(unused_mut)]
        let mut config = ClientConfig::with_nameserver(nameserver)
            .set_protocol_strategy(protocol_strategy)
            .set_recursion(recursion)
            .set_query_timeout(if self.query_timeout > 0 {
                Some(Duration::from_millis(self.query_timeout))
            } else {
                None
            })
            .set_query_lifetime(Duration::from_millis(self.query_lifetime))
            .set_edns(edns);

        #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
        if let Some(ref bd) = self.bind_device {
            config = config.set_bind_device(Some(bd))?;
        }

        self.config = config;
        self.qtype = Some(qtype);
        self.qnames = qnames;

        Ok(())
    }
}

fn get_param_val<T: FromStr>(s: &str) -> T {
    if let Some(p) = s.split('=').nth(1) {
        if let Ok(v) = T::from_str(p) {
            return v;
        }
    }
    eprintln!("bad option: {s}");
    exit(1);
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Zone
    }
}

impl OutputFormat {
    pub fn is_rfc3597(self) -> bool {
        self == OutputFormat::ZoneRfc3597
    }
}
