use crate::args::Args;
use anyhow::Result;
use chrono::{DateTime, Local};
use rsdns::{
    constants::RType,
    message::{reader::MessageReader, Header},
    resolvers::ResolverConfig,
};
use std::{
    fmt::Write,
    time::{Duration, SystemTime},
};

const DOMAIN_NAME_WIDTH: usize = 24;
const QCLASS_WIDTH: usize = 7;
const QTYPE_WIDTH: usize = 7;
const TTL_WIDTH: usize = 7;

#[derive(Debug, Default, Copy, Clone)]
struct Sizes {
    name: usize,
    ttl: usize,
    rclass: usize,
    rtype: usize,
}

#[allow(dead_code)]
pub struct Output<'a, 'b, 'c, 'd> {
    args: &'a Args,
    qname: &'b str,
    qtype: RType,
    msg: &'c [u8],
    ts: SystemTime,
    elapsed: Duration,
    resolver_conf: &'d ResolverConfig,
    sizes: Sizes,
}

macro_rules! fmt_size {
    ($itm:expr, $buf:ident) => {{
        $buf.clear();
        write!(&mut $buf, "{}", $itm)?;
        $buf.len()
    }};
}

impl<'a, 'b, 'c, 'd> Output<'a, 'b, 'c, 'd> {
    pub fn new(
        args: &'a Args,
        qname: &'b str,
        qtype: RType,
        msg: &'c [u8],
        ts: SystemTime,
        elapsed: Duration,
        resolver_conf: &'d ResolverConfig,
    ) -> Result<Self> {
        let sizes = Self::find_sizes(msg)?;
        Ok(Self {
            args,
            qname,
            qtype,
            msg,
            ts,
            elapsed,
            resolver_conf,
            sizes,
        })
    }

    fn find_sizes(msg: &[u8]) -> Result<Sizes> {
        let mut sizes = Sizes::default();
        let mut buf = String::new();
        let mr = MessageReader::new(msg)?;
        let q = mr.question()?;
        sizes.name = sizes.name.max(q.qname.len());
        sizes.rclass = sizes.rclass.max(fmt_size!(q.qclass, buf));
        sizes.rtype = sizes.rtype.max(fmt_size!(q.qtype, buf));

        for res in mr.records() {
            let (_, rec) = res?;
            sizes.name = sizes.name.max(rec.name.len());
            sizes.rclass = sizes.rclass.max(fmt_size!(rec.rclass, buf));
            sizes.rtype = sizes.rtype.max(fmt_size!(rec.rtype, buf));
            sizes.ttl = sizes.ttl.max(fmt_size!(rec.ttl, buf));
        }

        sizes.name = DOMAIN_NAME_WIDTH.max(sizes.name + 2);
        sizes.rtype = QTYPE_WIDTH.max(sizes.rtype + 1);
        sizes.rclass = QCLASS_WIDTH.max(sizes.rclass + 1);
        sizes.ttl = TTL_WIDTH.max(sizes.ttl + 1);

        Ok(sizes)
    }

    pub fn print(&self) -> Result<()> {
        self.print_header();
        self.print_message()?;
        self.print_footer();
        Ok(())
    }

    fn print_message(&self) -> Result<()> {
        let mr = MessageReader::new(self.msg)?;
        println!("{}", Self::format_response_header(mr.header())?);
        println!("{}", self.format_question(&mr)?);
        Ok(())
    }

    fn format_response_header(header: &Header) -> Result<String> {
        let mut output = String::new();
        writeln!(
            &mut output,
            ";; ->>HEADER<<- opcode: {}, status: {}, id: {}",
            header.flags.opcode(),
            header.flags.response_code(),
            header.id,
        )?;
        writeln!(
            &mut output,
            ";; flags: {}; QUERY: {}, ANSWER: {}, AUTHORITY: {}, ADDITIONAL: {}",
            Self::format_flags(header),
            header.qd_count,
            header.an_count,
            header.ns_count,
            header.ar_count,
        )?;
        Ok(output)
    }

    fn format_question(&self, mr: &MessageReader) -> Result<String> {
        let mut output = String::new();
        writeln!(&mut output, ";; QUESTION SECTION:")?;

        #[allow(clippy::for_loops_over_fallibles)]
        for q in mr.questions() {
            let q = q?;
            writeln!(
                &mut output,
                ";{:dn_width$}{:ttl_width$}{:qc_width$}{:qt_width$}",
                q.qname,
                " ",
                q.qclass,
                q.qtype,
                dn_width = self.sizes.name - 1,
                ttl_width = self.sizes.ttl,
                qc_width = self.sizes.rclass,
                qt_width = self.sizes.rtype,
            )?;
        }
        Ok(output)
    }

    fn format_flags(header: &Header) -> String {
        let mut flags_str = Vec::new();

        if header.flags.message_type().is_response() {
            flags_str.push("qr");
        }
        if header.flags.authoritative_answer() {
            flags_str.push("aa");
        }
        if header.flags.truncated() {
            flags_str.push("tc");
        }
        if header.flags.recursion_desired() {
            flags_str.push("rd");
        }
        if header.flags.recursion_available() {
            flags_str.push("ra");
        }

        flags_str.join(" ")
    }

    fn print_header(&self) {
        println!(
            "; <<>> ch4 {} <<>> {} {}",
            env!("CH4_VERSION"),
            self.qtype,
            self.qname,
        );
    }

    fn print_footer(&self) {
        let datetime: DateTime<Local> = DateTime::from(self.ts);
        println!(";; Query time: {:?}", self.elapsed);
        println!(";; SERVER: {}", self.resolver_conf.nameserver());
        println!(";; WHEN: {}", datetime.to_rfc2822());
        println!(";; MSG SIZE rcvd: {}", self.msg.len());
    }
}
