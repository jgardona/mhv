use anyhow::{Ok, Result};

#[test]
fn read_all_data() -> Result<()> {
    let expected = "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 \n\
                          [0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 \n\
                          [0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 ";

    let mut cmd = assert_cmd::Command::cargo_bin("hd")?;
    cmd.arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn skip_first_offset() -> Result<()> {
    let expected = "[0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 \n\
                          [0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 ";

    let mut cmd = assert_cmd::Command::cargo_bin("hd")?;
    cmd.arg("tests/data/data2")
        .arg("-s")
        .arg("1")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn skip_first_last_offset() -> Result<()> {
    let expected = "[0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36  | 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 ";

    let mut cmd = assert_cmd::Command::cargo_bin("hd")?;
    cmd.arg("tests/data/data2")
        .arg("-s")
        .arg("1")
        .arg("-l")
        .arg("1")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}
