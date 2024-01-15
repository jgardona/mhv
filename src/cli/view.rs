use ansistream::*;
use std::io::Write;

const OFFSET: usize = 16;
static EMPTY_LINE: &str = "*";

pub fn display_data<W: Write>(
    skip: usize,
    nosqueezing: bool,
    buffer: &[u8],
    writer: &mut W,
) -> std::io::Result<()> {
    let mut position = skip;
    let mut output = AnsiEscapeStream::default();
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
            output
                .write_text_fc_fmt(FC_DARK_GRAY, format_args!("{:08x} ", position))
                .unwrap();
            for b in d {
                match *b {
                    // null byte
                    0x00 => output
                        .write_text_fc_fmt(FC_DARK_GRAY, format_args!("00 "))
                        .unwrap(),
                    0xff => output
                        .write_text_fc_fmt(FC_LIGHT_RED, format_args!("FF "))
                        .unwrap(),
                    // ascii printable characters
                    0x21..=0x7e => output
                        .write_text_fc_fmt(FC_BLUE, format_args!("{:02x} ", b))
                        .unwrap(),
                    // ascii white space characters and controls
                    0x01..=0x08 | 0x0e..=0x1f => output
                        .write_text_fc_fmt(FC_GREEN, format_args!("{:02x} ", b))
                        .unwrap(),
                    0x09..=0x0d | 0x20 | 0x7f => output
                        .write_text_fc_fmt(FC_GREEN, format_args!("{:02x} ", b))
                        .unwrap(),
                    // ascii extended codes
                    0x80..=0xfe => output
                        .write_text_fc_fmt(FC_LIGHT_RED, format_args!("{:02x} ", b))
                        .unwrap(),
                }
            }

            if OFFSET - d.len() > 0 {
                write!(output, "{}", " ".repeat((OFFSET - d.len()) * 3)).unwrap();
            }

            for b in d {
                match *b {
                    0x00 => output
                        .write_text_fc_fmt(FC_DARK_GRAY, format_args!("◦"))
                        .unwrap(),
                    0xff => output
                        .write_text_fc_fmt(FC_LIGHT_RED, format_args!("×"))
                        .unwrap(),
                    0x21..=0x7e => output
                        .write_text_fc_fmt(FC_BLUE, format_args!("{}", *b as char))
                        .unwrap(),
                    0x09..=0x0d | 0x20 | 0x7f => output
                        .write_text_fc_fmt(FC_GREEN, format_args!("_"))
                        .unwrap(),
                    0x01..=0x08 | 0x0e..=0x1f => output
                        .write_text_fc_fmt(FC_GREEN, format_args!("•"))
                        .unwrap(),
                    0x80..=0xfe => output
                        .write_text_fc_fmt(FC_LIGHT_RED, format_args!("×"))
                        .unwrap(),
                }
            }
            writeln!(output).unwrap();
        }
        position += OFFSET;
    });

    output.set_position(0);
    std::io::copy(&mut *output, writer).unwrap();

    Ok(())
}

fn squeeze<'a>(
    new_buffer: &'a [u8],
    old_buffer: &'a mut [u8],
    writer: &mut AnsiEscapeStream,
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
            //writeln!(writer).expect("cant write to writer");
            writer
                .write_text_fc_fmt(FC_DARK_GRAY, format_args!("{EMPTY_LINE}\n"))
                .expect("cant write to writer");
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
    use ansistream::AnsiEscapeStream;
    use anyhow::{Ok, Result};

    const OFFSET: RangeInclusive<usize> = RangeInclusive::new(0, 18);
    const FIRST_CHAR: RangeInclusive<usize> = RangeInclusive::new(19, 31);

    #[test]
    fn test_squeeze() {
        let new_buffer = vec![0u8; 32];
        let mut cursor = AnsiEscapeStream::default();
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
        let striped_buffer = strip_ansi_escapes::strip(cursor.get_ref().to_vec());
        assert_eq!("*\n", String::from_utf8_lossy(&striped_buffer));
    }

    #[test]
    fn test_squeeze_5_bytes() {
        let new_buffer = vec![1, 1, 1, 1, 4, 5];
        let mut cursor = AnsiEscapeStream::default();
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
        assert_eq!("", String::from_utf8_lossy(&cursor.get_ref().to_vec()));
    }

    #[test]
    fn test_display_data_16_null_bytes() -> Result<()> {
        let input = [0u8; 16];
        let expected =
            "00000000 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦\n";
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
            27, 91, 57, 48, 109, 48, 48, 48, 48, 48, 48, 48, 48, 32, 27, 91, 51, 57, 109,
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
        let expected = [27, 91, 51, 50, 109, 48, 53, 32, 27, 91, 51, 57, 109];
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
        let expected = [27, 91, 51, 50, 109, 50, 48, 32, 27, 91, 51, 57, 109];
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
        let expected = [27, 91, 57, 49, 109, 56, 48, 32, 27, 91, 51, 57, 109];
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
            "00000000 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦\n\
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
            00000010 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦◦\n\
            *\n\
            00000030 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 ••••••••••••••••\n";

        assert_eq!(expected, result);

        Ok(())
    }
}
