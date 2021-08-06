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

obsolete!(Hinfo);
obsolete!(Minfo);

impl<W: Write> RDataFormatter<W, data::Mx> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Mx) -> Result<()> {
        write!(w, "{} {}", d.preference, d.exchange)?;
        Ok(())
    }
}

impl<W: Write> RDataFormatter<W, data::Txt> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Txt) -> Result<()> {
        let utf8 = match std::str::from_utf8(&d.text) {
            Ok(s) => s,
            Err(_) => {
                write!(w, "__CH4_NON_UTF8_STRING__")?;
                return Ok(());
            }
        };
        write!(w, "\"{}\"", utf8)?;
        Ok(())
    }
}

impl<W: Write> RDataFormatter<W, data::Aaaa> for RDataFmt {
    fn fmt(w: &mut W, d: &data::Aaaa) -> Result<()> {
        write!(w, "{}", d.address)?;
        Ok(())
    }
}
