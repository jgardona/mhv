use colored::Colorize;
use std::io::{Cursor, Write};

const OFFSET: usize = 16;
static EMPTY_LINE: &str = "*";

pub fn display_data(skip: usize, nosqueezing: bool, buffer: &[u8]) -> std::io::Result<()> {
    let mut position = skip;
    let mut output = Cursor::new(Vec::<u8>::new());
    let mut old_buffer = [0u8; 16];
    let mut is_printing = false;
    buffer.chunks(OFFSET).for_each(|d| {
        if !squeeze(d, &mut old_buffer, &mut output, nosqueezing, &mut is_printing) {
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
                    0x80..=0xfe => {
                        write!(output, "{} ", format!("{:02x}", b).bright_red()).unwrap()
                    }
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
        }
        position += OFFSET;
    });

    output.set_position(0);
    std::io::copy(&mut output, &mut std::io::stdout())?;

    Ok(())
}

fn squeeze<'a, W: Write>(
    new_buffer: &'a [u8],
    old_buffer: &'a mut [u8],
    writer: &mut W,
    squeezing: bool,
    printed: &mut bool,
) -> bool {
    if new_buffer.len() != old_buffer.len() || !squeezing {
        return false;
    }
    if new_buffer == old_buffer {
        if !*printed {
            writeln!(writer, "{}", EMPTY_LINE.bright_black()).expect("cant write to writer");
            *printed = true;
        }
        old_buffer.copy_from_slice(new_buffer);
        return true;
    }

    *printed = false;
    old_buffer.copy_from_slice(new_buffer);

    false
}

#[cfg(test)]
mod test_view {
    use std::io::Cursor;

    use super::squeeze;

    #[test]
    fn test_squeeze() {
        let new_buffer = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ];
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let mut old_buffer = [0u8; 16];
        let mut is_print = false;

        new_buffer.chunks(16).for_each(|d| {
            if squeeze(d, &mut old_buffer, &mut cursor, true, &mut is_print) {
                assert_eq!(d, old_buffer);
            }
        });
        assert_eq!(
            "\u{1b}[90m*\u{1b}[0m\n",
            String::from_utf8_lossy(&cursor.into_inner())
        );
    }

    #[test]
    fn test_squeeze_5_bytes() {
        let new_buffer = vec![1, 1, 1, 1, 4, 5];
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let mut old_buffer = [0u8; 16];
        let mut is_print = false;

        new_buffer.chunks(16).for_each(|d| {
            if squeeze(d, &mut old_buffer, &mut cursor, true, &mut is_print) {
                assert_eq!(d, old_buffer);
            }
        });
        assert_eq!("", String::from_utf8_lossy(&cursor.into_inner()));
    }
}
