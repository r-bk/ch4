mod rdata;
mod rrset;
mod rust;
mod zone;

//
// ----------------------------------------------------------------------
//

use crate::args::{Args, OutputFormat};
use anyhow::{bail, Result};
use rdata::{RDataFmt, RDataFormatter};
use rsdns::{
    constants::Type,
    records::{
        data::{self, RData},
        RecordSet,
    },
};
use std::time::{Duration, SystemTime};

pub struct Format<'a> {
    args: &'a Args,
    cnt: usize,
}

impl<'a> Format<'a> {
    pub fn new(args: &'a Args) -> Format<'a> {
        Self { args, cnt: 0 }
    }

    pub fn add(
        &mut self,
        qname: &str,
        msg: &[u8],
        ts: SystemTime,
        elapsed: Duration,
    ) -> Result<()> {
        match self.args.format {
            OutputFormat::Short => self.short(msg)?,
            OutputFormat::Zone => self.zone(qname, msg, ts, elapsed)?,
            OutputFormat::Rust => self.rust(qname, msg)?,
        };
        self.cnt += 1;
        Ok(())
    }

    pub fn done(&mut self) -> Result<()> {
        Ok(())
    }

    fn short(&self, msg: &[u8]) -> Result<()> {
        match self.args.qtype() {
            Type::A => Self::short_rrset::<data::A>(msg),
            Type::Ns => Self::short_rrset::<data::Ns>(msg),
            Type::Md => Self::short_rrset::<data::Md>(msg),
            Type::Mf => Self::short_rrset::<data::Mf>(msg),
            Type::Cname => Self::short_rrset::<data::Cname>(msg),
            Type::Soa => Self::short_rrset::<data::Soa>(msg),
            Type::Mb => Self::short_rrset::<data::Mb>(msg),
            Type::Mg => Self::short_rrset::<data::Mg>(msg),
            Type::Mr => Self::short_rrset::<data::Mr>(msg),
            Type::Null => Self::short_rrset::<data::Null>(msg),
            Type::Wks => Self::short_rrset::<data::Wks>(msg),
            Type::Ptr => Self::short_rrset::<data::Ptr>(msg),
            Type::Hinfo => Self::short_rrset::<data::Hinfo>(msg),
            Type::Minfo => Self::short_rrset::<data::Minfo>(msg),
            Type::Mx => Self::short_rrset::<data::Mx>(msg),
            Type::Txt => Self::short_rrset::<data::Txt>(msg),
            Type::Aaaa => Self::short_rrset::<data::Aaaa>(msg),
            Type::Axfr | Type::Mailb | Type::Maila | Type::Any => bail!("invalid qtype"),
        }
    }

    fn short_rrset<D: RData>(msg: &[u8]) -> Result<()>
    where
        RDataFmt: RDataFormatter<String, D>,
    {
        let rr_set = RecordSet::<D>::from_msg(msg)?;
        let mut buf = String::new();
        rrset::fmt_short(&mut buf, &rr_set)?;
        print!("{}", buf);
        Ok(())
    }

    fn zone(&self, qname: &str, msg: &[u8], ts: SystemTime, elapsed: Duration) -> Result<()> {
        if self.cnt > 0 {
            println!();
        }
        zone::Output::new(self.args, qname, msg, ts, elapsed)?.print()
    }

    fn rust(&self, qname: &str, msg: &[u8]) -> Result<()> {
        let name = format!("M{}", self.cnt);
        let mut buf = String::new();
        rust::fmt(&mut buf, self.args.qtype(), qname, &name, msg)?;
        println!("{}", buf);
        Ok(())
    }
}
