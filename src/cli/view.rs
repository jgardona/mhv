use colored::Colorize;

const OFFSET: usize = 16;

pub fn display_data(skip: usize, buffer: &[u8]) {
    let mut position = skip;
    buffer.chunks(OFFSET).for_each(|d| {
        print!("{}", format!("[0x{:08x}] ", position).bright_black());
        for b in d {
            match *b {
                // null byte
                0x00 | 0xff => print!("{}  ", "•".bright_black()),
                // ascii printable characters
                0x21..=0x7e => print!("{} ", format!("{:02x}", b).blue()),
                // ascii white space characters and controls
                0x01..=0x08 | 0x0e..=0x1f => print!("{} ", format!("{:02x}", b).green()),
                0x09..=0x0d | 0x20 | 0x7f => print!("{} ", format!("{:02x}", b).green()),
                // ascii extended codes
                0x80..=0xfe => print!("{} ", format!("{:02x}", b).bright_red()),
            }
        }

        print!("| ");

        for b in d {
            match *b {
                0x00 | 0xff => print!("{}", "•".bright_black()),
                0x21..=0x7e => print!("{}", format!("{}", *b as char).blue()),
                0x09..=0x0d | 0x20 | 0x7f => print!("{}", "_".green()),
                0x01..=0x08 | 0x0e..=0x1f => print!("{}", "•".green()),
                0x80..=0xfe => print!("{}", "•".bright_red()),
            }
        }
        println!();
        position += OFFSET;
    })
}
