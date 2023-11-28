use anyhow::{Ok, Result};

#[test]
fn test_run_success() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n").arg("tests/data/data5").assert().success();
    Ok(())
}

#[test]
fn test_no_squeeze_initial_2_equals_offset() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n").arg("tests/data/data6").assert().success();
    let result = cmd.output()?.stdout;
    let result = strip_ansi_escapes::strip(&result);
    let result = std::str::from_utf8(&result)?;
    let expected = "00000000 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 0000000000000000\n\
    00000010 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 0000000000000000\n";
    assert_eq!(expected, result);

    Ok(())
}

#[test]
fn test_no_squeeze_meedle_2_equals_offset() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n").arg("tests/data/data4").assert().success();
    let result = cmd.output()?.stdout;
    let result = strip_ansi_escapes::strip(&result);
    let result = std::str::from_utf8(&result)?;
    let expected = "00000000 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n\
    00000010 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 0000000000000000\n\
    00000020 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 0000000000000000\n\
    00000030 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n";
    assert_eq!(expected, result);

    Ok(())
}

#[test]
fn test_no_squeeze_skip_4_length_5b() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-s4")
        .arg("-l5")
        .arg("tests/data/data1")
        .assert()
        .success();
    let result = cmd.output()?.stdout;
    let result = strip_ansi_escapes::strip(&result);
    let result = std::str::from_utf8(&result)?;
    let expected = "00000004 71 75 69 63 6b                                  quick\n";
    assert_eq!(expected, result);

    Ok(())
}

// just to test the clap command parser
#[test]
fn test_no_squeeze_parse_1kb_() -> Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n")
        .arg("-l1kb")
        .arg("tests/data/data1kb")
        .assert()
        .success();
    Ok(())
}

#[test]
fn test_file_not_found() -> Result<()> {
    let expected = "Something wrong happened: No such file or directory (os error 2)\n";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/dat")
        .assert()
        .stderr(predicates::str::contains(expected))
        .failure();
    Ok(())
}

#[test]
fn test_buffer_overflow() -> Result<()> {
    let expected = "Something wrong happened: failed to fill whole buffer\n";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-l50")
        .arg("tests/data/data3")
        .assert()
        .stderr(predicates::str::contains(expected))
        .failure();

    Ok(())
}
