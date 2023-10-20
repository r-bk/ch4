mod rdata;
mod rrset;
mod rust;
mod save;
mod zone;

//
// ----------------------------------------------------------------------
//

use crate::{
    args::{Args, OutputFormat},
    fmt::save::EncodedMessage,
};
use anyhow::{bail, Result};
use rdata::{RDataFmt, RDataFormatter};
use rsdns::records::{
    data::{self, RData},
    RecordSet, Type,
};
use std::{
    net::SocketAddr,
    time::{Duration, SystemTime},
};

pub struct Format<'a> {
    args: &'a Args,
    cnt: usize,
    json: Vec<serde_json::Value>,
}

impl<'a> Format<'a> {
    pub fn new(args: &'a Args) -> Format<'a> {
        Self {
            args,
            cnt: 0,
            json: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        qname: Option<&str>,
        qtype: Option<Type>,
        msg: &[u8],
        ns: Option<SocketAddr>,
        ts: Option<SystemTime>,
        elapsed: Option<Duration>,
    ) -> Result<()> {
        match self.args.format {
            OutputFormat::Short => self.short(msg)?,
            OutputFormat::Zone | OutputFormat::ZoneRfc3597 => self.zone(msg, ns, ts, elapsed)?,
            OutputFormat::Rust => self.rust(qname, qtype, msg)?,
        };
        if self.args.has_save_path() {
            self.json
                .push(EncodedMessage::encode(msg, qname, qtype, ns, ts, elapsed)?);
        }
        self.cnt += 1;
        Ok(())
    }

    pub fn done(&mut self) -> Result<()> {
        if self.args.has_save_path() && !self.json.is_empty() {
            return EncodedMessage::save_all(&self.json, self.args.save_path.as_ref().unwrap());
        }
        Ok(())
    }

    pub fn read(&mut self) -> Result<()> {
        let read_path = self.args.read_path.as_ref().unwrap();
        let responses = EncodedMessage::load_all(read_path)?;

        for r in responses {
            self.add(
                r.qname(),
                r.qtype(),
                &r.msg(),
                r.nameserver(),
                r.time(),
                r.elapsed(),
            )?;
        }

        Ok(())
    }

    fn short(&self, msg: &[u8]) -> Result<()> {
        let qtype = self.args.qtype();
        match qtype {
            Type::A => Self::short_rrset::<data::A>(msg),
            Type::NS => Self::short_rrset::<data::Ns>(msg),
            Type::MD => Self::short_rrset::<data::Md>(msg),
            Type::MF => Self::short_rrset::<data::Mf>(msg),
            Type::CNAME => Self::short_rrset::<data::Cname>(msg),
            Type::SOA => Self::short_rrset::<data::Soa>(msg),
            Type::MB => Self::short_rrset::<data::Mb>(msg),
            Type::MG => Self::short_rrset::<data::Mg>(msg),
            Type::MR => Self::short_rrset::<data::Mr>(msg),
            Type::NULL => Self::short_rrset::<data::Null>(msg),
            Type::WKS => Self::short_rrset::<data::Wks>(msg),
            Type::PTR => Self::short_rrset::<data::Ptr>(msg),
            Type::HINFO => Self::short_rrset::<data::Hinfo>(msg),
            Type::MINFO => Self::short_rrset::<data::Minfo>(msg),
            Type::MX => Self::short_rrset::<data::Mx>(msg),
            Type::TXT => Self::short_rrset::<data::Txt>(msg),
            Type::AAAA => Self::short_rrset::<data::Aaaa>(msg),
            t if t.is_meta_type() => {
                bail!("invalid qtype: {}", qtype);
            }
            _ => {
                bail!("unsupported qtype: {}", qtype)
            }
        }
    }

    fn short_rrset<D: RData>(msg: &[u8]) -> Result<()>
    where
        RDataFmt: RDataFormatter<String, D>,
    {
        let rr_set = RecordSet::<D>::from_msg(msg)?;
        let mut buf = String::new();
        rrset::fmt_short(&mut buf, &rr_set)?;
        print!("{buf}");
        Ok(())
    }

    fn zone(
        &self,
        msg: &[u8],
        ns: Option<SocketAddr>,
        ts: Option<SystemTime>,
        elapsed: Option<Duration>,
    ) -> Result<()> {
        if self.cnt > 0 {
            println!();
        }
        zone::Output::new(self.args, msg, ns, ts, elapsed)?.print()
    }

    fn rust(&self, qname: Option<&str>, qtype: Option<Type>, msg: &[u8]) -> Result<()> {
        let name = format!("M{}", self.cnt);
        let mut buf = String::new();
        rust::fmt(&mut buf, qtype, qname, &name, msg)?;
        println!("{buf}");
        Ok(())
    }
}
