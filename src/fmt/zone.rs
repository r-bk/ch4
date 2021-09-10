use crate::{
    args::Args,
    fmt::rdata::{RDataFmt, RDataFormatter},
};
use anyhow::Result;
use chrono::{DateTime, Local};
use rsdns::{
    message::{reader::MessageReader, Header},
    records::data::RecordData,
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
pub struct Output<'a, 'b, 'c> {
    args: &'a Args,
    qname: &'b str,
    msg: &'c [u8],
    ts: SystemTime,
    elapsed: Duration,
    sizes: Sizes,
}

macro_rules! fmt_size {
    ($itm:expr, $buf:ident) => {{
        $buf.clear();
        write!(&mut $buf, "{}", $itm)?;
        $buf.len()
    }};
}

impl<'a, 'b, 'c> Output<'a, 'b, 'c> {
    pub fn new(
        args: &'a Args,
        qname: &'b str,
        msg: &'c [u8],
        ts: SystemTime,
        elapsed: Duration,
    ) -> Result<Self> {
        let sizes = Self::find_sizes(msg)?;
        Ok(Self {
            args,
            qname,
            msg,
            ts,
            elapsed,
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
        if !self.args.format.is_short() {
            self.print_header();
            self.print_message()?;
            self.print_footer();
        } else {
            self.print_records_short()?;
        }
        Ok(())
    }

    fn print_message(&self) -> Result<()> {
        let mr = MessageReader::new(self.msg)?;
        println!("{}", Self::format_response_header(mr.header())?);
        println!("{}", self.format_question(&mr)?);
        println!("{}", self.format_records(&mr)?);
        Ok(())
    }

    fn print_records_short(&self) -> Result<()> {
        let mr = MessageReader::new(self.msg)?;
        print!("{}", self.format_records(&mr)?);
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

    fn format_records(&self, mr: &MessageReader) -> Result<String> {
        let mut output = String::new();
        let mut section = None;

        for res in mr.records() {
            let (sec, rec) = res?;
            if !self.args.format.is_short() && section != Some(sec) {
                section = Some(sec);
                writeln!(&mut output, "\n;; {} SECTION:", sec.to_str().to_uppercase())?;
            }

            if !self.args.format.is_short() {
                write!(
                    &mut output,
                    "{:dn_width$}{:ttl_width$}{:qc_width$}{:qt_width$}",
                    rec.name,
                    rec.ttl.to_string(),
                    rec.rclass,
                    rec.rtype,
                    dn_width = self.sizes.name,
                    ttl_width = self.sizes.ttl,
                    qc_width = self.sizes.rclass,
                    qt_width = self.sizes.rtype,
                )?;
            }

            match rec.rdata {
                RecordData::A(ref a) => RDataFmt::fmt(&mut output, a)?,
                RecordData::Aaaa(ref aaaa) => RDataFmt::fmt(&mut output, aaaa)?,
                RecordData::Cname(ref cname) => RDataFmt::fmt(&mut output, cname)?,
                RecordData::Ns(ref ns) => RDataFmt::fmt(&mut output, ns)?,
                RecordData::Soa(ref soa) => RDataFmt::fmt(&mut output, soa)?,
                RecordData::Ptr(ref ptr) => RDataFmt::fmt(&mut output, ptr)?,
                RecordData::Mx(ref mx) => RDataFmt::fmt(&mut output, mx)?,
                RecordData::Txt(ref txt) => RDataFmt::fmt(&mut output, txt)?,
                RecordData::Hinfo(ref hinfo) => RDataFmt::fmt(&mut output, hinfo)?,
                RecordData::Wks(_)
                | RecordData::Null(_)
                | RecordData::Mr(_)
                | RecordData::Mg(_)
                | RecordData::Mb(_)
                | RecordData::Mf(_)
                | RecordData::Minfo(_)
                | RecordData::Md(_) => write!(&mut output, "OBSOLETE RTYPE")?,
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

    fn print_header(&self) {
        println!(
            "; <<>> ch4 {} <<>> {}",
            env!("CH4_VERSION"),
            self.args.cmd_line()
        );
    }

    fn print_footer(&self) {
        let datetime: DateTime<Local> = DateTime::from(self.ts);
        println!(";; Query time: {:?}", self.elapsed);
        println!(";; SERVER: {}", self.args.config.nameserver());
        println!(";; WHEN: {}", datetime.to_rfc2822());
        println!(";; MSG SIZE rcvd: {}", self.msg.len());
    }
}
