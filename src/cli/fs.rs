use std::{fs::File, io::Read};

pub fn read_data(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod fs_tests {
    use super::read_data;

    #[test]
    fn test_read_data() {
        let buffer = read_data("tests/data/data1").unwrap();
        assert_eq!(
            "the quick brown fox jumps over the lazy dog",
            String::from_utf8_lossy(&buffer)
        );
    }
}
