use std::io::{Cursor, Write};

use colored::Colorize;

const OFFSET: usize = 16;

pub fn display_data(skip: usize, buffer: &[u8]) -> std::io::Result<()> {
    let mut position = skip;
    let mut output = Cursor::new(Vec::<u8>::with_capacity(buffer.len()));
    buffer.chunks(OFFSET).for_each(|d| {
        write!(
            output,
            "{}",
            format!("[0x{:08x}] ", position).bright_black()
        )
        .unwrap();
        for b in d {
            match *b {
                // null byte
                0x00 | 0xff => write!(output, "{}  ", "•".bright_black()).unwrap(),
                // ascii printable characters
                0x21..=0x7e => write!(output, "{} ", format!("{:02x}", b).blue()).unwrap(),
                // ascii white space characters and controls
                0x01..=0x08 | 0x0e..=0x1f => {
                    write!(output, "{} ", format!("{:02x}", b).green()).unwrap()
                }
                0x09..=0x0d | 0x20 | 0x7f => {
                    write!(output, "{} ", format!("{:02x}", b).green()).unwrap()
                }
                // ascii extended codes
                0x80..=0xfe => write!(output, "{} ", format!("{:02x}", b).bright_red()).unwrap(),
            }
        }

        for b in d {
            match *b {
                0x00 | 0xff => write!(output, "{}", "•".bright_black()).unwrap(),
                0x21..=0x7e => write!(output, "{}", format!("{}", *b as char).blue()).unwrap(),
                0x09..=0x0d | 0x20 | 0x7f => write!(output, "{}", "_".green()).unwrap(),
                0x01..=0x08 | 0x0e..=0x1f => write!(output, "{}", "•".green()).unwrap(),
                0x80..=0xfe => write!(output, "{}", "•".bright_red()).unwrap(),
            }
        }
        writeln!(output).unwrap();
        position += OFFSET;
    });

    output.set_position(0);
    std::io::copy(&mut output, &mut std::io::stdout())?;

    Ok(())
}
