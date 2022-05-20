use crate::{
    args::Args,
    fmt::rdata::{RDataFmt, RDataFormatter},
};
use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use rsdns::{
    constants::{RecordsSection, Type},
    message::{reader::MessageReader, Header, RCodeValue},
    names::InlineName,
    records::{data::*, Opt},
};
use std::{
    convert::TryFrom,
    fmt::Write,
    net::SocketAddr,
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
    rdlen: usize,
}

#[allow(dead_code)]
pub struct Output<'a, 'b> {
    args: &'a Args,
    msg: &'b [u8],
    ns: Option<SocketAddr>,
    ts: Option<SystemTime>,
    elapsed: Option<Duration>,
    sizes: Sizes,
    opt: Option<Opt>,
}

macro_rules! fmt_size {
    ($itm:expr, $buf:ident) => {{
        $buf.clear();
        write!(&mut $buf, "{}", $itm)?;
        $buf.len()
    }};
}

impl<'a, 'b> Output<'a, 'b> {
    pub fn new(
        args: &'a Args,
        msg: &'b [u8],
        ns: Option<SocketAddr>,
        ts: Option<SystemTime>,
        elapsed: Option<Duration>,
    ) -> Result<Self> {
        let (sizes, opt) = Self::scan_message(msg)?;
        Ok(Self {
            args,
            msg,
            ns,
            ts,
            elapsed,
            sizes,
            opt,
        })
    }

    fn scan_message(msg: &[u8]) -> Result<(Sizes, Option<Opt>)> {
        let mut sizes = Sizes::default();
        let mut opt = None;
        let mut buf = String::new();
        let mut mr = MessageReader::new(msg)?;
        mr.header()?;
        let q = mr.the_question()?;
        sizes.name = sizes.name.max(q.qname.len());
        sizes.rclass = sizes.rclass.max(fmt_size!(q.qclass, buf));
        sizes.rtype = sizes.rtype.max(fmt_size!(q.qtype, buf));

        while mr.has_records() {
            let header = mr.record_header::<InlineName>()?;

            if header.section() == RecordsSection::Additional && header.rtype() == Type::Opt {
                opt = Some(mr.opt_record(header.marker())?);
            } else {
                sizes.name = sizes.name.max(header.name().len());
                sizes.rclass = sizes.rclass.max(fmt_size!(header.rclass(), buf));
                sizes.rtype = sizes.rtype.max(fmt_size!(header.rtype(), buf));
                sizes.ttl = sizes.ttl.max(fmt_size!(header.ttl(), buf));
                sizes.rdlen = sizes.rdlen.max(fmt_size!(header.rdlen(), buf));

                mr.skip_record_data(header.marker())?;
            }
        }

        sizes.name = DOMAIN_NAME_WIDTH.max(sizes.name + 2);
        sizes.rtype = QTYPE_WIDTH.max(sizes.rtype + 1);
        sizes.rclass = QCLASS_WIDTH.max(sizes.rclass + 1);
        sizes.ttl = TTL_WIDTH.max(sizes.ttl + 1);

        Ok((sizes, opt))
    }

    pub fn print(&self) -> Result<()> {
        self.print_header();
        self.print_message()?;
        self.print_footer();
        Ok(())
    }

    fn print_message(&self) -> Result<()> {
        let mut mr = MessageReader::new(self.msg)?;
        let header = mr.header()?;
        println!("{}", self.format_response_header(&header)?);
        if self.opt.is_some() {
            print!("{}", self.format_opt()?);
        }
        println!("{}", self.format_question(&mut mr)?);
        println!("{}", self.format_records(&mut mr, &header)?);
        Ok(())
    }

