use crate::fmt::rdata::{RDataFmt, RDataFormatter};
use anyhow::Result;
use rsdns::records::{data::RData, RecordSet};
use std::fmt::Write;

pub fn fmt_short<W, D>(w: &mut W, rrset: &RecordSet<D>) -> Result<()>
where
    W: Write,
    D: RData,
    RDataFmt: RDataFormatter<W, D>,
{
    for d in rrset.rdata.iter() {
        <RDataFmt as RDataFormatter<W, D>>::fmt(w, d)?;
        writeln!(w)?;
    }
    Ok(())
}
