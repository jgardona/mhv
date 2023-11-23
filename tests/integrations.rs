use anyhow::{Ok, Result};

#[test]
fn test_read_all_data() -> Result<()> {
    let expected = "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456\n\
                          [0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456\n\
                          [0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_16_bytes() -> Result<()> {
    let expected = "[0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456\n\
                          [0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .arg("-s")
        .arg("16")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_2_bytes() -> Result<()> {
    let expected =
        "[0x00000002] 33 34 35 36 37 38 39 30 31 32 33 34 35 36 31 32 | 3456789012345612\n\
        [0x00000012] 33 34 35 36 37 38 39 30 31 32 33 34 35 36 31 32 | 3456789012345612\n\
        [0x00000022] 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 34567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .arg("-s")
        .arg("2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_16_length_16_bytes() -> Result<()> {
    let expected =
        "[0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .arg("-s")
        .arg("16")
        .arg("-l")
        .arg("16")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_32_bytes() -> Result<()> {
    let expected =
        "[0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .arg("-s")
        .arg("32")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_length_16_bytes() -> Result<()> {
    let expected =
        "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 | 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .arg("-l")
        .arg("16")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}