    fn format_response_header(&self, header: &Header) -> Result<String> {
        let mut output = String::new();
        let status = if let Some(ref o) = self.opt {
            RCodeValue::extended(header.flags.response_code(), o.rcode_extension())
        } else {
            header.flags.response_code()
        };
        writeln!(
            &mut output,
            ";; ->>HEADER<<- opcode: {}, status: {}, id: {}",
            header.flags.opcode(),
            status,
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

    fn format_question(&self, mr: &mut MessageReader) -> Result<String> {
        let mut output = String::new();
        writeln!(&mut output, ";; QUESTION SECTION:")?;

        while mr.has_questions() {
            let q = mr.question()?;
            write!(
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

    fn format_records(&self, mr: &mut MessageReader, header: &Header) -> Result<String> {
        let mut output = String::new();
        let mut section = None;

        while mr.has_records() {
            let rec_header = mr.record_header::<InlineName>()?;
            let sec = rec_header.section();

            if section != Some(sec) {
                section = Some(sec);
                if sec != RecordsSection::Additional || self.opt.is_none() || header.ar_count > 1 {
                    writeln!(&mut output, "\n;; {} SECTION:", sec.to_str().to_uppercase())?;
                }
            }

            if sec == RecordsSection::Additional && rec_header.rtype() == Type::Opt {
                mr.skip_record_data(rec_header.marker())?;
                continue;
            }

            write!(
                &mut output,
                "{:dn_width$}{:<ttl_width$}{:qc_width$}{:qt_width$}",
                rec_header.name(),
                rec_header.ttl(),
                rec_header.rclass(),
                rec_header.rtype(),
                dn_width = self.sizes.name,
                ttl_width = self.sizes.ttl,
                qc_width = self.sizes.rclass,
                qt_width = self.sizes.rtype,
            )?;

            let rtype = if self.args.format.is_rfc3597() {
                Type::Any // use a type that forces RFC 3597 below
            } else {
                match Type::try_from(rec_header.rtype()) {
                    Ok(rt) => rt,
                    _ => Type::Any, // use a type that forces RFC 3597 below
                }
            };

            match rtype {
                Type::A => {
                    let a = mr.record_data::<A>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &a)?;
                }
                Type::Aaaa => {
                    let aaaa = mr.record_data::<Aaaa>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &aaaa)?;
                }
                Type::Cname => {
                    let cname = mr.record_data::<Cname>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &cname)?;
                }
                Type::Ns => {
                    let ns = mr.record_data::<Ns>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &ns)?;
                }
                Type::Soa => {
                    let soa = mr.record_data::<Soa>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &soa)?;
                }
                Type::Ptr => {
                    let ptr = mr.record_data::<Ptr>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &ptr)?;
                }
                Type::Mx => {
                    let mx = mr.record_data::<Mx>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &mx)?;
                }
                Type::Txt => {
                    let txt = mr.record_data::<Txt>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &txt)?;
                }
                Type::Hinfo => {
                    let hinfo = mr.record_data::<Hinfo>(rec_header.marker())?;
                    RDataFmt::fmt(&mut output, &hinfo)?;
                }
                Type::Wks
                | Type::Null
                | Type::Mr
                | Type::Mg
                | Type::Mb
                | Type::Mf
                | Type::Minfo
                | Type::Md
                | Type::Opt
                | Type::Axfr
                | Type::Maila
                | Type::Mailb
                | Type::Any => {
                    let bytes = mr.record_data_bytes(rec_header.marker())?;
                    write!(&mut output, "{}", self.format_rfc_3597(bytes)?)?;
                }
            }

            writeln!(&mut output)?;
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

    fn format_rfc_3597(&self, d: &[u8]) -> Result<String> {
        let mut output = String::new();

        write!(
            &mut output,
            "\\# {:<rdlen$}",
            d.len(),
            rdlen = self.sizes.rdlen
        )?;

        for chunk in d.chunks(4) {
            write!(&mut output, " ")?;
            for b in chunk {
                write!(&mut output, "{:02x}", *b)?;
            }
        }

        Ok(output)
    }

    fn print_header(&self) {
        println!(
            "; <<>> ch4 {} <<>> {}",
            env!("CH4_VERSION"),
            self.args.cmd_line()
        );
    }

    fn format_opt(&self) -> Result<String> {
        let mut output = String::new();

        if let Some(ref opt) = self.opt {
            let mut flags = "";
            if opt.dnssec_ok() {
                flags = " d0";
            }
            writeln!(&mut output, ";; OPT PSEUDOSECTION:")?;
            writeln!(
                &mut output,
                "; EDNS: version: {}, flags:{}; udp: {}",
                opt.version(),
                flags,
                opt.udp_payload_size(),
            )?;
            Ok(output)
        } else {
            bail!("no opt record present");
        }
    }

    fn print_footer(&self) {
        if let Some(elapsed) = self.elapsed {
            println!(";; Query time: {:?}", elapsed);
        }
        if let Some(ns) = self.ns {
            println!(";; SERVER: {}", ns);
        }
        if let Some(ts) = self.ts {
            let datetime: DateTime<Local> = DateTime::from(ts);
            println!(";; WHEN: {}", datetime.to_rfc2822());
        }
        println!(";; MSG SIZE rcvd: {}", self.msg.len());
    }
}
