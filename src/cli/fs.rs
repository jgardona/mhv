use std::{
    fs::File,
    io::{Read, Seek},
};

pub fn read_data(skip: usize, length: usize, path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    file.seek(std::io::SeekFrom::Start(skip as u64))?;

    if length != 0 {
        let mut buffer = vec![0u8; length];
        file.read_exact(&mut buffer)?;
        Ok(buffer)
    } else {
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod fs_tests {
    use super::read_data;
    use anyhow::Result;

    #[test]
    fn test_skip_4_length_5_read_data() -> Result<()> {
        let buffer = read_data(4, 5, "tests/data/data1")?;
        assert_eq!("quick", String::from_utf8_lossy(&buffer));
        Ok(())
    }

    #[test]
    fn test_read_full_data() -> Result<()> {
        let buffer = read_data(0, 0, "tests/data/data1")?;
        assert_eq!(
            "the quick brown fox jumps over the lazy dog",
            String::from_utf8_lossy(&buffer)
        );
        Ok(())
    }
}
