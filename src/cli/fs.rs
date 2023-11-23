use std::{
    fs::File,
    io::{Read, Seek},
};

pub fn read_data(skip: usize, length: Option<usize>, path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    file.seek(std::io::SeekFrom::Start(skip as u64))?;

    if let Some(len) = length {
        let mut buffer = vec![0u8; len];
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

    #[test]
    fn test_read_data() {
        let buffer = read_data(4, Some(5), "tests/data/data1").unwrap();
        assert_eq!("quick", String::from_utf8_lossy(&buffer));
    }
}
