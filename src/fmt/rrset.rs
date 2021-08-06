use crate::fmt::rdata::{RDataFmt, RDataFormatter};
use anyhow::Result;
use rsdns::records::{data::RData, RecordSet};
use std::fmt::Write;

pub fn fmt<W, D>(w: &mut W, rrset: &RecordSet<D>) -> Result<()>
where
    W: Write,
    D: RData,
    RDataFmt: RDataFormatter<W, D>,
{
    writeln!(w, "RRName: {}", rrset.name)?;
    writeln!(w, "RClass: {}", rrset.rclass)?;
    writeln!(w, "RType: {}", D::RTYPE)?;
    writeln!(w, "TTL: {}", rrset.ttl)?;

    for (i, d) in rrset.rdata.iter().enumerate() {
        write!(w, "{:>3}. ", i + 1)?;
        <RDataFmt as RDataFormatter<W, D>>::fmt(w, d)?;
        writeln!(w)?;
    }
    Ok(())
}
