use owo_colors::*;
use std::io::{Cursor, Write};

const OFFSET: usize = 16;
static EMPTY_LINE: &str = "*";

pub fn display_data<W: Write>(
    skip: usize,
    nosqueezing: bool,
    buffer: &[u8],
    writer: &mut W,
) -> std::io::Result<()> {
    let mut position = skip;
    let mut output = Cursor::new(Vec::<u8>::new());
    let mut old_buffer = [0u8; 16];
    let mut is_printing = false;
    let mut first_line = true;
    buffer.chunks(OFFSET).for_each(|d| {
        if !squeeze(
            d,
            &mut old_buffer,
            &mut output,
            nosqueezing,
            &mut is_printing,
            &mut first_line,
        ) {
            write!(output, "{:08x} ", position.bright_black()).unwrap();
            for b in d {
                match *b {
                    // null byte
                    0x00 => write!(output, "{} ", "··".bright_black()).unwrap(),
                    0xff => write!(output, "{} ", "••".bright_black()).unwrap(),
                    // ascii printable characters
                    0x21..=0x7e => write!(output, "{:02x} ", b.blue()).unwrap(),
                    // ascii white space characters and controls
                    0x01..=0x08 | 0x0e..=0x1f => write!(output, "{:02x} ", b.green()).unwrap(),
                    0x09..=0x0d | 0x20 | 0x7f => write!(output, "{:02x} ", b.green()).unwrap(),
                    // ascii extended codes
                    0x80..=0xfe => write!(output, "{:02x} ", b.bright_red()).unwrap(),
                }
            }

            for b in d {
                match *b {
                    0x00 => write!(output, "{}", "·".bright_black()).unwrap(),
                    0xff => write!(output, "{}", "•".bright_black()).unwrap(),
                    0x21..=0x7e => write!(output, "{}", (*b as char).blue()).unwrap(),
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
    std::io::copy(&mut output, writer)?;

    Ok(())
}

fn squeeze<'a, W: Write>(
    new_buffer: &'a [u8],
    old_buffer: &'a mut [u8],
    writer: &mut W,
    squeezing: bool,
    printed: &mut bool,
    first_line: &mut bool,
) -> bool {
    if new_buffer.len() != old_buffer.len() || !squeezing {
        return false;
    }
    if new_buffer == old_buffer {
        if *first_line {
            *first_line = false;
            return false;
        }
        if !*printed {
            writeln!(writer, "{}", EMPTY_LINE.bright_black()).expect("cant write to writer");
            *printed = true;
        }

        old_buffer.copy_from_slice(new_buffer);
        return true;
    }

    *printed = false;
    *first_line = false;
    old_buffer.copy_from_slice(new_buffer);

    false
}

#[cfg(test)]
mod test_view {
    use std::{io::Cursor, ops::RangeInclusive};

    use super::{display_data, squeeze};
    use anyhow::{Ok, Result};

    const OFFSET: RangeInclusive<usize> = RangeInclusive::new(0, 17);
    const FIRST_CHAR: RangeInclusive<usize> = RangeInclusive::new(19, 30);

    #[test]
    fn test_squeeze() {
        let new_buffer = vec![0u8; 32];
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let mut old_buffer = [0u8; 16];
        let mut is_print = false;
        let mut first_line = true;
        const OFFSET: usize = 16;

        new_buffer.chunks(OFFSET).for_each(|d| {
            if squeeze(
                d,
                &mut old_buffer,
                &mut cursor,
                true,
                &mut is_print,
                &mut first_line,
            ) {
                assert_eq!(d, old_buffer);
            }
        });
        let striped_buffer = strip_ansi_escapes::strip(cursor.into_inner());
        assert_eq!("*\n", String::from_utf8_lossy(&striped_buffer));
    }

    #[test]
    fn test_squeeze_5_bytes() {
        let new_buffer = vec![1, 1, 1, 1, 4, 5];
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let mut old_buffer = [0u8; 16];
        let mut is_print = false;
        let mut first_line = true;

        new_buffer.chunks(16).for_each(|d| {
            if squeeze(
                d,
                &mut old_buffer,
                &mut cursor,
                true,
                &mut is_print,
                &mut first_line,
            ) {
                assert_eq!(d, old_buffer);
            }
        });
        assert_eq!("", String::from_utf8_lossy(&cursor.into_inner()));
    }

    #[test]
    fn test_display_data_16_null_bytes() -> Result<()> {
        let input = [0u8; 16];
        let expected =
            "00000000 •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  ••••••••••••••••\n";
        let mut output = Cursor::new(Vec::<u8>::new());

        display_data(0, false, &input, &mut output)?;

        let result = output.get_ref();
        let plain_result = strip_ansi_escapes::strip(result);
        assert_eq!(expected, std::str::from_utf8(&plain_result)?);

        let result = String::from_utf8(output.into_inner())?;
        println!("{result}");

        Ok(())
    }

    #[test]
    fn test_offset_color_must_be_bright_black() -> Result<()> {
        let new_buffer = [0u8; 16];
        let mut output = Cursor::new(Vec::<u8>::new());
        display_data(0, false, &new_buffer, &mut output)?;
        let result = output.get_ref();
        let offset_data = &result[OFFSET];
        let expect = [
            0x1b, 0x5b, 0x39, 0x30, 0x6d, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x1b,
            0x5b, 0x33, 0x39, 0x6d,
        ];
        // prove output offset section data is eight zeros with ansi code bright black color
        assert_eq!(expect, offset_data);
        Ok(())
    }

    #[test]
    fn test_control_character_must_be_green() -> Result<()> {
        let new_buffer = [0x05; 16];
        let mut output = Cursor::new(Vec::<u8>::new());
        display_data(0, false, &new_buffer, &mut output)?;
        let result = output.get_ref();
        let first_char = &result[FIRST_CHAR];
        // ansi code red 0x05 ascii control
        let expected = [
            0x1b, 0x5b, 0x33, 0x32, 0x6d, 0x30, 0x35, 0x1b, 0x5b, 0x33, 0x39, 0x6d,
        ];
        assert_eq!(expected, first_char);
        Ok(())
    }

    #[test]
    fn test_spaces_character_must_be_green() -> Result<()> {
        let new_buffer = [0x20; 16];
        let mut output = Cursor::new(Vec::<u8>::new());
        display_data(0, false, &new_buffer, &mut output)?;
        let result = output.get_ref();
        let first_char = &result[FIRST_CHAR];
        // ansi code green 0x05 ascii control
        let expected = [
            0x1b, 0x5b, 0x33, 0x32, 0x6d, 0x32, 0x30, 0x1b, 0x5b, 0x33, 0x39, 0x6d,
        ];
        assert_eq!(expected, first_char);
        Ok(())
    }

    #[test]
    fn test_extended_character_must_be_red() -> Result<()> {
        let new_buffer = [0x80; 16];
        let mut output = Cursor::new(Vec::<u8>::new());
        display_data(0, false, &new_buffer, &mut output)?;
        let result = output.get_ref();
        let first_char = &result[FIRST_CHAR];
        // ansi code red 0x05 ascii extended
        let expected = [
            0x1b, 0x5b, 0x39, 0x31, 0x6d, 0x38, 0x30, 0x1b, 0x5b, 0x33, 0x39, 0x6d,
        ];
        assert_eq!(expected, first_char);
        Ok(())
    }

    #[test]
    fn test_display_data_squeezing_line_restore() -> Result<()> {
        let mut new_buffer = vec![0u8; 32];
        new_buffer.extend(vec![1u8; 16]);
        let mut output = Cursor::new(Vec::<u8>::new());
        display_data(0, true, &new_buffer, &mut output)?;
        let result = output.get_ref();
        let plain_result = strip_ansi_escapes::strip(result);

        let result = std::str::from_utf8(&plain_result)?;

        let expected =
            "00000000 •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  ••••••••••••••••\n\
            *\n\
            00000020 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 ••••••••••••••••\n";

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn test_display_data_squeezing_after_first_line_restore() -> Result<()> {
        let mut new_buffer = vec![1u8; 16];
        new_buffer.extend(vec![0u8; 32]);
        new_buffer.extend(vec![1u8; 16]);
        let mut output = Cursor::new(Vec::<u8>::new());
        display_data(0, true, &new_buffer, &mut output)?;
        let result = output.get_ref();
        let plain_result = strip_ansi_escapes::strip(result);

        let result = std::str::from_utf8(&plain_result)?;

        let expected =
            "00000000 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 ••••••••••••••••\n\
            00000010 •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  •  ••••••••••••••••\n\
            *\n\
            00000030 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 ••••••••••••••••\n";

        assert_eq!(expected, result);

        Ok(())
    }
}
