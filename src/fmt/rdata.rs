use anyhow::Result;
use rsdns::records::data;
use std::fmt::Write;

macro_rules! obsolete {
    ($RR:ident) => {
        // https://en.wikipedia.org/wiki/List_of_DNS_record_types#Obsolete_record_types
        impl<W: Write> RDataFormatter<W, data::$RR> for RDataFmt {
            fn fmt(w: &mut W, _d: &data::$RR) -> Result<()> {
                write!(w, "OBSOLETE RTYPE")?;
                Ok(())
            }
        }
    };
}

fn str_from_text(text: &[u8]) -> &str {
    match std::str::from_utf8(text) {
        Ok(s) => s,
        Err(_) => "__CH4_NON_UTF8_STRING__",
    }
}

pub trait RDataFormatter<W: Write, D: data::RData> {
    fn fmt(w: &mut W, d: &D) -> Result<()>;
}

pub struct RDataFmt;

impl<W: Write> RDataFormatter<W, data::A> for RDataFmt {
    fn fmt(w: &mut W, d: &data::A) -> Result<()> {
        write!(w, "{}", d.address)?;
        Ok(())
    }
}

impl<W: Write> RDataFormatter<W, data::Ns> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Ns) -> Result<()> {
        write!(w, "{}", d.nsdname)?;
        Ok(())
    }
}

obsolete!(Md);
obsolete!(Mf);

impl<W: Write> RDataFormatter<W, data::Cname> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Cname) -> Result<()> {
        write!(w, "{}", d.cname)?;
        Ok(())
    }
}

impl<W: Write> RDataFormatter<W, data::Soa> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Soa) -> Result<()> {
        write!(
            w,
            "{} {} {} {} {} {} {}",
            d.mname, d.rname, d.serial, d.refresh, d.retry, d.expire, d.minimum,
        )?;
        Ok(())
    }
}

obsolete!(Mb);
obsolete!(Mg);
obsolete!(Mr);
obsolete!(Null);
obsolete!(Wks);

impl<W: Write> RDataFormatter<W, data::Ptr> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Ptr) -> Result<()> {
        write!(w, "{}", d.ptrdname)?;
        Ok(())
    }
}

impl<W: Write> RDataFormatter<W, data::Hinfo> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Hinfo) -> Result<()> {
        write!(
            w,
            "\"{}\" \"{}\"",
            str_from_text(&d.cpu),
            str_from_text(&d.os)
        )?;
        Ok(())
    }
}

obsolete!(Minfo);

impl<W: Write> RDataFormatter<W, data::Mx> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Mx) -> Result<()> {
        write!(w, "{} {}", d.preference, d.exchange)?;
        Ok(())
    }
}

impl<W: Write> RDataFormatter<W, data::Txt> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Txt) -> Result<()> {
        let utf8 = str_from_text(&d.text);
        write!(w, "\"{utf8}\"")?;
        Ok(())
    }
}

impl<W: Write> RDataFormatter<W, data::Aaaa> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Aaaa) -> Result<()> {
        write!(w, "{}", d.address)?;
        Ok(())
    }
}
