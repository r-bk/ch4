use anyhow::Result;
use rsdns::constants::Type;
use std::fmt::Write;

const CHUNK_LEN: usize = 12;

#[inline]
fn is_ascii_printable(b: &u8) -> bool {
    (0x20..=0x7E).contains(b)
}

pub fn fmt<W: Write>(
    w: &mut W,
    qtype: Option<Type>,
    qname: Option<&str>,
    name: &str,
    msg: &[u8],
) -> Result<()> {
    if qtype.is_some() && qname.is_some() {
        writeln!(w, "// {} {}", qtype.unwrap(), qname.unwrap())?;
    }

    writeln!(w, "const {}: [u8; {}] = [", name, msg.len())?;

    let chunks = msg.chunks(CHUNK_LEN);
    let mut max_chunk_len = 0;

    for (index, chunk) in chunks.enumerate() {
        let len = chunk.len();
        max_chunk_len = max_chunk_len.max(len);

        //
        // chunk bytes as hex literals
        //
        for (i, b) in chunk.iter().enumerate() {
            let pfx = if i == 0 { "    " } else { " " };
            write!(w, "{}{:#04x?},", pfx, b)?;
        }

        //
        // fill the last, possibly shorter, line
        //
        if len < max_chunk_len {
            write!(w, " /*")?;
            let n_spaces = (max_chunk_len - len) * 6 - 4 - 1;
            for _ in 0..n_spaces {
                write!(w, " ")?;
            }
            write!(w, "*/")?;
        }

        //
        // visual comment
        //
        write!(w, " // |")?;
        for b in chunk {
            let c = if is_ascii_printable(b) {
                std::str::from_utf8(std::slice::from_ref(b)).unwrap()
            } else {
                "."
            };
            write!(w, "{}", c)?;
        }

        writeln!(w, "| {}", (index * CHUNK_LEN))?;
    }

    writeln!(w, "];")?;
    Ok(())
}
