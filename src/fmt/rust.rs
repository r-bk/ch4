use anyhow::Result;
use rsdns::constants::Type;
use std::fmt::Write;

const CHUNK_LEN: usize = 12;

#[inline]
fn is_ascii_printable(b: &u8) -> bool {
    (0x20..=0x7E).contains(b)
}

pub fn fmt<W: Write>(w: &mut W, qtype: Type, qname: &str, name: &str, msg: &[u8]) -> Result<()> {
    writeln!(w, "// {} {}", qtype, qname)?;
    writeln!(w, "const {}: [u8; {}] = [", name, msg.len())?;

    let chunks = msg.chunks(CHUNK_LEN);
    let mut max_chunk_len = 0;

    for chunk in chunks {
        let len = chunk.len();
        max_chunk_len = max_chunk_len.max(len);

        write!(w, "    ")?; // indentation

        //
        // chunk bytes as hex literals
        //
        for b in chunk {
            write!(w, "{:#04x?}, ", b)?;
        }
        // fill the last, possibly shorter, line
        for _ in 0..(max_chunk_len - len) {
            write!(w, "      ")?;
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
        // fill the last, possibly shorter, line
        for _ in 0..(max_chunk_len - len) {
            write!(w, " ")?;
        }
        writeln!(w, "|")?;
    }

    writeln!(w, "];")?;
    Ok(())
}
