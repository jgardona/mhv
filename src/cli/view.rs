const OFFSET: usize = 16;

pub fn display_data(skip: usize, length: usize, buffer: &[u8]) {
    let mut position = skip * OFFSET;
    buffer.chunks(OFFSET).skip(skip).take(length).for_each(|d| {
        print!("[0x{:08x}] ", position);
        for b in d {
            match *b {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", b),
            }
        }

        print!(" | ");

        for b in d {
            match *b {
                0x00..=0x20 | 0x7f..=0xff => print!(". "),
                _ => print!("{} ", *b as char),
            }
        }
        println!();
        position += OFFSET;
    })
}

#[cfg(test)]
mod view_test {
    use super::display_data;

    #[test]
    fn test_display_data() {
        let skip = 0;
        let length = 10;
        let data = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        display_data(skip, length, &data);
    }
}
