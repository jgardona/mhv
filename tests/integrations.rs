use anyhow::{Ok, Result};

#[test]
fn test_read_all_data() -> Result<()> {
    let expected = "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n\
                          [0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n\
                          [0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .arg("-n")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_16_bytes() -> Result<()> {
    let expected = "[0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n\
                          [0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n")
        .arg("-s")
        .arg("16")
        .arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_2_bytes() -> Result<()> {
    let expected =
        "[0x00000002] 33 34 35 36 37 38 39 30 31 32 33 34 35 36 31 32 3456789012345612\n\
        [0x00000012] 33 34 35 36 37 38 39 30 31 32 33 34 35 36 31 32 3456789012345612\n\
        [0x00000022] 33 34 35 36 37 38 39 30 31 32 33 34 35 36 34567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n")
        .arg("-s")
        .arg("2")
        .arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_16_length_16_bytes() -> Result<()> {
    let expected = "[0x00000010] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n")
        .arg("-s")
        .arg("16")
        .arg("-l")
        .arg("16")
        .arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_skip_32_bytes() -> Result<()> {
    let expected = "[0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n")
        .arg("-s")
        .arg("32")
        .arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_length_16_bytes() -> Result<()> {
    let expected = "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-n")
        .arg("-l")
        .arg("16")
        .arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_squeezing_full() -> Result<()> {
    let expected =
        "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n*\n";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data2")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_squeezing_with_middle_diff() -> Result<()> {
    let expected =
        "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n\
        [0x00000010] 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 0000000000000000\n\
        [0x00000020] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data3")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_squeezing_with_middle_equality() -> Result<()> {
    let expected =
        "[0x00000000] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456\n\
        [0x00000010] 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 30 0000000000000000\n\
        *\n\
        [0x00000030] 31 32 33 34 35 36 37 38 39 30 31 32 33 34 35 36 1234567890123456";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data4")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_squeezing_no_equality_skip_16_length_4() -> Result<()> {
    let expected = "[0x00000010] 30 30 30 30 0000";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("-s16")
        .arg("-l4")
        .arg("tests/data/data4")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}

#[test]
fn test_read_500_b_urandom() -> Result<()> {
    let expected =
        "[0x00000000] e4 •  aa 20 9c a6 af 9c 0f ec ea 14 29 e6 e5 0f •••_••••••••)•••\n\
        [0x00000010] be a6 73 b2 ee 71 0e b9 47 29 fb 39 f2 cf 0b 7c ••s••q••G)•9••_|\n\
        [0x00000020] ca 40 1b 90 bf 54 96 73 7a 12 76 d6 e3 56 cd 0f •@•••T•sz•v••V••\n\
        [0x00000030] 90 3e 1e 51 39 ab 93 1a 74 76 42 67 da 6d 9e 2f •>•Q9•••tvBg•m•/\n\
        [0x00000040] 44 a8 8f 59 06 44 51 7f b1 01 27 99 3a c2 75 17 D••Y•DQ_••\'•:•u•\n\
        [0x00000050] d3 85 7e b1 04 8d cb 78 e0 fd 65 db 5b 64 25 16 ••~••••x••e•[d%•\n\
        [0x00000060] 64 6e 55 36 81 1e 41 a4 b6 c6 41 97 61 52 2d 11 dnU6••A•••A•aR-•\n\
        [0x00000070] 9d a3 a6 9a ca 9d a7 2f bd 1d b9 80 b7 7f de 06 •••••••/•••••_••\n\
        [0x00000080] 0e b9 f9 b0 a5 ea 49 c7 d8 7d 68 af b7 4b 4e 15 ••••••I••}h••KN•\n\
        [0x00000090] 50 c8 24 0a b0 90 c1 c3 22 61 1b 04 3e 08 78 ed P•$_••••\"a••>•x•\n\
        [0x000000a0] 9c e3 56 b0 cf b3 1c ac 9e c7 0a 94 75 93 0b 38 ••V•••••••_•u•_8\n\
        [0x000000b0] 73 25 a4 42 ba a4 92 b7 4e eb f8 bf d3 60 8b 2a s%•B••••N••••`•*\n\
        [0x000000c0] a4 a6 7f 7a ad d9 1a 25 51 32 6d aa 50 75 54 d0 ••_z•••%Q2m•PuT•\n\
        [0x000000d0] 21 d5 59 34 c1 78 6e 92 f9 0e 77 3a 40 6c 98 b9 !•Y4•xn•••w:@l••\n\
        [0x000000e0] 33 84 91 4c af 26 a1 09 b9 80 d0 bb 51 e8 3d 9c 3••L•&•_••••Q•=•\n\
        [0x000000f0] 1b b0 05 e5 c4 f0 43 30 66 2d 55 c6 85 dc 08 a8 ••••••C0f-U•••••\n\
        [0x00000100] a0 ca ed 25 c6 5a d7 54 86 29 e0 34 2f fc 74 2e •••%•Z•T•)•4/•t.\n\
        [0x00000110] 92 df 69 16 98 •  a1 22 eb 8f ae 20 02 64 83 ce ••i••••\"•••_•d••\n\
        [0x00000120] 66 6e 44 83 e9 a5 7c 76 08 92 ae 05 8a 6b b8 ce fnD•••|v•••••k••\n\
        [0x00000130] 1b 72 a3 16 7f 31 35 43 be d6 cb 1b a7 2f 69 9c •r••_15C•••••/i•\n\
        [0x00000140] •  ed ab a9 0d d2 01 b3 bd 1c dd b9 74 21 9b 66 ••••_•••••••t!•f\n\
        [0x00000150] c5 fa f1 0a 01 37 37 da a5 e2 8d 86 47 07 09 20 •••_•77•••••G•__\n\
        [0x00000160] f9 4b dc 1e e3 8c d6 44 1b 05 0f 84 ce 05 84 f0 •K•••••D••••••••\n\
        [0x00000170] 8b d7 97 f1 cb 73 34 79 40 98 9f 67 3b a5 fa cf •••••s4y@••g;•••\n\
        [0x00000180] cc d1 23 9a fd 2a b6 99 ed 7f 26 95 95 78 bd 49 ••#••*•••_&••x•I\n\
        [0x00000190] 8a 0c ac 3c c6 59 c8 f2 21 f1 81 a3 6a 87 dd 99 •_•<•Y••!•••j•••\n\
        [0x000001a0] 55 f4 ec d1 41 b8 f6 9e 6d 39 c3 d7 04 47 b7 a3 U•••A•••m9•••G••\n\
        [0x000001b0] db 7d 81 0b 08 c6 ea fd 45 2f 5f b7 fc d3 92 20 •}•_••••E/_••••_\n\
        [0x000001c0] 63 80 8d 4d 10 ba d4 e5 ae 05 74 ae 9f 3f 4b 4e c••M••••••t••?KN\n\
        [0x000001d0] 0d 42 89 f3 44 ad 8f 1c c2 6b 3b •  41 eb 26 c0 _B••D••••k;•A•&•\n\
        [0x000001e0] af 5f 35 30 ed 1d 35 de 37 ae b8 fe 2d ed 75 56 •_50••5•7•••-•uV\n\
        [0x000001f0] 29 b8 f5 de )•••";

    let mut cmd = assert_cmd::Command::cargo_bin("mhv")?;
    cmd.arg("tests/data/data5")
        .assert()
        .stdout(predicates::str::contains(expected))
        .success();

    Ok(())
}
